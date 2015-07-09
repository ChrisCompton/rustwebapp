FROM adreeve/rust-nightly:2015-07-02

ADD . /source
WORKDIR /source

EXPOSE 8080

RUN rustc -V
RUN cargo -V
RUN cargo build

CMD cargo run