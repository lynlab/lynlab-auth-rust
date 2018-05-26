FROM rust:1.26

WORKDIR /usr/src/app
COPY . .

RUN cargo install
RUN cargo install diesel_cli

ENTRYPOINT [ "./docker-entrypoint.sh" ]
CMD [ "lynlab-auth" ]
