axum-vsa
A production-ready Rust/Axum project template based on Vertical Slice Architecture.
This repository provides a solid starting point for building robust, maintainable, and scalable web services in Rust using the axum web framework. It follows the Vertical Slice Architecture (VSA) pattern to organize code around features, promoting high cohesion and low coupling between different parts of the application.
🚀 Key Features
 * Vertical Slice Architecture: Code is organized by feature (e.g., user, product, order), with all related components (handlers, business logic, data models, etc.) co-located.
 * Production-Grade Dependencies:
   * Web Framework: axum for a type-safe and ergonomic API.
   * Asynchronous Runtime: tokio for high-performance, non-blocking I/O.
   * Database: sqlx for a compile-time checked, async-compatible SQL client.
   * Configuration: config crate for managing settings from files, environment variables, and command-line arguments.
   * Validation: validator crate for declarative data validation on request payloads.
   * OpenAPI Documentation: utoipa for automatic generation of OpenAPI (Swagger) documentation.
   * Observability: opentelemetry for tracing, metrics, and logging, integrated with a tracing subscriber.
 * Clean Application Context: The application state (database pool, configuration, etc.) is managed in a central AppContext struct, which is passed to handlers via axum::extract::State. This avoids the need for a global state and promotes testability.
 * Structured Error Handling: A centralized error enum (AppError) handles all application-specific errors, providing a consistent and robust way to manage and respond to errors.
 * Simple Migrations: The project includes a simple setup for managing database migrations with sqlx.
⚙️ Architectural Guidelines
Vertical Slices
Each major feature is a "vertical slice" and is contained within its own module in the src/features/ directory.
src/
├── main.rs         # Application entry point
├── config.rs       # Application configuration struct
├── app_state.rs    # Application context struct (db pool, config, etc.)
├── features/
│   ├── mod.rs      # Contains the feature modules
│   ├── health/
│   │   ├── handler.rs  # The HTTP handler for the health check
│   │   └── mod.rs
│   └── users/
│       ├── create_user.rs     # Logic for a single use case (e.g., creating a user)
│       ├── get_user.rs        # Logic for getting a single user
│       ├── handlers.rs        # All HTTP handlers for the user feature
│       ├── models.rs          # DTOs, domain models
│       ├── routes.rs          # Axum router for the user feature
│       └── mod.rs
└── ... (other common modules)

Within a slice, you can organize the code as you see fit. A common approach is to group all components for a specific "use case" or "request" together (e.g., a create_user.rs module that contains the handler, input validation logic, and business logic for creating a new user).
Configuration
Application configuration is managed by the config crate. An AppConfig struct is defined in config.rs and populated at startup. It supports loading from:
 * A config/default.toml file
 * An environment-specific config/{env}.toml file (e.g., config/development.toml)
 * Environment variables (e.g., APP__SERVER__PORT)
Application Context
The AppContext struct in app_state.rs holds shared, application-wide resources. The sqlx::PgPool is a prime example. This struct is instantiated once at application startup and then provided to every handler via axum::extract::State.
Validation
The validator crate is used for declarative validation of request payloads. A #[derive(Validate)] macro is used on structs, and the validation is performed automatically by an Axum extractor.
Observability
The tracing and opentelemetry crates are integrated to provide structured logging, tracing, and metrics. The application's main function sets up a tracing subscriber that exports traces to an OpenTelemetry collector. This allows for rich, context-aware logging and distributed tracing.
📋 Simple Functional Requirements
This project implements a simple API for managing users, demonstrating the core principles of VSA.
API Endpoints
 * POST /users
   * Request Body: JSON payload with email and password.
   * Validation:
     * email must be a valid email format.
     * password must be at least 8 characters long.
   * Behavior: Creates a new user in the database. Returns the new user's details (e.g., id, email).
   * Response: 201 Created with the new user's JSON data.
 * GET /users/:id
   * Parameters: id (UUID) in the URL path.
   * Behavior: Retrieves a user from the database by their ID.
   * Response: 200 OK with the user's JSON data, or 404 Not Found if the user does not exist.
 * GET /health
   * Behavior: A simple health check endpoint.
   * Response: 200 OK with a JSON payload indicating the service is healthy.
🚀 Getting Started
 * Clone the repository:
   git clone https://github.com/your-username/axum-vsa.git
cd axum-vsa

 * Set up your environment:
   * Ensure you have Rust and Cargo installed.
   * Install a PostgreSQL database and create a new database.
   * Install the sqlx-cli for running migrations: cargo install sqlx-cli --no-default-features --features postgres.
 * Configure:
   * Copy config/default.toml to config/development.toml.
   * Set the DATABASE_URL environment variable or add it to your .env file.
   * Example .env file: DATABASE_URL=postgres://user:password@localhost/my_app_db
 * Run migrations:
   sqlx database create
sqlx migrate run

 * Run the application:
   cargo run

   The server will start on http://localhost:3000. You can now use a tool like cURL or a REST client to interact with the API.
📝 OpenAPI Documentation
An OpenAPI specification is automatically generated and served at /api-docs/openapi.json. You can view it with a Swagger UI at /api-docs/.
This README provides a comprehensive overview of the project's structure, architectural choices, and functional requirements, making it easy for a developer to understand and get started with the VSA pattern in Rust and Axum.
