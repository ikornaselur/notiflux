# Run lint and test
all: lint test

# Run `cargo build`
build:
  cargo build

# Run the webserver with auto reload on file changes
watch:
  cargo watch -x run

# Run the server in release mode
server_release:
  cargo run --release

# Run all tests
test:
  cargo test

# Lint the project with fmt and clippy
lint: fmt clippy

fmt:
  cargo fmt --all -- --check

clippy:
  cargo clippy -- -D warnings

# Build the docker container
#
# the docker container will be tagged as notiflux:dev
docker_build:
  docker build -f docker/Dockerfile -t notiflux:dev .
  
