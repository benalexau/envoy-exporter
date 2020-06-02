FROM rust as builder
WORKDIR /usr/src/envoy-exporter
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y libcurl4 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/envoy-exporter/target/release/envoy-exporter /usr/local/bin/envoy-exporter
ENTRYPOINT [ "/usr/local/bin/envoy-exporter" ]
CMD [ "/config.toml" ]