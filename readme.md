# BYTE4DIRECTORY

**BYTE4DIRECTORY** is a minimal implementation of a function selector for Starknet. The project is inspired by the [Ethereum Signature Database](https://www.4byte.directory/).

## Installation

To run the project, you need to have Rust installed. If Rust is not installed, you can install it by following the instructions [here](https://www.rust-lang.org/tools/install).

### Steps:

1. **Clone the repository**:
    ```bash
    git clone https://github.com/codeWhizperer/bytedirectory.git
    ```

2. **Install dependencies**:
    Navigate to the project directory and build the project:
    ```bash
    cargo build
    ```

3. **Configure for local development**:
    - Update the port configuration in the source code. Change `.bind(("0.0.0.0:8088"))?` to `.bind(("127.0.0.1:8080"))?` to bind the server to localhost on port 8080.

4. **Run the server**:
    After configuring the project, run the following command to start the server:
    ```bash
    cargo run
    ```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
