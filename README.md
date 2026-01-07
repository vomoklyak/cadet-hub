# Cadet-Hub

## Introduction

**Cadet Hub** cross-platform desktop application for management cadet course registry<br><br>
**Stack**: Rust, Dioxus, Tokio, Sqlite<br>
**Test Stack**: Rust, Sqlite, Mockall, Spectral<br>

## Build and Test

**To build project for Ubuntu**:<br>

* enter project root:<br>
  `cadet-hub`<br>
* run:<br>
  `dx build --release --platform desktop --target x86_64-unknown-linux-gnu && cargo deb`<br>
* deb artifact:<br>
  `target/debian/cadet-hub_1.0.0-1_amd64.deb`

**To build project for Windows**:<br>

* enter project root:<br>
  `cadet-hub`<br>
* run:<br>
  `dx build --release --platform desktop --target x86_64-pc-windows-gnu`<br>
* execution directory layout for Windows (css hashes are build specific):<br>
  `assets/main-dxh5237.css (copy target/dx/frontend/release/linux/app/assets/main-dxh5237.css)`<br>
  `assets/tailwind-dxh64e.css (copy target/dx/frontend/release/linux/app/assets/tailwind-dxh64e.css)`<br>
  `configs/application.yaml (copy configs/application.yaml)`<br>
  `frontend.exe (copy target/x86_64-pc-windows-gnu/desktop-release/frontend.exe)`<br>
  `WebView2Loader.dll (copy crates/frontend/os_windows/WebView2Loader.dll)`<br>

**To run test suite**:<br>
`cargo test --workspace`<br>

## Run

**To run application locally**:

* enter project root:<br>
  `cadet-hub`
* update if required local config properties:
  `configs/application-local.yaml`
* run (from IDE in run or debug mode):<br>
  `crates/frontend/src/main.rs`