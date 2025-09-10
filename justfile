cargo := require("cargo")

# install rustup
[linux]
[macos]
deps:
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install rustup
[windows]
deps:
  winget install --id Rustlang.Rustup -e

build:
  cargo build

run:
  cargo run

fmt:
  cargo fix
  cargo fmt

test:
  cargo test

# =======================================

install:
  cargo install --path .

post-commit: 
  just --quiet install

doc:
  cd docs && pnpm install && pnpm run docs:dev

stat:
  cd src && tokei .
