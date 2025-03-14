// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Bigint,
        task_id -> Nullable<Bigint>,
        document_id -> Nullable<Bigint>,
        file_path -> Text,
        uploaded_by -> Bigint,
        uploaded_at -> Timestamp,
    }
}

diesel::table! {
    calendar_events (id) {
        id -> Bigint,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        project_id -> Nullable<Bigint>,
        created_by -> Bigint,
        created_at -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Bigint,
        task_id -> Bigint,
        user_id -> Bigint,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    document_comments (id) {
        id -> Bigint,
        document_id -> Bigint,
        user_id -> Bigint,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    document_history (id) {
        id -> Bigint,
        document_id -> Bigint,
        content -> Text,
        changed_at -> Timestamp,
        changed_by -> Bigint,
    }
}

diesel::table! {
    document_tags (document_id, tag_id) {
        document_id -> Bigint,
        tag_id -> Bigint,
    }
}

diesel::table! {
    documents (id) {
        id -> Bigint,
        #[max_length = 255]
        title -> Varchar,
        content -> Nullable<Text>,
        project_id -> Nullable<Bigint>,
        team_id -> Nullable<Bigint>,
        created_by -> Bigint,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    notifications (id) {
        id -> Bigint,
        user_id -> Bigint,
        content -> Text,
        is_read -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    projects (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        owner_id -> Bigint,
        created_at -> Timestamp,
        team_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    roles (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    tags (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Bigint,
        tag_id -> Bigint,
    }
}

diesel::table! {
    tasks (id) {
        id -> Bigint,
        project_id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 20]
        status -> Varchar,
        assigned_to -> Nullable<Bigint>,
        due_date -> Nullable<Date>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    team_members (id) {
        id -> Bigint,
        team_id -> Bigint,
        user_id -> Bigint,
        #[max_length = 20]
        role -> Varchar,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Bigint,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Bigint,
        role_id -> Bigint,
    }
}

diesel::table! {
    users (id) {
        id -> Bigint,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password_hash -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(attachments -> documents (document_id));
diesel::joinable!(attachments -> tasks (task_id));
diesel::joinable!(attachments -> users (uploaded_by));
diesel::joinable!(calendar_events -> projects (project_id));
diesel::joinable!(calendar_events -> users (created_by));
diesel::joinable!(comments -> tasks (task_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(document_comments -> documents (document_id));
diesel::joinable!(document_comments -> users (user_id));
diesel::joinable!(document_history -> documents (document_id));
diesel::joinable!(document_history -> users (changed_by));
diesel::joinable!(document_tags -> documents (document_id));
diesel::joinable!(document_tags -> tags (tag_id));
diesel::joinable!(documents -> projects (project_id));
diesel::joinable!(documents -> teams (team_id));
diesel::joinable!(documents -> users (created_by));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(projects -> teams (team_id));
diesel::joinable!(projects -> users (owner_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));
diesel::joinable!(tasks -> projects (project_id));
diesel::joinable!(tasks -> users (assigned_to));
diesel::joinable!(team_members -> teams (team_id));
diesel::joinable!(team_members -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    attachments,
    calendar_events,
    comments,
    document_comments,
    document_history,
    document_tags,
    documents,
    notifications,
    posts,
    projects,
    roles,
    tags,
    task_tags,
    tasks,
    team_members,
    teams,
    user_roles,
    users,
);
