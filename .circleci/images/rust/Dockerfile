FROM circleci/rust:stretch

USER root

RUN rustup update && \
	rustup install stable && \
	rustup default stable

RUN apt-get update && \
	apt-get -y install \
		build-essential \
		libncurses-dev \
		libssl-dev \
		pkgconf

RUN cargo install --force cargo-deb

WORKDIR /app
