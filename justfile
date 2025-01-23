[working-directory: 'styles']
build-styles:
  tailwindcss -i ./base.css -o ../static/styles.css

build-rust:
  cargo build

build:
  @just build-styles
  @just build-rust

run:
  @just build-styles
  cargo run

test:
  cargo test
