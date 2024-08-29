<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Introduction](#introduction)
- [Badges](#badges)
- [Supported Platforms](#supported-platforms)
- [Installation](#installation)
  - [Install latest binary version](#install-latest-binary-version)
  - [Install specific binary version](#install-specific-binary-version)
  - [Install using cargo](#install-using-cargo)
- [Uninstall](#uninstall)
- [Usage](#usage)
  - [Help](#help)
  - [Generate TOC](#generate-toc)
  - [Generate TOC for a specific directory](#generate-toc-for-a-specific-directory)
- [Local development](#local-development)
- [Or run once](#or-run-once)
- [TODO](#todo)
- [License](#license)
<!-- END OF TOC -->

> [!WARNING]
> v0.1.0 is working fine unless will not respect content above the TOC. I'm working on it. Visit the [TODO](#todo) section for more information.

<p align="center">
    <img src="logo.png" alt="logo" width="250"/>
    <h3 align="center">mtoc 📄</h3>
    <p align="center">Markdown table of contents generator</p>
    <p align="center">Built with ❤ in Rust</p>
</p>

# Introduction

I'm very supper fan of [doctoc](https://github.com/thlorenz/doctoc) and I use it a lot, but I wanted to create a similar tool in Rust.

# Badges

|               |                                                                                                                                                                                                                                                 |
|---------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Language      | ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)                                                                                                                                          |
| Crates        | ![Crates.io Version](https://img.shields.io/crates/v/mtoc)                                                                                                                                                                                      |
| Release       | [![Release](https://img.shields.io/github/release/containerscrew/mtoc)](https://github.com/containerscrew/mtoc/releases/latest)                                                                                                                 |
| Code          | ![Code Size](https://img.shields.io/github/languages/code-size/containerscrew/mtoc)                                                                                                                                                             |
| CI - Build    | [![Build](https://github.com/containerscrew/mtoc/actions/workflows/build.yml/badge.svg)](https://github.com/containerscrew/mtoc/actions/workflows/build.yml)                                                                                    |
| CI - Release  | [![Build](https://github.com/containerscrew/mtoc/actions/workflows/release.yml/badge.svg)](https://github.com/containerscrew/mtoc/actions/workflows/release.yml)                                                                                |
| CI - Test     | [![Build](https://github.com/containerscrew/mtoc/actions/workflows/test.yml/badge.svg)](https://github.com/containerscrew/mtoc/actions/workflows/test.yml)                                                                                      |
| CI - Coverage | [![Build](https://github.com/containerscrew/mtoc/actions/workflows/coverage.yml/badge.svg)](https://github.com/containerscrew/mtoc/actions/workflows/coverage.yml)                                                                              |
| Meta          | [![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit) [![License - MIT](https://img.shields.io/github/license/containerscrew/mtoc)](/LICENSE) |
| Codecov       | [![codecov](https://codecov.io/github/containerscrew/mtoc/graph/badge.svg?token=UTTEOR9BUO)](https://codecov.io/github/containerscrew/mtoc)                                                                                                     |
| Downloads     | [![Downloads](https://img.shields.io/github/downloads/containerscrew/mtoc/total.svg?logo=github)](https://somsubhra.github.io/github-release-stats/?username=containerscrew&repository=mtoc)                                                    |


# Supported Platforms

| Arch    | ARM64 | AMD64 |
|---------|------|------|
| darwin  | ✅    | ✅  |
| linux   | ✅    | ✅  |
| windows | ❌     | ❌   |


# Installation

## Install latest binary version

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/mtoc/main/install.sh | sh
```

## Install specific binary version

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/gitrack/main/install.sh | sh -s -- -v "v0.8.0"
```

## Install using cargo

* **[Install Cargo](https://rustup.rs/)**

```shell
cargo install mtoc
```

# Uninstall

Binary:

```bash
sudo rm /usr/local/bin/mtoc
```

With cargo:

```shell
cargo uninstall mtoc
```

# Usage

## Help

```bash
mtoc --help

Git markdown table of contents generator.

Usage: mtoc [OPTIONS]

Options:
  -d, --directory <DIRECTORY>  Directory to search for markdown files [default: .]
  -h, --help                   Print help
  -V, --version                Print version
```

## Generate TOC

```bash
mtoc # default the current dir where the command is run
```

## Generate TOC for a specific directory

```bash
mtoc -d /path/to/directory
```

# Local development

1. Make your changes
2. Run pre-commit
3. Test and build pipelines must pass

```shell
pre-commit install
# Or run once
pre-commit run -a
```

# TODO

* When generate the TOC, respect the content above the TOC.
* Push the package to crates.io
* Generate the TOC for a specific file
* Exclude specific directories
* Exclude CHANGELOG.md always
* Push to homebrew

# License

[License](./LICENSE)
