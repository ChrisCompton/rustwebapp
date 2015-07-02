FROM geal/archlinux-rust
EXPOSE 8080
RUN cargo build
CMD cargo run