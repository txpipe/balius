FROM debian:12-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

ARG TARGETARCH
COPY bin/baliusd-Linux-${TARGETARCH} /bin/baliusd
RUN chmod +x /bin/baliusd
RUN ln -s /bin/baliusd /baliusd

ENTRYPOINT [ "baliusd" ]
