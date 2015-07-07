FROM bradrydzewski/base

ADD . /source
WORKDIR /source

ENV SHELL="/bin/sh"
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly -y --save

EXPOSE 8080
RUN rustc -V
RUN cargo build --verbose
CMD cargo run