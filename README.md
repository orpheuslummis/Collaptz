# Collaptz

# [Collaptz | ETHGlobal Waterloo 2023 - Winner of 🥈 Risc Zero — Best Use of Bonsai](https://ethglobal.com/showcase/collaptz-623pz)

**Collaptz** is an open computational task, a collaborative verifiable system, to empirically verify the Collatz conjecture.

The computational integrity we obtain from running part of the computation in a zkVM allows to trustlessly distribute the computation, thereby allowing contributions from public/untrusted participants.


### Computational task: Build a dataset of Collatz sequences to analyse the conjecture

A Collatz sequence begins with a positive integer. The sequence follows two simple rules: if the current number is even, it is divided by two; if it is odd, it's tripled and increased by one.

The **Collatz conjecture** proposes that no matter the starting integer, the sequence will inevitably reach one.

**Why?**

While the conjecture has been heavily studied, sequences have explored up to $2^{68}$ as of mid-2023. (https://link.springer.com/article/10.1007/s11227-020-03368-x)

Useful patterns and surprises might lie within this computation.


## Contribute to the public dataset of Collatz sequences!

---

# Architecture

## Storage layer (accessible via HTTP API)

### Install

```shell
cd collatz-api
# python -m venv .venv  # create a virtual environment
source .venv/bin/activate
pip install pip-tools alembic
pip install -r requirements.txt 
```

Also ensure that docker and docker-compose are installed.

### Run the API

Create .env file with full DB_URL

```shell
docker-compose up -d db
make migration
make migrate
make run
```

## ZK computer

Compile the Rust program on risc0 zkVM, run it and generate the proof of computational integrity.
After the computation is complete, it will be uploaded to the storage layer via HTTP API.

```shell
cd collatz-risc0/examples/collatz/
cargo run
```


## Explore the data


### Install the plotly dash frontend

```shell
pip install -e . 
````

Spin up a Frontend for live visualization of the newly computed sequences.

```shell
python frontend/dashboard.py
```

Check out the following pages for even cooler visualisations of Collatz sequences:

 - [Overlayed Line Plots](https://tools.opencurve.info/collatzconjecture/collatzcollection.html)
 - [Harriss Plot](https://tools.opencurve.info/collatzconjecture/collatz-harriss.html#graph)

---

## "The Collatz Thinker" NFT
A NFT of the resulting public computation will be offered in auction late 2023.
You can think of a progressive sequence generation as a form of generative art!

