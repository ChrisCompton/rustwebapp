FROM adreeve/drone-rust

ADD . /source
WORKDIR /source

USER root

EXPOSE 8080
RUN rustc -V
RUN cargo build
CMD cargo run