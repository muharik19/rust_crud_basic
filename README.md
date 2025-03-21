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
    ├── README.md         # Project description
    ├── LICENSE           # Project license
    ├── Cargo.toml        # Dependency list
    ├── src/              # Application logic
    │   ├── config.rs     # Configuration
    │   ├── handlers.rs   # Handling requests
    │   ├── main.rs       # Main applications running
    │   ├── models.rs     # Domain models
    │   ├── repository.rs # Repository for data access
    │   ├── routes.rs     # Routes for RESTful API

## Documentation API Postman

[API](https://documenter.getpostman.com/view/4324137/2sAYkGLega)