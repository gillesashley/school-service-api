# School Management Service API

**⚠️ Work in Progress: This project is currently under active development. Features are incomplete and subject to change. ⚠️**

## Overview

This project is a RESTful API for a school management system, built with Rust and the Actix-web framework. It aims to provide a backend service for managing courses, teachers, students, and more. The goal is to build a robust, scalable, and modern web service to showcase best practices in Rust development for web backends.

## Technology Stack

- **Framework**: [Actix-web](https://actix.rs/)
- **Database**: [PostgreSQL](https://www.postgresql.org/)
- **Database Interaction**: [SQLx](https://github.com/launchbadge/sqlx)
- **Serialization**: [Serde](https://serde.rs/)
- **Authentication**: JWT (JSON Web Tokens)

## Prerequisites

Before you begin, ensure you have the following installed:
- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)

## Getting Started

### 1. Clone the Repository
```bash
git clone <your-repository-url>
cd school-service-api
```

### 2. Configure Environment Variables
Create a `.env` file in the root of the project by copying the example:
```
DATABASE_URL=postgres://postgres:YourPassword@localhost:5432/school_db
JWT_SECRET=your-super-secret-and-long-jwt-key
```
- **DATABASE_URL**: Your PostgreSQL connection string. Remember to [URL-encode](https://www.urlencoder.org/) special characters in your password (e.g., `@` becomes `%40`).
- **JWT_SECRET**: A long, random, and secret string used for signing authentication tokens.

### 3. Set Up the Database
1.  Ensure your PostgreSQL server is running.
2.  Create a new database named `school_db`.
3.  Execute the `database.sql` script to create the necessary tables and populate them with sample data. You can do this using a tool like `psql`:
    ```bash
    psql -U your_postgres_user -d school_db -f database.sql
    ```

## How to Run the Application

### Check the Build
You can check if the project compiles correctly without running it:
```bash
cargo check
```

### Run the Server
To start the API server:
```bash
cargo run
```
The server will start on `http://127.0.0.1:8000`.

## Testing the API

With the server running, you can test the available endpoints. Here's an example using `curl` to fetch all courses for the teacher with `id = 1`:

```bash
curl http://127.0.0.1:8000/courses/1
```

You should receive a JSON response with a list of courses.

## Future Work
This project is just getting started. Here are some of the features planned for the near future:
-   Full CRUD functionality for teachers and courses.
-   Student enrollment system.
-   User registration and authentication with JWT.
-   Comprehensive error handling and validation.
-   Unit and integration tests.
-   CI/CD pipeline. 