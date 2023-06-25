SHELL := /bin/bash

docker_image = collaptz-api
docker_username = adamcuculich
formatted_code := app/ migrations/
rev_id = ""
migration_message = ""

.ONESHELL:

.PHONY: test run

requirements.txt: requirements.in
	pip-compile --quiet --generate-hashes ---unsafe --resolver=backtracking --output-file=$@

format:
	isort $(formatted_code)
	black $(formatted_code)

lint:
	pylint app

run:
	python -m app --reload

make run-container:
	docker-compose up -d

build:
	docker build -t $(docker_username)/$(docker_image):latest .

push:
	docker push $(docker_username)/$(docker_image):latest

test: build
	function removeContainers {
		docker-compose -p collaptz-api-continuous-integration rm -s -f test_db
	}
	trap removeContainers EXIT
	docker-compose -p collaptz-api-continuous-integration run --rm continuous-integration


migration:
	if [ -z $(rev_id)] || [ -z $(migration_message)]; \
	then \
		echo -e "\n\nmake migration requires both a rev_id and a migration_message.\nExample usage: make migration rev_id=0001 migration_message=\"my message\"\n\n"; \
	else \
		alembic revision --autogenerate --rev-id "$(rev_id)" -m "$(migration_message)"; \
	fi

migrate:
	alembic upgrade head
