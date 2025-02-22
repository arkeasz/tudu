
FROM rust:latest as builder

RUN apt update && apt install -y --no-install-recommends \
    wget \
    lsb-release \
    pkg-config \
    ca-certificates \
    libmariadb-dev \
    gnupg \
    default-mysql-client \
    && wget --no-check-certificate https://dev.mysql.com/get/mysql-apt-config_0.8.33-1_all.deb \
    && DEBIAN_FRONTEND=noninteractive dpkg -i mysql-apt-config_0.8.33-1_all.deb \
    && apt-get update


WORKDIR /app

# ---------------------- dotenv -------------
ARG MYSQL_USER
ARG MYSQL_PASSWORD
ARG MYSQL_DATABASE
ARG DATABASE_URL

ENV MYSQL_USER=${MYSQL_USER}
ENV MYSQL_PASSWORD=${MYSQL_PASSWORD}
ENV MYSQL_DATABASE=${MYSQL_DATABASE}
ENV DATABASE_URL=${DATABASE_URL_DOCKER}
# -------------------!! dotenv !! ---------------------

COPY . .
RUN cargo install diesel_cli --no-default-features --features mysql
RUN cargo build --release

# ---------------------- Second stage ----------------------
FROM debian:bookworm

RUN apt update && apt install -y --no-install-recommends \
    wget \
    lsb-release \
    pkg-config \
    ca-certificates \
    gnupg \
    libmariadb-dev \
    default-mysql-client \
    && wget --no-check-certificate https://dev.mysql.com/get/mysql-apt-config_0.8.33-1_all.deb \
    && DEBIAN_FRONTEND=noninteractive dpkg -i mysql-apt-config_0.8.33-1_all.deb \
    && apt-get update

WORKDIR /app


COPY --from=builder /app/target/release/server /usr/local/bin/server
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY migrations ./migrations

COPY wait-for-db.sh /usr/local/bin/wait-for-db.sh
RUN chmod +x /usr/local/bin/wait-for-db.sh

EXPOSE 8080
ENTRYPOINT ["wait-for-db.sh", "db"]
CMD ["sh", "-c", "diesel migration run && server"]
