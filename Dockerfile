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
COPY ./templates ./templates
COPY ./Commands.toml ./

# build for release, remove dummy compiled files
RUN rm ./target/release/deps/sushii_site2*

RUN cargo build --release

## Final base image with only the picatch binary
FROM debian:buster-slim

WORKDIR /config
COPY --from=build /usr/src/sushii-site2/target/release/sushii-site2 /usr/local/bin/sushii-site2

EXPOSE 8000

ENTRYPOINT ["sushii-site2"]
