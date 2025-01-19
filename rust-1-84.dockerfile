FROM rust:1.84-bookworm
WORKDIR /gh
RUN apt update &&\
	apt install --yes vim
CMD ["/bin/bash"]
