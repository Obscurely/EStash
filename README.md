<div id="top"></div>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <img src="https://github.com/Obscurely/EStash/blob/master/assets/logo.png" alt="Logo" width="150" height="150">

  <h1 align="center"></h1>

  <p align="center">
    A cross-platform encrypted digital vault. Built in Rust using ECIES cryptography and Argon2id key derivation for secure, one-click file deployment.
    <br />
    <a href="https://github.com/Obscurely/EStash/issues">Report Bug</a>
    |
    <a href="https://github.com/Obscurely/EStash/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#overview">Overview</a>
      <ul>
        <li><a href="#video-showcase">Video showcase</a></li>
        <li><a href="#built-with">Built with</a></li>
        <ul>
          <li><a href="#core-dependencies">Core Dependencies</a></li>
        </ul>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#running-the-program">Running the Program</a></li>
        <ul>
          <li><a href="#windows">Windows</a></li>
          <ul>
            <li><a href="#portable-exe">Portable EXE</a></li>
            <li><a href="#installer">Installer</a></li>
          </ul>
          <li><a href="#linux">Linux</a></li>
          <ul>
            <li><a href="#portable-bin">Portable Bin</a></li>
            <li><a href="#appimage">AppImage</a></li>
            <li><a href="#aur">AUR</a></li>
            <li><a href="#nix-file">Nix File</a></li>
            <li><a href="#deb-file">Deb File</a></li>
            <li><a href="#arch-pkg-file">Arch Pkg File</a></li>
          </ul>
          <li><a href="#macos">MacOS</a></li>
          <ul>
            <li><a href="#portable-binary">Portable Bin</a></li>
            <li><a href="#app-folder">App Folder</a></li>
            <li><a href="#dmg-installer">DMG Installer</a></li>
            <li><a href="#homebrew">Homebrew</a></li>
          </ul>
          <li><a href="#all-platforms">All Platforms</a></li>
        </ul>
        <li><a href="#compilation">Compilation</a></li>
      </ul>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#basics">Basics</a></li>
        <ul>
          <li><a href="#login">Login</a></li>
          <li><a href="#vault-operations">Vault Operations</a></li>
        </ul>
        <li><a href="#advanced">Advanced</a></li>
      </ul>
    </li>
    <li><a href="#road-map">Road Map</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

## Overview

An open source encrypted vault for text and files, written in Rust. Each vault can assign a target filesystem path, enabling one-click content deployment to that location. For example, store SSH keys in a vault, place the vault within a dotfiles repository, and synchronize keys across machines.<br>

Vault keys are derived via Argon2id. Implements ECIES (X25519 Diffie-Hellman + XChaCha20Poly1305) for vault encryption.

Vault namespaces are hashed via BLAKE3. Passwords are never stored; they are used to derive a 32-byte Argon2id key, which decrypts the vault's private ECIES key. A successful login requires decryption of the private key and presence of the vault name. All vault content is encrypted using the vault's key pair.

### Video showcase

https://user-images.githubusercontent.com/59087558/206248579-a786b277-b0fc-4306-be50-9db1c948e901.mp4

### Built with

- [Rust 1.64.0](https://www.rust-lang.org/)

#### Core Dependencies:

- [BLAKE3](https://lib.rs/crates/blake3) cryptographic hash function.
- [rust-argon2](https://lib.rs/crates/rust-argon2) password hashing.
- [rand](https://lib.rs/crates/rand) random number generation.
- [rand_hc](https://lib.rs/crates/rand_hc) HC128 random number generator.
- [zeroize](https://lib.rs/crates/zeroize) secure memory zeroing.
- [crypto_box](https://lib.rs/crates/crypto_box) ECIES implementation (X25519 + XChaCha20Poly1305).
- [chacha20poly1305](https://lib.rs/crates/chacha20poly1305) AEAD encryption.
- [sled](https://lib.rs/crates/sled) embedded database.
- [Serde](https://lib.rs/crates/serde) serialization framework.
- [serde_json](https://lib.rs/crates/serde_json) JSON serialization.
- [FLTK](https://lib.rs/crates/fltk) GUI bindings.
- [dirs](https://lib.rs/crates/dirs) platform-specific standard directories.

## Getting Started

### Running The Program

- [Windows](#windows)
- [Linux](#linux)
- [MacOS](#macos)
- [All Platforms](#all-platforms)

---

### Windows

- [Portable EXE](#portable-exe)
- [Installer](#installer)

#### Portable EXE

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-windows.exe_ file.
2. Execute the downloaded file.
3. Refer to the [Usage](#usage) section for operational details.

#### Installer

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-windows-installer.exe_ file.
2. Execute the installer and follow the provided steps.
3. A shortcut will be created in the Start Menu or on the desktop.
4. Refer to the [Usage](#usage) section for operational details.

---

### Linux

- [Portable Bin](#portable-bin)
- [AppImage](#appimage)
- [AUR](#aur)
- [Nix File](#nix-file)
- [Deb File](#deb-file)
- [Arch Pkg File](#arch-pkg-file)

#### Portable Bin

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-linux_ file.
2. Execute the downloaded file.
3. Refer to the [Usage](#usage) section for operational details.

#### AppImage

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-linux.AppImage_ file.
2. Execute the AppImage. For system integration, consider installing [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher).
3. Refer to the [Usage](#usage) section for operational details.

#### AUR

The available packages are: estash (stable), estash-bin (precompiled), and estash-git (source).

a. Using an AUR helper such as paru or yay:

1. Execute the installation command for your chosen package. For example:

```shell
paru -Sy estash
```

2. Launch the application from your system launcher.
3. Refer to the [Usage](#usage) section for operational details.

b. Manual installation from AUR:

1. Install build dependencies:

```shell
sudo pacman -Sy gcc base-devel --needed
```

2. Clone the package repository:

```shell
git clone https://aur.archlinux.org/estash.git
```

3. Navigate to the cloned directory and build the package:

```shell
makepkg
```

4. Install the built package:

```shell
sudo pacman -U estash-*.pkg.tar.zst
```

#### Nix File

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-linux.nix_ file.
2. For Nix flakes, place the file in your packages directory and add the appropriate tag. For non-flake setups, integrate the code into your default.nix.
3. Refer to the [Usage](#usage) section for operational details.

#### Deb File

Note: This method does not provide a desktop entry. Execution is via terminal.

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-linux.deb_ file.
2. Open a terminal in the download directory and execute:

```shell
sudo dpkg -i estash-linux.deb
```

3. Execute `estash` from the terminal.
4. Refer to the [Usage](#usage) section for operational details.

#### Arch Pkg File

Note: Using the AUR package is the recommended method.

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-linux.pkg.tar.zst_ file.
2. Execute the installation command:

```shell
sudo pacman -U estash-linux.pkg.tar.zst
```

3. Launch the application from your system launcher.
4. Refer to the [Usage](#usage) section for operational details.

---

### MacOS

- [Portable binary](#portable-binary)
- [App Folder](#app-folder)
- [DMG Installer](#dmg-installer)
- [Homebrew](#homebrew)

#### Portable binary

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-macos_ file.
2. Execute the downloaded file.
3. Refer to the [Usage](#usage) section for operational details.

#### App Folder

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-macos-app.tar.gz_ file.
2. Extract the archive:

```shell
tar -xzf estash-macos-app.tar.gz
```

3. Execute the extracted application.
4. Refer to the [Usage](#usage) section for operational details.

#### DMG Installer

1. Navigate to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the _estash-macos-installer.dmg_ file.
2. Mount the DMG file.
3. Drag the application to the Applications folder.
4. The application will appear in Launchpad.
5. Refer to the [Usage](#usage) section for operational details.

#### Homebrew

Note: This method does not create a desktop entry. Execute via the `estash` command.

1. Ensure [Homebrew](https://brew.sh) is installed.
2. Add the required tap repository:

```shell
brew tap Obscurely/tap
```

3. Install the package:

```shell
brew install estash
```

4. Refer to the [Usage](#usage) section for operational details.

---

### All Platforms

This method is supported on Linux, Windows 10/11, and macOS (Big Sur and later).

1. Install Rust via the official [rustup installer](https://www.rust-lang.org/tools/install) or a system package manager.
2. Execute the installation command:

```shell
cargo install estash
```

3. Ensure `$HOME/.cargo/bin` is in your PATH. For Linux and macOS, add the following to your shell configuration file (e.g., `$HOME/.zshrc`):

```shell
export PATH=$HOME/.cargo/bin:$PATH
```

On Windows, the PATH is typically configured automatically; a system restart may be required after initial Rust installation. 4. For convenience, create a symlink (Linux/macOS) or a desktop shortcut (Windows) to the binary. 5. To update, re-run the `cargo install estash` command.

---

### Compilation

This program uses cross-platform libraries. Rust must be installed.

1. Clone the repository:

```shell
git clone https://github.com/Obscurely/estash.git
```

Alternatively, download the source as a ZIP archive from the repository page and extract it.

2. Navigate to the extracted directory:

```shell
cd the/path
```

3. Compile the release build:

```shell
cargo build --release
```

Compilation time varies by system.

4. The executable is located at `target/release/estash`.

## Usage

Refer to the provided [video](#video-showcase) for a functional overview.

### Basics

#### Login

1. Select Signup. Provide a vault name, password, and password confirmation. Click Signup.
2. After vault creation, click the back arrow in the upper-left corner.
3. From the main menu, select Login. Provide the vault name and password. Click Login.

#### Vault Operations

- Add an entry: Input text in the lower-left text box and click the adjacent plus sign.
- Retrieve entry content: Click the entry name in the tree view.
- Enable/disable install path: Click the plus/minus sign adjacent to the install path box.
- Validate install path: Click the Check button adjacent to the install path box.
- Content box: Stores UTF-8 text content. Non-UTF-8 files (e.g., images) disable this box and are stored directly.
- Clear Content: Empties the content box.
- Select File: Opens a file selector to import file content into the content box. Large or binary files are stored directly.
- Notes box: Optional metadata field.
- Delete: Removes the selected entry.
- Install: Writes the current content box data to the enabled install path.
- Save: Encrypts and persists the current entry to the database.

### Advanced

- Modify the install path without saving the entry for one-time use.
- Modify content without saving, for direct installation to a file.

## Road Map

The project roadmap is available in the Projects tab or at [this link](https://github.com/users/Obscurely/projects/1).

## Contributing

Read [CONTRIBUTING.md](https://github.com/Obscurely/EStash/blob/master/CONTRIBUTING.md). The standard process is to edit a file, submit a pull request, and await review.

## License

Distributed under the GPL-3.0 License. See LICENSE for details.

## Contact

Submit an issue via the [Issues Tab](https://github.com/Obscurely/falion/issues) or contact [EMAIL](me@obscurely.dev).

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/Obscurely/EStash.svg?style=for-the-badge
[contributors-url]: https://github.com/Obscurely/EStash/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Obscurely/EStash.svg?style=for-the-badge
[forks-url]: https://github.com/Obscurely/EStash/network/members
[stars-shield]: https://img.shields.io/github/stars/Obscurely/EStash.svg?style=for-the-badge
[stars-url]: https://github.com/Obscurely/EStash/stargazers
[issues-shield]: https://img.shields.io/github/issues/Obscurely/EStash.svg?style=for-the-badge
[issues-url]: https://github.com/Obscurely/EStash/issues
[license-shield]: https://img.shields.io/github/license/Obscurely/EStash.svg?style=for-the-badge
[license-url]: https://github.com/Obscurely/EStash/blob/master/LICENSE
