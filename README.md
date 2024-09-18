# Rust Backend API

A simple backend API built with Rust, Actix Web, and SeaORM, designed to provide user authentication, profile management, and verification functionalities.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Technologies Used](#technologies-used)
- [Project Structure](#project-structure)
- [Setup Instructions](#setup-instructions)
  - [Prerequisites](#prerequisites)
  - [Environment Variables](#environment-variables)
  - [Database Migration](#database-migration)
  - [Build and Run the Application](#build-and-run-the-application)
- [API Endpoints](#api-endpoints)
  - [Authentication](#authentication)
    - [Register User](#register-user)
    - [Login](#login)
    - [Logout](#logout)
  - [User Management](#user-management)
    - [Get User Profile](#get-user-profile)
    - [Update User Profile](#update-user-profile)
    - [Change Password](#change-password)
  - [Verification](#verification)
    - [Send OTP](#send-otp)
    - [Verify OTP](#verify-otp)
- [To-Do](#to-do)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This project is a backend API service that offers user authentication, profile management, and verification processes. It leverages Rust's performance and safety features, along with Actix Web for HTTP server capabilities and SeaORM as the Object-Relational Mapping (ORM) tool for database interactions.

## Features

- **User Authentication**: Secure user registration and login with JWT-based authentication.
- **Profile Management**: Retrieve and update user profiles.
- **Password Management**: Change user passwords securely.
- **Verification Processes**: Send and verify OTPs and email verifications.
- **Session Management**: Utilize Redis for managing user sessions.
- **Database Interactions**: Use SeaORM for database operations with MySQL.
- **Modular Architecture**: Clean separation of concerns with domain, infrastructure, and interface layers.

## Technologies Used

- **Rust**: Programming language for performance and safety.
- **Actix Web**: Web framework for building HTTP servers and clients.
- **SeaORM**: Async & dynamic ORM for Rust.
- **MySQL**: Relational database management system.
- **Redis**: In-memory data structure store for caching and session management.
- **SeaORM CLI**: Command-line tool for managing database migrations.
- **Tokio**: Asynchronous runtime for the Rust programming language.
- **Docker**: Containerization platform (optional for deployment).

## Project Structure
```
simple-backend-api/
├── Cargo.toml                 # Workspace configuration
├── migration/                 # Database migrations
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── migrations/        # Migration files
├── simple_backend_api/        # Main application crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── domain/            # Business logic layer
│       ├── infrastructure/    # Database, cache, and external services
│       └── interface/         # Controllers and routes
└── README.md                  # Project documentation
```
- **domain**: Contains business logic, entities, repositories, and services.
- **infrastructure**: Implements database connections, caching, messaging, and storage.
- **interface**: Handles HTTP requests, controllers, and data transfer objects (DTOs).

## Setup Instructions

### Prerequisites

- **Rust**: Install Rust and Cargo from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **MySQL**: Install and configure a MySQL server.
- **Redis**: Install and configure Redis.
- **SeaORM CLI**: Install SeaORM CLI for managing migrations:

  ```bash
  cargo install sea-orm-cli
  ```

### Environment Variables
Create a .env file in the root directory with the following variables:

  ```
  DATABASE_URL=mysql://username:password@localhost/database_name
REDIS_URL=redis://localhost
JWT_SECRET=your_jwt_secret
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=your_smtp_user
SMTP_PASS=your_smtp_password
API_PORT=8080
  ```

### Database Migration
Initialize the migration project and run migrations:

  ```bash
  cd migration
  sea-orm-cli migrate init
  sea-orm-cli migrate up
  ```

### Build and Run the Application
Navigate to the main application directory and run:

  ```bash
  cd simple_backend_api
  cargo run
  ```

The API server will start and listen on the port specified in the environment variables (default is 8080).

### API Endpoints
### Authentication
**Register User**
```
URL: /auth/register
Method: POST
Content-Type: application/json
Request Body:
{
  "user_name": "johndoe",
  "name_surname": "John Doe",
  "email": "john@example.com",
  "phone_number": "1234567890",
  "company_name": "Example Corp",
  "password": "password123"
}
Success Response:
  Code: 200 OK
  Content: "User registered successfully"
Error Response:
  Code: 400 Bad Request
```

**Login**
```
URL: /auth/login
Method: POST
Content-Type: application/json
Request Body:
{
"user_name": "johndoe",
"password": "password123"
}
Success Response:
  Code: 200 OK
  Content:
  {
    "token": "jwt_token_here"
  }
Error Response:
  Code: 401 Unauthorized
```

**Logout**
```
URL: /auth/logout
Method: POST
Content-Type: application/json
Request Body:
{
  "token": "jwt_token_here"
}
Success Response:
  Code: 200 OK
  Content: "Logged out successfully"
Error Response:
  Code: 400 Bad Request
```

### User Management

**Get User Profile**
```
URL: /user/get-profile
Method: POST
Content-Type: application/json
Request Body:
{
  "user_id": "user_id_here"
}
Success Response:
  Code: 200 OK
  Content:
  {
    "user_id": "user_id_here",
    "user_name": "johndoe",
    "name_surname": "John Doe",
    "email": "john@example.com",
    "phone_number": "1234567890",
    "company_name": "Example Corp"
  }
Error Response:
  Code: 404 Not Found
```

**Update User Profile**
```
URL: /user/update-profile
Method: POST
Content-Type: application/json
Request Body:
{
  "user_id": "user_id_here",
  "user_data": {
    "name_surname": "Johnathan Doe",
    "email": "johnathan@example.com"
  }
}
Success Response:
  Code: 200 OK
  Content: "Profile updated successfully"
Error Response:
  Code: 400 Bad Request
```

**Change Password**
```
URL: /user/change-password
Method: POST
Content-Type: application/json
Request Body:
{
  "user_name": "johndoe",
  "old_password": "old_password",
  "new_password": "new_password",
  "close_sessions": true
}
Success Response:
  Code: 200 OK
  Content: "Password changed successfully"
Error Response:
  Code: 400 Bad Request
```

### Verification

**Send OTP**
```
URL: /verification/send-otp
Method: POST
Content-Type: application/json
Request Body:
{
  "user_name": "johndoe",
  "otp_type": "email" // or "sms"
}
Success Response:
  Code: 200 OK
  Content: "OTP sent successfully"
Error Response:
  Code: 400 Bad Request
```

**Verify OTP**
```
URL: /verification/verify-otp
Method: POST
Content-Type: application/json
Request Body:
{
  "user_name": "johndoe",
  "otp_code": "123456"
}
Success Response:
  Code: 200 OK
  Content: "OTP verified successfully"
Error Response:
  Code: 400 Bad Request
```

### To-Do
	      •	Implement the following endpoints:
	              •	User Management
	                  •	Update Preferences (/user/update-preferences)
	                  •	Upload Profile Photo (/user/upload-photo)
	                  •	Delete Profile Photo (/user/delete-photo)
	              •	Verification
	                  •	Send Verification Email (/verification/send-verification-email)
	                  •	Verify Email (/verification/verify-email)
	      •	Enhancements:
	              •	Add comprehensive error handling and input validation.
	              •	Implement unit tests and integration tests.
	              •	Improve logging and monitoring mechanisms.
	              •	Optimize database queries and indexes.
	              •	Dockerize the application for easier deployment.

### Contributing
Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch with a descriptive name.
3. Make your changes and commit them.
4. Open a pull request with a detailed description of your changes.