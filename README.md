# DPM - Docker Package Manager

[![Build](https://github.com/FlorianB-DE/dpm/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/FlorianB-DE/dpm/actions/workflows/rust.yml)

This is dpm! Your new way of running programs. 

DPM (Docker Package Manager) is a Rust-based command-line tool that simplifies the management of your installed programs and dependencies by allowing you to run them as Docker containers. 

## Features

- Replace locally installed programs with containerized versions
- Automatically manage container creation and configuration
- Use pre-built or custom images
- Easily keep track of installed programs and dependencies
- Quickly remove programs and dependencies by deleting their container images

## Usage

To use DPM, simply run the `dpm` command followed by the name of the program you want to replace with a container. For example, if you want to run a containerized version of Python, you would run:

```
dpm python
```

DPM will automatically download and run a Docker container with Python installed. You can also specify a specific version of Python using the `--tag` flag:

```
dpm --tag 3.8 python
```

To see a list of available programs and their corresponding Docker images, run:

```
dpm list
```

## Installation

To install DPM, you will need Rust installed on your system. Once you have Rust installed, you can install DPM using Cargo, Rust's package manager.

```
cargo install dpm
```

## Contributing

If you would like to contribute to DPM, please feel free to open a pull request or issue on our [GitHub repository](https://github.com/yourusername/dpm). We welcome all contributions, including bug reports, feature requests, and code contributions.

## License

DPM is released under the [MIT License](https://github.com/yourusername/dpm/blob/main/LICENSE).
