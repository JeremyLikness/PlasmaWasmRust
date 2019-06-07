# Plasma WASM Rust

[![Build Status](https://jeremylikness.visualstudio.com/PlasmaWasmRust/_apis/build/status/JeremyLikness.PlasmaWasmRust?branchName=master)](https://jeremylikness.visualstudio.com/PlasmaWasmRust/_build/latest?definitionId=12&branchName=master)

![Deployment Status](https://jeremylikness.vsrm.visualstudio.com/_apis/public/Release/badge/0761d6ee-ac22-48b2-8a85-d357b62f4a39/1/1)

This is a port of my `Go` implementation of plasma to Rust. It uses `wasm-pack-template`. Inspect the Azure pipelines definition for build details.

👀 [Live Demo](https://jlikme.z13.web.core.windows.net/wasm/PlasmaWasmRust)

For a full walkthrough, read: [Plasma gets Rust-y: Another WebAssembly Experiment](https://blog.jeremylikness.com/plasma-gets-rust-y-another-webassembly-experiment-bde6abf3061c).

> This repository is continuously built and deployed using free Azure Pipelines. If you're interested in how it was setup and configured to build automatically and deploy to low cost Azure Storage Static Websites, read [Deploy WebAssembly from GitHub to Azure Storage Static Websites with Azure Pipelines](https://jlik.me/fzk).
