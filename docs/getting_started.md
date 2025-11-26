# Getting Started

This guide will walk you through the process of installing, building, and running the Rust Load Tester.

## Prerequisites

Before you begin, you'll need to have the following installed:

- **Rust and Cargo:** You can find instructions on how to install Rust and Cargo on the [official Rust website](https://www.rust-lang.org/tools/install).
- **C++ build tools for Visual Studio:** You can install these through the Visual Studio Installer by selecting the "Desktop development with C++" workload.

## Installation

Once you have the prerequisites, you can clone the repository and build the project.

```bash
git clone <repository-url>
cd rust-load-tester
cargo build
```

This will build the project and create an executable in the `target/debug` directory.

## Running the tool

To run the load test, you can use the following command:

```bash
cargo run
```

This will execute the test plan defined in the `config.yml` file. You can also run the executable directly:

```bash
./target/debug/rust-load-tester
```

## Next Steps

Now that you have the tool up and running, you can start creating your own test plans. For more information on how to configure the tool, please see the [configuration documentation](configuration.md).
