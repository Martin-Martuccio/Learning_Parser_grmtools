# Calculator Parser Project

This project implements a simple calculator parser using Yacc, grmtools, and a custom lexer to interpret mathematical expressions. The implementation follows detailed instructions and guidelines from the grmtools documentation, accessible at [Grmtools Documentation](https://softdevteam.github.io/grmtools/latest_release/book/).

## Overview

The parser can interpret basic mathematical expressions, handling operations like addition and multiplication. It was developed using Yacc for grammar definition, while grmtools was used to facilitate parsing and syntax error handling.

## File Structure

- `main.rs`: Contains the main Rust code for running the parser.
- `build.rs`: Build script used to configure the parser compilation process.
- `errors.txt`: Document describing how errors work during parsing.

The `main.rs` and `build.rs` files include detailed comments that explain the operation of the code and the implementation choices. These files are recommended for a better understanding of the parser's logic.

## Dependency Management

The project uses local dependencies initially. However, these dependencies can also be sourced from online repositories by specifying the version in the `Cargo.toml` file. For instructions on downloading and linking these libraries from an online source, please refer to the [Libraries and Tools Section](https://softdevteam.github.io/grmtools/latest_release/book/libsandtools.html) of the grmtools documentation.

## Parsing Errors and Recovery

The parser features an advanced error recovery system capable of suggesting useful corrections in case of syntax errors. This allows users to correct multiple errors in a single pass. For a detailed discussion on how the system handles and recovers from syntax errors, refer to [Error Recovery in Grmtools](https://softdevteam.github.io/grmtools/latest_release/book/errorrecovery.html).

## Usage

To use the parser, follow these steps:

1. Clone the repository.
2. Compile the project with `cargo build`.
3. Run the program with `cargo run`.

Ensure you have Rust and Cargo installed on your machine to compile and run the parser.

## License

This project is distributed under the MIT license. Further details can be found in the `LICENSE` file included in the repository.

## Contributing

Contributions to the project are welcome, whether they be bug reports, suggestions for improvements, or pull requests. To contribute, you can open an issue or a pull request on GitHub.
