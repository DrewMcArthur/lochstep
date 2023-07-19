FROM rust:1.71 as build

# create a new empty shell project
RUN USER=root cargo new --bin lochstep
WORKDIR /lochstep

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/lochstep*
RUN cargo build --release

# our final base
FROM rust:1.71-slim

# install sqlite lib
RUN apt-get update && apt-get install sqlite3 -y

# copy the build artifact from the build stage
COPY --from=build /lochstep/target/release/lochstep .
COPY --from=build /lochstep/src/ui /src/ui

# set the startup command to run your binary
CMD ["./lochstep"]
