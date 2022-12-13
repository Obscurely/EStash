pipeline {
  // this pipeline uses the docker plugin
  agent {
    docker {
      image 'rust:latest'
    }
  }

  stages {
    stage('UpdateData') {
        steps {
            // make the workspace fresh
            sh "rm -rf ./*"
            sh "rm -rf .git .gitignore*"
            sh "git clone https://www.github.com/Obscurely/EStash ."
        }
    }
    stage('Prepare') {
        steps {
            // download anything needed
            // get everything needed installed
            sh "apt update"
            sh "apt install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev libfuse2 -y"
            sh "cargo install cargo-deb"
        }
    }
    stage('Build') {
      steps {
        // build general bin and deb pkg
        sh "cargo build --release" // build classic bin package
        sh "cargo deb" // build deb package
      }
    }
    stage('AppImage') {
        steps {
            // create app-image
            sh "cp target/release/estash linux/EStash.AppDir/usr/bin/estash"
            sh "cp LICENSE linux/EStash.AppDir/LICENSE"
            sh "chmod +x linux/EStash.AppDir/usr/bin/estash"
            sh "wget -c \"https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage\" -O appimagetool-x86_64.AppImage"
            sh "chmod a+x appimagetool-x86_64.AppImage"
            sh "./appimagetool-x86_64.AppImage --appimage-extract" // extract the appimage since we can't use fuse in docker
            sh "ARCH=x86_64 ./squashfs-root/AppRun linux/EStash.AppDir"
        }
    }
    stage('Tar') {
        steps {
            // Create a tar archive with the app for the -bin aur pkg
            sh "cp target/release/estash ./estash"
            sh "tar -czf estash-linux.tar.gz estash LICENSE README.md linux/desktop"
            sh "sha256sum estash-linux.tar.gz | sed -e 's/  estash-linux.tar.gz//g' > tar_sha"
        }
    }
    stage('PKGBUILDs') {
        steps {
            // Create the pkgbuilds files in order to update the aur pkgs
            sh "git tag | sort -V | tail -1 | sed -e 's/v//g' | sed -e 's/-stable//g' > pkgver"
            sh "git describe --tags | sed -e 's/-stable//' | sed -e 's/v//' | sed -e 's/-/./g' > pkgver_git"
            // replace plaholder for pkg version
            sh "sed -i \"s/PKGVER-PLACEHOLDER/\$(cat pkgver)/g\" linux/AUR/bin/PKGBUILD"
            sh "sed -i \"s/PKGVER-PLACEHOLDER/\$(cat pkgver)/g\" linux/AUR/bin/.SRCINFO"
            sh "sed -i \"s/PKGVER-PLACEHOLDER/\$(cat pkgver_git)/g\" linux/AUR/git/PKGBUILD"
            sh "sed -i \"s/PKGVER-PLACEHOLDER/\$(cat pkgver_git)/g\" linux/AUR/git/.SRCINFO"
            sh "sed -i \"s/PKGVER-PLACEHOLDER/\$(cat pkgver)/g\" linux/AUR/stable/PKGBUILD"
            sh "sed -i \"s/PKGVER-PLACEHOLDER/\$(cat pkgver)/g\" linux/AUR/stable/.SRCINFO"
            // replace sha placeholder for bin
            sh "sed -i \"s/SHA-PLACEHOLDER/\$(cat tar_sha)/g\" linux/AUR/bin/PKGBUILD"
            sh "sed -i \"s/SHA-PLACEHOLDER/\$(cat tar_sha)/g\" linux/AUR/bin/.SRCINFO"
        }
    }
    stage('GatherBuilds') {
        steps {
            // gather all the builds in a single folder
            sh "mkdir Builds"
            sh "cp target/release/estash Builds/estash-linux"
            sh "cp target/debian/estash_* Builds/estash-linux.deb"
            sh "cp EStash-x86_64.AppImage Builds/estash-linux.AppImage"
            sh "cp estash-linux.tar.gz Builds/estash-linux.tar.gz"
        }
    }
    stage('CreateNIX') {
        steps {
            // Make the required changes to the .nix file
            sh "sha256sum Builds/estash-linux.AppImage | sed -e 's/  Builds\\/estash-linux.AppImage//g' > appimage_sha"
            sh "sed -i \"s/VERSION_PLACEHOLDER/\$(cat pkgver)/\" linux/estash.nix"
            sh "sed -i \"s/SHA_PLACEHOLDER/\$(cat appimage_sha)/\" linux/estash.nix"
            sh "cp linux/estash.nix Builds/estash-linux.nix"
        }
    }
    stage('GatherPKGBUILDs') {
        steps {
            // Gather the PKGBUILDs files
            sh "mkdir PKGBUILDs"
            sh "cp -r linux/AUR/* PKGBUILDs/."
        }
    }
  }
}
