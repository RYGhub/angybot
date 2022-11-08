FROM rust:buster AS files
WORKDIR /usr/src/angybot
COPY . .

FROM files AS ytdlp
RUN wget "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux" --output-file=/usr/bin/yt-dlp
RUN chmod a+x /usr/bin/yt-dlp

FROM ytdlp AS install
RUN cargo install --path .

FROM install AS environment
ENV RUST_LOG="angybot"

FROM environment AS entrypoint
ENTRYPOINT [ "angybot" ]
CMD []
