# Universal macOS Rust apps test

This is a sample repo of building a Rust app to work both on Intel & Apple Silicon macs using GitHub Actions as a build platform. The core functionality is building for both the `x86_64-apple-darwin` and `aarch64-apple-darwin` targets and then combining both binaries into one using the `lipo` mac utility which creates single binaries that can run on multiple architectures. To see how this is done, visit the `.github/workflows/build.yml` file.
