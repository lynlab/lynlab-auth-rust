FROM rust:1.26

WORKDIR /usr/src/app
COPY . .

RUN cargo install

ENTRYPOINT [ "./docker-entrypoint.sh" ]
CMD [ "lynlab-auth" ]
