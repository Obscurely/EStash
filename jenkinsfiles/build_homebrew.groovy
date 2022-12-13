pipeline {
  // this pipeline uses the docker plugin
  agent {
    docker {
      image 'alpine:latest'
    }
  }

  stages {
      stage('Clean') {
          steps {
              // clear the workspace
              sh "rm -rf *"
          }
      }
      stage('Prepare') {
          steps {
              // get everything ready (pkgs)
              sh "apk update"
              sh "apk add git wget"
          }
      }
      stage('GetCurrentVersion') {
          steps {
              // get current version
              sh "git clone https://github.com/Obscurely/EStash"
              dir('EStash') {
                  sh "git tag | sort -V | tail -1 | sed -e 's/v//g' | sed -e 's/-stable//g' > pkgver"
              }
              
              // download latest macos bin
              sh "wget \"https://github.com/Obscurely/EStash/releases/download/v\$(cat EStash/pkgver)-stable/estash-macos\""
              sh "mv estash-macos estash"
              
              // copy readme and license files
              sh "cp EStash/README.md README.md"
              sh "cp EStash/LICENSE LICENSE"
              
              // pkg the bin
              sh "tar -czf estash-macos.tar.gz estash LICENSE README.md"
              
              // make the brew formula
              sh "sha256sum estash-macos.tar.gz | sed -e 's/  estash-macos.tar.gz//g' > tar_sha"
              sh "sed -i \"s/VERSION_PLACEHOLDER/\$(cat EStash/pkgver)/g\" EStash/macos/estash.rb"
              sh "sed -i \"s/SHA_PLACEHOLDER/\$(cat tar_sha)/g\" EStash/macos/estash.rb"
              
          }
      }
      stage('GatherBuilds') {
          // gather all the builds in one directory
          steps {
              // create builds directory
              sh "mkdir Builds"
              
              // copy over files
              sh "cp EStash/macos/estash.rb Builds/estash.rb"
              sh "cp estash-macos.tar.gz Builds/estash-macos.tar.gz"
          }
      }
  }
}
