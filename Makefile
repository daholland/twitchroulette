start-db:
	docker run -d \
		--name psql \
		-e POSTGRES_PASSWORD=password \
		-v twitch-psql:/var/lib/postgresql/data \
		-p 5432:5432 \
		postgres

start-redis:
	docker run -d \
		--name redis \
		-v twitch-redis:/data \
		-p 6379:6379 \
		redis \
		redis-server --appendonly yes
