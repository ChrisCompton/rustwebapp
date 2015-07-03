FROM geal/archlinux-rust

ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib

ADD . /source
WORKDIR /source

EXPOSE 8080
RUN rustc -V
RUN cargo build
CMD cargo run