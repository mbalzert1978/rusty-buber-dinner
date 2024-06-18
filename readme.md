# Domaintemplate Workspace

This project is a Rust workspace consisting of multiple packages: `domain`, `application`, `infrastructure`, and `presentation`. This structure allows for a clear separation of responsibilities and makes it easier to manage dependencies between packages.

## Directory Structure

```bash
domaintemplate_workspace/
├── Cargo.toml
├── domain/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── application/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── infrastructure/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── presentation/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Packages

### Domain

The `domain` package contains the core logic and models of the application. It has no dependencies on other packages.

### Application

The `application` package contains the application logic and can depend on the `domain` package.

### Infrastructure

The `infrastructure` package contains the infrastructure and integration logic. It can depend on both the `domain` and `application` packages.

### Presentation

The `presentation` package is the binary package that contains the main application. It can depend on `domain`, `application`, and `infrastructure`.

## Setup

### Step 1: Create Directory Structure

```sh
mkdir domaintemplate_workspace
cd domaintemplate_workspace
mkdir domain application infrastructure presentation
```

### Step 2: Initialize the Workspace

Create a `Cargo.toml` file in the root of `domaintemplate_workspace` with the following content:

```toml
[workspace]
members = [
    "domain",
    "application",
    "infrastructure",
    "presentation"
]
```

### Step 3: Set Up Each Package

#### Domain Package

```sh
cd domain
cargo init --lib
cd ..
```

#### Application Package

```sh
cd application
cargo init --lib
cd ..
```

#### Infrastructure Package

```sh
cd infrastructure
cargo init --lib
cd ..
```

#### Presentation Package

```sh
cd presentation
cargo init --bin
cd ..
```

### Step 4: Define Dependencies

Update the `Cargo.toml` files for each package to define the dependencies as described.

#### `application/Cargo.toml`

```toml
[package]
name = "application"
version = "0.1.0"
edition = "2021"

[dependencies]
domain = { path = "../domain" }
```

#### `infrastructure/Cargo.toml`

```toml
[package]
name = "infrastructure"
version = "0.1.0"
edition = "2021"

[dependencies]
domain = { path = "../domain" }
application = { path = "../application" }
```

#### `presentation/Cargo.toml`

```toml
[package]
name = "presentation"
version = "0.1.0"
edition = "2021"

[dependencies]
domain = { path = "../domain" }
application = { path = "../application" }
infrastructure = { path = "../infrastructure" }
```

### Step 5: Verify the Setup

Run the following command from the root of your workspace to ensure everything is set up correctly:

```sh
cargo build
```

This command should build all the packages in your workspace without any errors.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
```

This README provides a comprehensive guide to setting up and understanding the structure of your Rust workspace.
