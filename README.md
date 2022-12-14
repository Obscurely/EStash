<div id="top"></div>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h1 align="center">EStash</h1>

  <p align="center">
    An open source, cross-platform, programmed in rust, encrypted digital vault (store files and text) with the capability to set a path and 
    with the click of a button to copy the content to that file. For example store your ssh keys safely, put your vault in like your github dotfiles, 
    download it on another machine and easily install those keys.
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
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="supported-programming-sources">Supported programming sources</a></li>
        <li><a href="#video-showcase">Video showcase</a></li>
        <li><a href="#built-with">Built with</a></li>
        <ul>
          <li><a href="#the-stock-libraries-and-these-awesome-3rd-party-ones">The stock libraries and these awesome 3rd party ones</a></li>
        </ul>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#running-the-program">Running the Program</a></li>
        <ul>
          <li><a href="#install-with-cargo">Install with cargo</a></li>
          <li><a href="#install-from-aur">Install from AUR</a></li>
          <li><a href="#install-from-provided-binaries">Install from provided binaries</a></li>
          <li><a href="#manually">Manually</a></li>
        </ul>
        <li><a href="#compilation">Compilation</a></li>
      </ul>
    </li>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#basics">Basics</a></li>
        <li><a href="#key-binds">Key Binds</a></li>
      </ul>
    </li>
    <li><a href="#road-map">Road Map</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

## About The Project
* An open source, programmed in rust, encrypted digital vault (store files and text) with the capability to set a path and 
with the click of a button to copy the content to that file. For example store your ssh keys safely, put your vault in like your github dotfiles, 
download it on another machine and easily install those keys.<br>
* The vault is encrypted using a key derived from your password (the strength of your password decides the safetyness of your vault) using argon2id 
and that key is used to encrypt the private key. The encryption algorithm used is an ECIES, combines X25519 Diffie-Hellman function and XChaCha20Poly1305. (I used an [ECIES](https://itecspec.com/spec/3gpp-33-501-c-3-elliptic-curve-integrated-encryption-scheme-ecies/) for future proof reasons as there are no security downsides)

### Video showcase

https://user-images.githubusercontent.com/59087558/206248579-a786b277-b0fc-4306-be50-9db1c948e901.mp4

### Built with

- [Rust 1.64.0](https://www.rust-lang.org/)

#### The stock libraries and these awesome 3rd party ones:
- [BLAKE3](https://lib.rs/crates/blake3) hash function, much faster then sha2 and more secure.
- [rust-argon2](https://lib.rs/crates/rust-argon2) for deriving the encryption key from the password.
- [rand](https://lib.rs/crates/rand) random number generators and other randomness functionality.
- [rand_hc](https://lib.rs/crates/rand_hc) HC128 random number generator.
- [zeroize](https://lib.rs/crates/zeroize) securely clear secrets from memory with a simple trait.
- [crypto_box](https://lib.rs/crates/crypto_box) [ECIES](https://itecspec.com/spec/3gpp-33-501-c-3-elliptic-curve-integrated-encryption-scheme-ecies/) that combines X25519 Diffie-Hellman function and XChaCha20Poly1305.
- [chacha20poly1305](https://lib.rs/crates/chacha20poly1305) simple, fast and strong [AEAD](https://en.wikipedia.org/wiki/Authenticated_encryption) encryption algorithm.
- [sled](https://lib.rs/crates/sled) lightweight high-performance pure-rust transactional embedded database.
- [Serde](https://lib.rs/crates/serde) a generic serialization/deserialization framework.
- [serde_json](https://lib.rs/crates/serde_json) a JSON serialization file format.
- [FLTK](https://lib.rs/crates/fltk) rust bindings for the FLTK GUI library.
- [dirs](https://lib.rs/crates/dirs) a tiny low-level library that provides platform-specific standard locations.


## Getting Started

### Running The Program
- [Windows](#windows)

### Windows
- [Portable EXE](#portable-exe)
- [Installer](#installer)

#### Portable EXE
1. Go to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the *estash-windows.exe* file (might have to click show all).
2. Double click the exe you just downloaded and there you go the program works.
3. Might wanna take a look at the [Usage Tab](usage) if you don't understand something about it.

#### Installer
1. Go to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the *estash-windows-installer.exe* files (might have to click show all).
2. Double click the installer and go through the it as you would with any other installer.
3. If you look now in the start menu (or on the desktop if you ticked create desktop shortcut) you are gonna see a shortcut for estash, just run it like any other program.
4. Might wanna take a look at the [Usage Tab](usage) if you don't understand something about it.

### Linux
- [Portable Bin](#portable-bin)
- [AppImage](#appimage)
- [AUR](#aur)
- [Nix File](#nix-file)
- [Deb File](#deb-file)
- [Arch Pkg File](#arch-pkg-file)

#### Portable Bin
1. Go to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the *estash-linux* file.
2. Double click the bin you just downloaded and there you go the program works.
3. Might wanna take a look at the [Usage Tab](usage) if you don't understand something about it.

#### AppImage
1. Go to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the *estash-linux.AppImage* file.
2. Double click the AppImage you download and there you go the program just works. You may want to install [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher) if you don't have it already, when you start the AppImage you'll get a prompt asking if you want to integrate and run it and if you do so it will appear just as if you installed it.
3. Might wanna take a look at the [Usage Tab](usage) if you don't understand something about it.

#### AUR
The PKGs are: estash (for stable), estash-bin (for precompiled) and estash-git (to compile latest source code)

a. if you have an AUR manager (like [paru](https://github.com/Morganamilo/paru/blob/master/README.md#installation) or [yay](https://github.com/Jguer/yay/blob/next/README.md#installation), which you should)
  1. Just like with any other AUR pkg choose your prefered type and you can run the following command for example.
  ```shell
  paru -Sy estash
  ```
  2. Search for estash in your app launcher and launch it.
  3. Might wanna take a look at the [Usage Tab](usage) if you don't understand something about it.
  
b. Manually cloning and building it from AUR
  1. First install the basic build dependencies, if you don't already have them:
  ```shell
  sudo pacman -Sy gcc base-devel --needed
  ```
  2. Then clone the build script
  ```shell
  git clone https://aur.archlinux.org/estash.git # or estash-bin & estash-git
  ```
  3. Cd into the new cloned repository and run the following to build the package
  ```shell
  makepkg
  ```
  4. In order to install the package run the following (where * is just an any other characters place holder)
  ```shell
  sudo pacman -U estash-*.pkg.tar.zst
  ```

#### Nix File
You are using NixOS, don't worry I got you bro.
1. Go to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the *estash-linux.nix* file.
2. If you use flakes then put it in your pkgs folder, and up-top add your tag (like *my*). If you don't just add the code in your default.nix file and install it this way.
3. Might wanna take a look at the [Usage Tab](usage) if you don't understand something about it.

#### Deb File
You should use the app image. This does not provide a desktop file, you'll have to run it from the command line. It's here just as another means if needed. I will try to make a ppa.
1. Go to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the *estash-linux.deb* file.
2. Open a terminal in the folder where your download is and run the following command:
```shell
sudo dpkg -i estash-linux.deb
```
3. Run *estash* in the terminal and there it is, the app.
4. Might wanna take a look at the [Usage Tab](usage) if you don't understand something about it.

#### Arch Pkg File
You shouldn't use this method, install the estash-bin AUR pkg instead. This is here just as another means if needed.
1. Go to the [Releases Tab](https://github.com/Obscurely/EStash/releases) and download the *estash-linux.pkg.tar.zst* file.
2. From you Arch Linux command line run the following command:
```shell
sudo pacman -U estash-linux.pkg.tar.zst
```
3. Search for estash in your app launcher and launch it.
4. Might wanna take a look at the [Usage Tab](usage) if you don't understand something about it.

This method will work across any Linux distribution, Windows 10/11 and macOS (Big Sur+ tested).
1. Install rust, either using the official [rustup installer](https://www.rust-lang.org/tools/install) or any pkg manager you may use. (There is also a shell.nix file in the repo if you use nix)
2. Run the following command in your terminal of choice:
```shell
cargo install estash
```
3. Make sure you have .cargo/bin in path, for linux and macOS you would need to add the following line in your terminal RC file (e.g $HOME/.zshrc)
```shell
export PATH=$HOME/.cargo/bin:$PATH # This is for Linux & macOS, look below for Windows.
```
On windows it should work automatically (restart if just installed), if not you can follow this [guide](https://www.computerhope.com/issues/ch000549.htm) for how to add something to path. The cargo bin folder will be {your-user-folder}\\.cargo\\bin

4. You may want to create a [symlink](https://www.freecodecamp.org/news/symlink-tutorial-in-linux-how-to-create-and-remove-a-symbolic-link/) on Linux & macOS or [create a shortcut](https://support.microsoft.com/en-us/office/create-a-desktop-shortcut-for-an-office-program-or-file-9a8df64b-cd87-4700-95cc-4bc3e2a962da) if you are on Windows to the bin file for easy access.

4. In order to update run the install command again, and you can now follow [usage](#usage) for more information on how to use it.

#### Install from AUR (for Arch & Arch derivatives)
a. Using yay or any other AUR helper
  - You can install it by building from source the latest stable release
  ```shell
  yay -Sy falion
  ```
  - Or you can install the bin version so you don't have to wait for it to compile
  ```shell
  yay -Sy falion-bin
  ```
  - Or you can install it by building from source the latest git commit (that compiles and runs)
  ```shell
  yay -Sy falion-git
  ```
  
b. Manually cloning and building it from AUR
  1. First install the basic build dependencies, if you don't already have them:
  ```shell
  sudo pacman -Sy gcc base-devel --needed
  ```
  2. Then clone the build script
  ```shell
  git clone https://aur.archlinux.org/falion-bin.git
  ```
  3. Cd into the new cloned repository and run the following to build the package
  ```shell
  makepkg
  ```
  4. In order to install the package run the following (where * is just an any other characters place holder)
  ```shell
  sudo pacman -U falion-bin-*.pkg.tar.zst
  ```

#### Install from provided binaries
a. For Arch Linux based distros (not recommended, use AUR in order to have auto updates as well)
  1. Download from the [releases tab](https://github.com/Obscurely/EStash/releases/) from the version you want (latest stable recommended), the file named like falion-bin-\*.pkg.tar.zst
  2. From where you downloaded it run the following command in your terminal of choice (where * is just an any other characters place holder):
  ```shell
  sudo pacman -U falion-bin-*.pkg.tar.zst
  ```
b. For Debian based distros (I'm working on a PPA, for now I recommended you use the cargo version instead)
  1. Download from the [releases tab](https://github.com/Obscurely/falion/releases/) from the version you want (latest stable recommended), the file named like falion_\*\_debian_amd64.deb
  2. From where you downloaded it run the following command in your terminal of choice (where * is just an any other characters place holder):
  ```shell
  sudo dpkg -i falion_*_debian_amd64.deb
  ```

#### Manually
Placing the executable somewhere than adding it to path. (Not recommended, [installing it with cargo](#install-with-cargo) is better)
1. Either follow [compilation](#compilation) and build it for the platform of your choice or download from the [releases tab](https://github.com/Obscurely/EStash/releases/) the prebuilt Linux binary, called "falion"
2. Copy the falion executable to a location you want (it will have to stay there), usually in Linux you would create a folder in /opt called falion and put the executable there, or you can place anywhere else in the home dir.
3. On Linux modify your .zshrc / .bashrc / .fishrc , the hell you use, and add this line to it: (without quotation marks) "alias falion=your/path". On windows you will have to modify your path variable, here is a [guide](https://www.computerhope.com/issues/ch000549.htm). And on Mac same as Linux.
4. After you are done, you should be able to just type "falion" in terminal and you should see something pop up, saying you didn't input any query and directing you to run falion -h.

### Compilation

This program only uses cross platform libraries, but I have problems compiling it for windows from Linux, when I have time I will spin up a VM to see if it compiles in windows (on MacOS it should like 99.99% compile without problems). The following steps require that you have rust installed, check their official [installation page](https://www.rust-lang.org/tools/install).

1.  Clone this repo on your PC, you can use "git clone", if you have git installed, like this:
```shell
git clone https://github.com/Obscurely/falion.git
```
Otherwise in the right up side of the repo page you will see a download button, download the repo as zip and extract it in a folder

2.  Open a new terminal/cmd window in the folder you extracted the repo in, if you can't right click on the folder and open it there do:
```shell
cd the/path
```
and you will get there.

3.  From there run this compile command in the terminal:
```shell
cargo build --release
```
It will take a bit depending on your system because of executable size optimizations, but be patient.

4. Done, navigate to target/release and grab only the "falion" file from there. Now you can follow [manually](#manually) install

## Usage

### Basics
1. First you would have to get it installed and in path, follow [this](#getting-started), after you can continue.
2. Then from the terminal (regardless of the os) you can use it by running these commands. <div></div>
For getting help about the program
```shell
falion -h
```
For getting a list of the key binds, also available on this README at [key binds](#key-binds)
```shell
falion -k
```
For doing a search
```shell
falion rust how to print
```
Or if you want to do a search and see all the warnings (like parsing problems of text etc) run it in verbose mode
```shell
falion -v rust how to print
```

### Key binds

#### Key Binds list for falion!
**Note: where '..' is used it means from that to that like '1..5' would mean from 1 to 5.**

#### Main menu:
**[1..5]**         = Access that resource.<br />
**SHIFT + [1..5]** = Go to the next element in the list of that resource.<br />
**ALT + [1..5]**   = Go to the previous element in the list of that resource.<br />
**CTRL + n**       = Move to the next element in the list of every resource.<br />
**CTRL + b**       = Move back to the previous element in the list of every resource.<br />
**CTRL + c**       = Clear terminal and exit.<br />

#### Sub menus for the resources:
**CTRL + n**       = Move to the next element in the content list (like questions & answers).<br />
**CTRL + b**       = Move back to the previous element in the content list.<br />
**CTRL + q**       = Go back to the main menu.<br />
**CTRL + c**       = Clear terminal and exit.<br />

#### These were all the key binds, enjoy using Falion!

## Road Map

Adding more generic resources, but also maybe add lanaguage related one that get enabled based on the first word in the query. And also just improve it in general.

## Contributing

Edit a file you want, do a [pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request), I will look at it and if the change makes sense and is a good one I will accept it and that's it.

## License

Is under [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html) so stick to the license conditions and have fun :)

## Contact

Either post an issue in the [Issues Tab](https://github.com/Obscurely/falion/issues) or contact me at this email adddress if you have more to say: obscurely.social@protonmail.com

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
