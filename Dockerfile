FROM rust AS files
WORKDIR /usr/src/angybot
COPY . .

FROM files AS system
RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y libopus-dev

FROM system AS ytdlp
RUN wget "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux" --output-document=/usr/bin/yt-dlp
RUN chmod a+x /usr/bin/yt-dlp

FROM ytdlp AS install
RUN cargo install --path .

FROM install AS environment
ENV RUST_LOG="angybot"

FROM environment AS entrypoint
ENTRYPOINT [ "angybot" ]
CMD []
