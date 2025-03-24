> Proposed RESTful API based service. This project is still in development stage, any critic and suggestion all very based including but not limited to project name, function naming, folder structure etc. please refer to CONTRIBUTING.md.

## Prerequisite

- Install [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) in local machine for convenience development experience (auto complete, code sugestion, etc)
- Install [rust plugin](https://www.rust-lang.org/tools) to your editor choice (ie. VSCode, Vim/Neovim, Emacs, Eclipse)
- [Docker](https://docs.docker.com/install/) and [docker-compose](https://docs.docker.com/compose/)

## Continuous Integration support with Github Actions
- Rust: Triggers the following on every push or pull request, using the latest stable toolchain:
    - Ensure uniform code formatting `cargo fmt`
    - Ensure idiomatic code `cargo clippy`
    - Ensure compilation succeeds on Linux, MacOS, Windows and WebAssembly `cargo check`
    - Run all tests `cargo test`
    - Run all benchmarks `cargo bench`
- Release: Create a new GitHub Release draft when a tag starting with `v` is pushed.
  - Publish: Automated publishing of binary assets for a GitHub Release:
    - Build binaries for Linux, MacOS, Windows and WebAssembly
    - Archive binaries with a license, readme and appropate files for each platform
    - Upload archives as assets for the appropriate GitHub release

## How to run

Despite, it is possible to run this project in local machine Please follow this steps:
- Changes file .env to your config `.cp .env.example to .env`.
- Run apps to root project `cargo run`.
- Run apps to root project `cargo build`.
- Run apps to root project use docker `docker compose up -d`.

# Project-Structure

    rust_crud_basic/
    ├── src/
    │   ├── main.rs          # Entry point
    │   ├── lib.rs           # Optional (for shared logic)
    │   ├── api/             # API route handlers
    │   │   ├── mod.rs       # Module declarations
    │   │   ├── users.rs     # User-related endpoints
    │   │   ├── auth.rs      # Authentication endpoints
    │   ├── db/              # Database abstraction
    │   │   ├── mod.rs       # Module declarations
    │   │   ├── models.rs    # Data models
    │   │   ├── schema.rs    # ORM schema (if using Diesel)
    │   │   ├── repository.rs # Database operations
    │   ├── services/        # Business logic
    │   │   ├── mod.rs       # Module declarations
    │   │   ├── user_service.rs # User-related business logic
    │   ├── config/          # Configuration management
    │   │   ├── mod.rs       # Module declarations
    │   │   ├── settings.rs  # Load config from env or files
    │   ├── middlewares/     # Middleware components
    │   │   ├── mod.rs       # Module declarations
    │   │   ├── auth_mw.rs   # Authentication middleware
    │   ├── utils/           # Utility functions/helpers
    │   │   ├── mod.rs       # Module declarations
    │   │   ├── hash.rs      # Password hashing utilities
    │   ├── errors.rs        # Error handling
    │   ├── routes.rs        # API route definitions
    │   ├── app.rs           # Application setup (router, middleware)
    ├── tests/               # Integration tests
    │   ├── users_test.rs    # API tests for users
    │   ├── auth_test.rs     # API tests for authentication
    ├── migrations/          # Database migrations (Diesel)
    ├── .env                 # Environment variables (optional)
    ├── Cargo.toml           # Dependencies and package metadata
    ├── Cargo.lock           # Dependency lock file (generated)
    ├── README.md            # Project documentation
    ├── LICENSE              # Project license
    └── target/              # Build output (generated)

## Documentation API Postman

[API](https://documenter.getpostman.com/view/4324137/2sAYkGLega)