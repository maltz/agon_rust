FROM rust:1.26.2

# copy your source tree
COPY ./ ./

# build for release
RUN cargo build --release

# set the startup command to run your binary
CMD ["./target/release/agon-rust"]

# run
# docker run -it --rm -p 8080:8080 agon-rust