all: build

build:
	docker build --tag="jakescott/rustwebapp" .

#create: 
#	docker run --name="rustwebapp" -p 8080:8080 -d -P jakescott/rustwebapp

create: 
	docker run --rm --name="rustwebapp" jakescott/rustwebapp




stop:
	docker stop rustwebapp

start:
	docker start rustwebapp

remove:
	docker rm rustwebapp