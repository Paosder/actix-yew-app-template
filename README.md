# actix-yew-app-template

Rust based full stack web application template.

## backend

actix_web (rust)

## frontend

Frontend Framework: yew (rust)
CSS Framework: tailwindcss

## Prerequisites

- Rust 1.50 or above
- Node 14 or above
- Yarn 1.22 or above

This repository uses yew, the Rust based WASM framework, so you have to install wasm-related crate.

```shell
> rustup target add wasm32-unknown-unknown
> cargo install wasm-bindgen-cli
```

Or just execute `bootstrap.sh`.

## Install

```shell
> yarn && yarn bootstrap
```

## Development

```shell
> yarn dev
```

## Build

```shell
> yarn build
```

The result directory is `dist`, execute `backend-main` to run server.
