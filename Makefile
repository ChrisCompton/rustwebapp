run:
	cargo run

build:
	cargo build

clean:
	cargo clean

build-c:
	docker build -t bradrydzewski/base .

run-c:
	docker run -p 8080:8080 -d -P geal/archlinux-rust

push:
	git push origin master

deploy:
	git push deis master