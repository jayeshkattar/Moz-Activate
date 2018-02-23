FROM krishnakumar4a4/rust-nightly:1.0
COPY * /rust-irc-server/
COPY src/* /rust-irc-server/src/
WORKDIR /rust-irc-server/
EXPOSE 80
ENTRYPOINT cargo run
