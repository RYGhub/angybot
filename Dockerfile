FROM rust AS files
WORKDIR /usr/src/bobbot
COPY . .

FROM files AS install
RUN cargo install --path .

FROM install AS environment
ENV RUST_LOG="angybot"

FROM environment AS entrypoint
ENTRYPOINT [ "angybot" ]
CMD []
