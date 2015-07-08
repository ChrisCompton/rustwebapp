# rustwebapp

Uses the following crates
- https://github.com/iron/iron
- https://github.com/iron/persistent
- https://github.com/iron/router
- https://github.com/sfackler/r2d2-postgres
- https://github.com/sfackler/rust-postgres

## Install Postgresql

I recommend using this Docker image https://github.com/sameersbn/docker-postgresql and using a Makefile like this.

````make
all: build

build:
	@docker build --tag=${USER}/postgresql .

remove:
	docker rm postgresql

create: 
	docker run --name postgresql -d \
	-e 'DB_USER=dbuser' \
	-e 'DB_PASS=dbpass' \
	-e 'DB_NAME=dbname' \
	-e 'PSQL_TRUST_LOCALNET=true' \
	-v /opt/postgresql/data:/var/lib/postgresql \
	-p 5432:5432 \
	sameersbn/postgresql:9.4-1

stop:
	docker stop postgresql

start:
	docker start postgresql

connect:
	docker exec -it postgresql sudo -u postgres psql

.PHONY: start


````
