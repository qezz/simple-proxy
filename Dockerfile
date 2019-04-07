FROM ekidd/rust-musl-builder
WORKDIR /app

RUN sudo mkdir /build
RUN sudo chown -R rust:rust /build

COPY ./ ./

RUN sudo chown -R rust:rust ./

CMD ["/bin/bash", "./build.sh"]
