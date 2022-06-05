# Anonfiles-CLI

<p align="center"><img src="https://anonfiles.com/static/logo.png"></p>

## Introduction
This is a CLI tool for interacting directly with the `anonfiles` API. (from the ✨***command line***✨)

## Installation

If you don't want to build this yourself, download one of the releases and put it inside one of your PATH entries.

- Linux: usually `/bin/` or `/usr/bin/`
- Windows: `C:\Windows\System32` is good for it but don't use windows

### Building
- Clone the project (`git clone https://github.com/ItzKernel/anonfiles-cli.git`)
- Open a terminal in the project folder
- Check if you have cargo (Rust's package manager) installed, just type in `cargo`
- If cargo is installed, run `cargo build --release`
- Put the executable into one of your PATH entries

## Usage
```
anonfiles-cli -h        | Displays the usage
anonfiles-cli -u <path> | Uploading a file to anonfiles

```
