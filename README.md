# Collaptz

TODO

## Compute a Collatz sequence and contribute it to the public data pool

```shell
cd collatz-risc0/examples/collatz/
cargo run
```

## Start the HTTP API

```shell
cd collatz-api
# python -m venv .venv
source .venv/bin/activate
pip install pip-tools alembic
pip install -r requirements.txt 
ensure docker is installed
create .env file with db credentials
docker-compose up -d db
make migration
make migrate
make run 
```