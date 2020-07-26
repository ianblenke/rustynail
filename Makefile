all:
	# Force a local refresh of upstream base images
	docker pull rust:latest
	docker pull alpine:latest
	# Build and restart everything
	docker-compose build
	docker-compose up -d --force-recreate
	docker-compose logs -f
