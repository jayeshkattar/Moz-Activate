FROM krishnakumar4a4/rust-nightly:1.0
COPY * /rust-irc/
COPY src/* /rust-irc/src/
WORKDIR /rust-irc/
EXPOSE 80
ENTRYPOINT cargo run
