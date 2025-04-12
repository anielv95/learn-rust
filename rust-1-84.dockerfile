FROM rust:1.84-bookworm
WORKDIR /gh
RUN apt update &&\
	apt install --yes vim &&\
	apt install tree &&\
	rustup component add rustfmt
CMD ["/bin/bash"]
