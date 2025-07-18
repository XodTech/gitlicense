# Gitlicense - CLI Utility for Adding License File to Your Project

## Introduction

Gitlicense is a CLI utility that helps you add license files to your projects. It supports multiple license types and provides a simple way to configure and manage licenses.

## Getting Started


1. Install Gitlicense using the [installation instructions](#installation) below.
2. Configure it using the [configuration instructions](#configuration) bellow
3. Run `gitlicense --help` to get available CLI arguments.

## Features

-   Add licenses to projects
-   Support for multiple license types
-   Configuration options for customizing the license file

## Installation

### On UNIX-Based OS:

-   Move into the directory where you cloned this repository.
-   Ensure you have `rust` installed on your machine.
-   Run the following command: `cargo build --release`.
-   Then, you can use the `install.sh` script or manually copy the executable binary to your `/usr/bin` directory. Optionally, you can add an alias `gl = "gitlicense"` to your shell configuration.

### Windows:
*That is a new feature,and it hasn't beent tested on Windows machine yet.If something doesn't work correctly please create an issue and let me know*

- Ensure you have the Rust compiler installed on your machine. You can download it from the official [Rut-lang](https://www.rust-lang.org/tools/install) website if you haven't already
- Execute the `install.bat` script to install Gitlicense

**Important Note:** Never run `.bat` scripts from untrusted sources, as they can potentially harm your system. Only run scripts from sources you trust, and make sure you understand what the script is doing before executing it.

## Configuration

### Add Default Configuration

- Run `gitlicense` in your terminal, and a default configuration file will be created.

```toml
[User]
Name  =  "YOUR_NAME"

[Paths]
TargetLicenseFilename  =  "LICENSE"
LicensesPath  =  "./licenses"

[Settings]
SetUsername  =  true
SetDate  =  true
SetCustomMessage  =  true
AlwaysOverwrite  =  true
```

### Configure it Yourself

-   Move to `~/.config/gitlicense`.
-   Create or edit the `config.toml` file.

Available configuration fields:
``` toml
[User]
Name  =  ""

[Paths]
TargetLicenseFilename  =  ""
LicensesPath  =  ""  # Path to licenses you want to use

[Settings]
SetUsername  =  true #or false
SetDate  =  true #or false
SetCustomMessage  =  true #or false
AlwaysOverwrite  =  true #or false
```

## Adding Licenses

### Simple Licenses (One File)

-   You need to add license content to a file with a name that you will use in the CLI, into the directory that you provided in the 'LicensesPath' field.
-   Then you can modify license content and add there special placeholders:
which will be automatically substituted by the CLI.
Special placeholders you can provide:
    + `[USERNAME]` - will be substituted with your username, as provided in the configuration
    + `[DATE]` - will be substituted with the current year
    + `[CUSTOM_MESSAGE]` - will be substituted with a custom message that you can provide when invoking the `gitlicense` command

### Complex Licenses (Two File)

-   You need to create `your_license_name.toml` in the directory where you store all your licenses.
-   Add the following content to it:

```toml
[LICENSE]  # Section for short license
Content  =  "" # You can add special placeholders here, such as in one file's licenses
[FULL_LICENSE]  # Section for full license
Filename  =  ""
Content  =  ""
```


## Contributing

To contribute to this repository:

- Fork the repository
- Clone the forked repository to your local machine
- Create a new branch for your contribution
- Make changes, commit, and push to your fork
- Create a pull request to the original repository

Please:
- Follow the existing coding style
- Test new features
- Use commit messages that are meaningful and consistent in style with existing ones

## License
Gitlicense is a free software and it's released under the terms of the GNU GPL v3 [LICENSE file](LICENSE)
