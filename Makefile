build:
	docker build -t=rustwebapp .

run:
	docker run --name=rustwebapp --rm=true -i -t -p 8080:8080 \
	-e DATABASE_URL=postgres://dbuser:dbpass@dbname:5432/test \
	--link=postgresql:dbname rustwebapp

stop:
	docker stop rustwebapp

start:
	docker start rustwebapp

