pipeline {
  // this pipline uses the docker plugin
  agent {
    docker {
      image 'archlinux:base-devel'
    }
  }

  stages {
    stage('Prepare') {
        steps {
            // get anything needed ready
            // git
            sh "pacman -Sy git --needed --noconfirm"
            // make depends
            sh "pacman -S rust cargo binutils libx11 libxext libxft libxinerama libxcursor libxrender libxfixes pango cairo libgl mesa --needed --noconfirm"
            // create a user to use with makepkg
            sh "useradd -m -G wheel -s /bin/bash ouruser "
        }
    }
    stage('UpdateData') {
        steps {
            // make the workspace fresh
            sh "rm -rf ./*"
            sh "rm -rf .git .gitignore*"
            sh "git clone https://www.github.com/Obscurely/EStash ."
        }
    }
    stage('PKGBUILDs') {
        steps {
            // create the pkgbuild files
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
            sh "sed -i \"s/SHA-PLACEHOLDER/SKIP/g\" linux/AUR/bin/PKGBUILD"
            sh "sed -i \"s/SHA-PLACEHOLDER/SKIP/g\" linux/AUR/bin/.SRCINFO"
        }
    }
    stage('Prepare MakePKG') {
        steps {
            // prepare for making the pkgs
            // change permissions
            dir('linux/AUR/') {
                sh "chmod 777 bin/"
                sh "chmod 777 stable/"
                sh "chmod 777 git/"
            }
        }
    }
    stage('MakePKG') {
        // Make all the pkgs (stable, bin & git) for testing
        parallel {
            stage('Bin Package') {
                steps {
                    // package bin
                    dir('linux/AUR/bin/') {
                        sh "su ouruser -c makepkg"
                    }
                }
            }
            stage('Stable Package') {
                steps {
                    // package stable
                    dir('linux/AUR/stable/') {
                        sh "su ouruser -c makepkg"
                    }
                }
            }
            stage('Git Package') {
                steps {
                    // package git
                    dir('linux/AUR/git/') {
                        sh "su ouruser -c makepkg"
                    }
                }
            }
        }
    }
    stage('GatherBuilds') {
        // gather the bin build in order the upload it to the github releases
        steps {
            sh "mkdir Builds"
            sh "cp linux/AUR/stable/estash*.pkg.tar.zst Builds/estash-linux.pkg.tar.zst"
        }
    }
  }
}
