# Collaptz

**Collaptz** is an open computational task, a collaborative verifiable system, to experiment the Collatz conjecture.

The computational integrity we obtain from running part of the computation in a zkVM allows to trustlessly distribute the computation, thereby allowing contributions from public/untrusted participants.


### Computational task: Build a dataset of Collatz sequences to analyse the conjecture

A Collatz sequence begins with a positive integer. The sequence follows two simple rules: if the current number is even, it is divided by two; if it is odd, it's tripled and increased by one.

The **Collatz conjecture** proposes that no matter the starting integer, the sequence will inevitably reach one.

**Why?**

While the conjecture has been heavily studied, sequences have explored up to $2^{68}$ as of mid-2023. (https://link.springer.com/article/10.1007/s11227-020-03368-x)

Useful patterns and surprises might lie within this computation.


## Contribute to the public dataset of Collatz sequences

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


## "The Collatz Thinker" NFT
A NFT of the resulting public computation will be offered in auction late 2023.

