# Server TuDu
---
## 🚀 Technologies Used

- **Rust** (Backend)
- **Axum** (Web framework)
- **Diesel** (ORM for Rust)
- **MySQL** (Database)
- **Docker & Docker Compose** (Containerization and deployment)
- **Serde** (Serialization and deserialization)
- **Tokio** (Asynchronous runtime)

---

## 🐳 Running the Project with Docker

### Prerequisites

- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

### Steps

1. **Clone the repository:**
   ```bash
   git clone https://github.com/arkeasz/tudu.git
   cd server
   ```

2. **Create a `.env` file:**
   ```env
   MYSQL_PASSWORD=yourpassword
   MYSQL_DATABASE=yourdatabase
   MYSQL_USER=youruser
   DATABASE_URL_DOCKER=mysql://youruser:yourpassword@db:3306/yourdatabase
   ```

3. **Run the containers:**
   ```bash
   docker-compose up --build
   ```

4. **Access the application:**
   Open your browser and go to:
   ```
   http://localhost:8080
   ```

---

## 🌐 Available Endpoints

### `GET /`
- **Description:** Basic root endpoint.

### `GET /posts`
- **Description:** Retrieves a list of the latest 5 published posts.
- **Response:**
  ```json
  [
    {
      "id": 1,
      "title": "Post Title",
      "body": "Post Content",
      "published": true
    }
  ]
  ```

### `GET /posts/{id}`
- **Description:** Retrieves a single post by its ID.
- **Response (Success):**
  ```json
  {
    "id": 1,
    "title": "Post Title",
    "body": "Post Content"
  }
  ```
- **Response (Not Found):**
  ```json
  {
    "message": "Unable to find post {id}"
  }
  ```

---

## ⚡ Project Structure

```
server/
├── src/
│   ├── handlers.rs      # API endpoint handlers
│   ├── lib.rs           # Module declarations and DB connection
│   ├── main.rs          # Application entry point
│   ├── models.rs        # Data models for Diesel
│   └── schema.rs        # Diesel schema
├── Dockerfile           # Docker configuration
├── docker-compose.yml   # Docker Compose configuration
├── .env.example         # Environment variable template
├── Cargo.toml           # Rust project configuration
└── wait-for-db.sh       # Script to ensure DB is ready before starting the app
```

---

## 📦 Deployment

With Docker support, you can deploy this server to any cloud provider that supports Docker, such as:

- **AWS (Elastic Beanstalk / ECS)**
- **Azure App Service**
- **Google Cloud Run**
- **DigitalOcean App Platform**
- **Heroku (with Docker support)**

Ensure that your `.env` file has the correct production environment variables for the deployment environment.
