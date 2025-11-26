# Development

This document provides a step-by-step guide on how the Rust Load Tester was built. This is intended to be used by developers and AI IDEs to understand the project's history and structure.

## Step 1: Project Initialization

1.  **Create a new Rust project:**
    ```bash
    cargo new rust_load_tester
    ```

2.  **Add dependencies:**
    The following dependencies were added to the `Cargo.toml` file:
    - `tokio`: For asynchronous runtime.
    - `reqwest`: For making HTTP requests.
    - `serde`: For serializing and deserializing data.
    - `serde_json`: For working with JSON data.
    - `serde_yaml`: For parsing the YAML configuration file.
    - `config`: For managing configuration.

## Step 2: Project Structure

1.  **Create the `docs` directory:**
    ```bash
    mkdir docs
    ```

2.  **Create the `src/config.rs` and `src/runner.rs` files:**
    ```bash
    touch src/config.rs src/runner.rs
    ```

## Step 3: Configuration Module

The `src/config.rs` file defines the data structures for the test configuration.

1.  **Define the structs:**
    The `TestConfig`, `TestStep`, and `Request` structs were defined to represent the configuration.

2.  **Add `serde::Deserialize`:**
    The `serde::Deserialize` derive macro was added to the structs to allow them to be deserialized from a YAML file.

3.  **Add `Clone`:**
    The `Clone` derive macro was added to the structs to allow them to be cloned.

## Step 4: Test Runner

The `src/runner.rs` file contains the logic for executing the load test.

1.  **Create the `run_test` function:**
    The `run_test` function is the main entry point for the test runner. It takes a `TestConfig` as input and executes the test steps.

2.  **Implement concurrency:**
    The `tokio::spawn` function is used to run the requests concurrently.

3.  **Implement step chaining:**
    The response from the previous step is passed to the next step, allowing for simple templating.

## Step 5: Main Entry Point

The `src/main.rs` file is the main entry point for the application.

1.  **Load the configuration:**
    The configuration is loaded from a `config.yml` file.

2.  **Call the test runner:**
    The `run_test` function is called with the loaded configuration.

3.  **Make the `main` function asynchronous:**
    The `#[tokio::main]` attribute is used to make the `main` function asynchronous.

## Step 6: Documentation

The following documentation files were created:

- `README.md`: A high-level overview of the project.
- `docs/introduction.md`: An introduction to the tool.
- `docs/getting_started.md`: A guide on how to get started.
- `docs/configuration.md`: A detailed explanation of the configuration file format.
- `docs/development.md`: This file, which provides a step-by-step guide on how the project was built.
