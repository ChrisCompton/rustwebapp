# rustwebapp

[![wercker status](https://app.wercker.com/status/35093a002c9d8b82da010d7c0b3c772d/m "wercker status")](https://app.wercker.com/project/bykey/35093a002c9d8b82da010d7c0b3c772d)

Crates used:
- https://github.com/iron/iron
- https://github.com/iron/persistent
- https://github.com/iron/router
- https://github.com/iron/mount
- https://github.com/iron/staticfile
- https://github.com/sunng87/handlebars-iron
- https://github.com/rust-lang/rustc-serialize
- https://github.com/sfackler/r2d2-postgres
- https://github.com/sfackler/rust-postgres

### Postgresql Docker Commands

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

### Rustwebapp Docker Commands

````make
build:
	docker build -t=rustwebapp .

run:
	docker run --name=rustwebapp --rm=true -i -t -p 8080:8080 -e DATABASE_URL=postgres://dbuser:dbpass@dbname:5432/test --link=postgresql:dbname rustwebapp

stop:
	docker stop rustwebapp

start:
	docker start rustwebapp
````
