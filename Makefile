.PHONY : build cargo explain fmt package release run start stop
.DEFAULT_GOAL := start

local_path := $(shell pwd)

project_name := muxed

repo_name := brianp/${project_name}:${MUXED_ENV}

docker_instance_name := ${project_name}.${MUXED_ENV}.container

docker_exec := docker exec ${docker_instance_name}

ifeq (${MUXED_ENV}, osx)
	target := x86_64-apple-darwin
else
	target := x86_64-unknown-linux-gnu
endif

build:
	docker build -t ${repo_name} -f ${MUXED_ENV}.dockerfile .

cargo:
	${docker_exec} cargo ${cmd} --target ${target}

explain:
	${docker_exec} rustc --explain ${err}

fmt:
	${docker_exec} cargo fix -Z unstable-options --clippy --target ${target} --allow-dirty; \
	${docker_exec} cargo fmt

package:
	${docker_exec} tar -cvzf muxed-$(shell git tag  | grep -E '^[0-9]' | sort -V | tail -1)-${target}.tar.gz -C ./target/${target}/release/ muxed

release:
	${docker_exec} cargo build --release --target ${target}

run:
	${docker_exec} ${cmd}

start:
	docker run -d -it -v "${local_path}:/usr/src/" --name ${docker_instance_name} --rm ${repo_name}

stop:
	docker stop ${docker_instance_name}

help:
	@echo build: build docker image
	@echo cargo: run cargo commands inside development container
	@echo explain: use the rustc --explain command
	@echo fmt: run cargo clippy and rustfmt
	@echo package: bundle releases into compressed packages
	@echo release: build release bin
	@echo run: run any command inside the development container
	@echo start: run the docker development container
	@echo stop: stop the running development container
