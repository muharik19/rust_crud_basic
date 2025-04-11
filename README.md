> Proposed RESTful API based service. This project is still in development stage, any critic and suggestion all very based including but not limited to project name, function naming, folder structure etc. please refer to CONTRIBUTING.md.

## Prerequisite
- Install [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) in local machine for convenience development experience (auto complete, code sugestion, etc)
- Install [rust plugin](https://www.rust-lang.org/tools) to your editor choice (ie. VSCode, Vim/Neovim, Emacs, Eclipse)
- [Docker](https://docs.docker.com/install/) and [docker-compose](https://docs.docker.com/compose/)
- Install rust hot reloading `cargo install cargo-watch`

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
- Build apps to root project `cargo build`.
- Run apps to root project hot reloading `cargo watch -x 'run'`.
- Run apps to change directory deployments use docker `docker compose up -d`.

## Creating the Tables
> Now that the database is set up, it's time to create migrations to define the tables in our schema. We will create two tables: users and items in separate migration files.

## Create the Users Table Migration
- Create the users table migration `sqlx migrate add create_users_table`
- Create the items table migration `sqlx migrate add create_items_table`

## Apply the Migrations
```sh
sqlx migrate run
```

# Project-Structure

    rust_crud_basic/
    ├── src/
    │   ├── main.rs                         # Entry point
    ├── api/                                # API definitions
    │   ├── rest/                           # REST API definitions
    │   │   ├── api/                        # REST API implementation
    │   │   │   ├── routes/                 # Routes for REST API
    │   │   │   |   ├── mod.rs              # Module declarations
    │   │   │   |   └── routes.rs           # API route definitions
    │   │   │   ├── mod.rs                  # Module declarations
    │   │   │   └── server.rs               # Server setup for REST API
    │   │   └── mod.rs                      # Module declarations
    │   ├── mod.rs                          # Module declarations
    │   ├── config/                         # Configuration management
    │   │   ├── mod.rs                      # Module declarations
    │   │   ├── settings.rs                 # Load config from env or files
    │   ├── internal/                       # Code that can only be used by this project
    │   │   ├── application/                # Application logic
    │   │   |   ├── controllers/            # Controllers for handling requests
    │   │   │   |   ├── items/              # Items related controllers
    │   │   │   |   |   ├── items.rs        # Items controller
    │   │   │   |   |   └── mod.rs          # Module declarations
    │   │   │   |   └── mod.rs              # Module declarations
    │   │   |   ├── repositories/           # Repositories for data access
    │   │   │   |   ├── items/              # Items related repositories
    │   │   │   |   |   ├── items.rs        # Items repositories
    │   │   │   |   |   └── mod.rs          # Module declarations
    │   │   │   |   └── mod.rs              # Module declarations
    │   │   |   ├── usecases/               # Use cases for application logic
    │   │   │   |   ├── items/              # Items related use cases
    │   │   │   |   |   ├── items.rs        # Items use cases
    │   │   │   |   |   └── mod.rs          # Module declarations
    │   │   │   |   └── mod.rs              # Module declarations
    │   |   |   ├── constant/               # Constant internal
    │   │   |   |   ├── mod.rs              # Module declarations
    │   │   |   |   └── status.rs           # Constant declarations
    │   |   |   ├── domain/                 # Domain models
    │   │   |   |   ├── entities/           # Domain entities
    │   │   |   |   |   ├── items/          # Items related entities group
    |   |   |   |   |   |   ├──items.rs     # Items entity definition
    |   |   |   |   |   |   └── mod.rs      # Module declarations
    │   │   |   |   |   ├── mod.rs          # Module declarations
    │   │   |   |   |   ├── response.rs     # Response standardization entities
    │   │   |   |   └── mod.rs              # Module declarations
    │   │   |   ├── pkg/                    # Internal packages
    │   │   |   |   ├── database/           # Database related code
    │   │   |   |   |   ├── sql/            # SQL database connection code
    │   │   |   |   |   |   ├── mod.rs      # Module declarations
    │   │   |   |   |   |   ├── postgres.rs # Initial Connection database
    │   │   |   |   |   ├── mod.rs          # Module declarations
    │   │   |   |   ├── utils/              # Utility functions
    │   │   |   |   |   ├── mod.rs          # Module declarations
    │   │   |   |   |   └── pagination.rs   # function declarations
    │   │   |   |   └── mod.rs              # Module declarations
    │   │   |   └── mod.rs                  # Module declarations
    │   ├── middlewares/                    # Middleware components
    │   │   ├── mod.rs                      # Module declarations
    │   │   └── auth_mw.rs                  # Authentication middleware
    ├── tests/                              # Integration tests
    │   ├── users_test.rs                   # API tests for users
    │   └── auth_test.rs                    # API tests for authentication
    ├── build/                              # Build-related files
    │   ├── package/                        # Package-related files
    │   │   └── Dockerfile                  # Dockerfile for building the application
    ├── deployments/                        # Deployment-related files
    │   └── docker-compose.yml              # Docker Compose configuration file
    ├── migrations/                         # Database migration scripts
    │   ├── 20250326071808_create_users_table.sql # Initial migration script for users
    │   └── 20250326085525_create_items_table.sql # Initial migration script for items
    ├── .env                                # Environment variables (optional)
    ├── Cargo.toml                          # Dependencies and package metadata
    ├── Cargo.lock                          # Dependency lock file (generated)
    ├── README.md                           # Project documentation
    ├── LICENSE                             # Project license
    └── target/                             # Build output (generated)

## Documentation API Postman

[API](https://documenter.getpostman.com/view/4324137/2sAYkGLega)

## References
- [Rustfinity](https://www.rustfinity.com/blog/create-high-performance-rest-api-with-rust)
- [Documentation Rust](https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html)