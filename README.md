<p align="center">
    <img src="logo.png" alt="logo" width="250"/>
    <h3 align="center">mtoc 📄</h3>
    <p align="center">Markdown table of contents generator</p>
    <p align="center">Built with ❤ in Rust</p>
</p>

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
  - [Exclude directories from search](#exclude-directories-from-search)
  - [Generate TOC for a specific file](#generate-toc-for-a-specific-file)
- [Using pre-commit](#using-pre-commit)
- [Example](#example)
- [Local development](#local-development)
- [IN PROGRESS](#in-progress)
- [License](#license)
<!-- END OF TOC -->

# Introduction

I'm very supper fan of [doctoc](https://github.com/thlorenz/doctoc) and I use it a lot, but I wanted to create a similar tool in Rust.

**From this:**

```markdown
# Hello
## World
### How are you?
```

**To this:**

```markdown
<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Hello](#hello)
  - [World](#world)
    - [How are you?](#how-are-you?)
<!-- END OF TOC -->
```

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
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/mtoc/main/install.sh | sh -s -- -v "v0.8.0"
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

By default, the command will search for markdown files in the current directory and generate the table of contents for each file. But, you can specify a directory to search for markdown files, specify only a specific file, or exclude directories from the search.

## Help

```bash
$ mtoc --help

Git markdown table of contents generator.

Usage: mtoc [OPTIONS]

Options:
  -d, --directory <DIRECTORY>     Directory to search for markdown files [default: .]
  -e, --exclude-dir <EXCLUDE>...  Exclude directories from search
  -f, --file <FILE>...            Only generate TOC for the specified file(s)
  -h, --help                      Print help
  -V, --version                   Print version
```

## Generate TOC

```bash
mtoc # default the current dir where the command is executed. All the files, all the directories will be scanned.
```

## Generate TOC for a specific directory

```bash
mtoc -d /path/to/directory
```

## Exclude directories from search

```bash
mtoc -e ".target/" -e "node_modules/"
```

## Generate TOC for a specific file

```bash
mtoc -f README.md
```

# Using pre-commit

Add this configuration to your `.pre-commit-config.yaml`

```yaml
- repo: https://github.com/containerscrew/mtoc
  rev: ...  # substitute a tagged version
  hooks:
    - id: mtoc
      args: ["-e", "./target"]
```

> Always stay up to date with the latest changes in the project markdown files.

# Example

```bash
$ mtoc -e ".target/"                                                                                                                                                                                                                                         🦀 v1.80.1
Excluding directories  [".target/"]
Updated markdown file  ./docs/test.md
Updated markdown file  ./CHANGELOG.md
Updated markdown file  ./README.md
```

# Local development

1. Make your changes
2. Run pre-commit
3. Test and build pipelines must pass

```bash
$ pre-commit install
# Or run once
$ pre-commit run -a
```

# IN PROGRESS

[IN PROGRESS](./docs/todo.md)

# License

`mtoc` is distributed under the terms of the [`GNU AFFERO GENERAL PUBLIC LICENSE`](./LICENSE).
