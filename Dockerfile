FROM rust:1.47 as build

# create a new empty shell project
WORKDIR /usr/src/sushii-site2
RUN USER=root cargo init --bin

# copy over manifests
COPY ./Cargo.lock ./Cargo.toml ./

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree and files
COPY ./src ./src
COPY ./static ./static

# build for release, remove dummy compiled files
RUN rm ./target/release/deps/sushii_site2*

RUN cargo build --release

## Final base image with only the picatch binary
FROM debian:buster-slim

WORKDIR /config

# Copy binary
COPY --from=build /usr/src/sushii-site2/target/release/sushii-site2 /usr/local/bin/sushii-site2

RUN mkdir ./src
# CSS files are built from src/styles
COPY --from=build /usr/src/sushii-site2/src/styles ./src/styles
# Static files + destination for built CSS files
COPY --from=build /usr/src/sushii-site2/static ./static

# Templates are read from file during runtime
COPY ./templates ./templates
# Configurations needs to be in pwd during runtimme
COPY ./Rocket.toml ./Commands.yml ./

EXPOSE 8000

ENTRYPOINT ["sushii-site2"]
