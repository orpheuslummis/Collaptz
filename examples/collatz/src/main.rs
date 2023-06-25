// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;

use collatz::do_collatz;
use collatz_methods::COLLATZ_ID;
use futures::executor::block_on;
use rand::distributions::{Distribution, Uniform};
use reqwest;
use risc0_zkvm::serde::from_slice;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Journal {
    pub sequence: Vec<u32>, // Rust question or a Vec
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub sequence: Vec<u32>,
    pub receipt: Vec<u8>,
    pub program_digest: [u32; 8],
}

const DEFAULT_API_URL: &'static str = "http://localhost:8000";

// #[derive(Serialize)]
// struct Output {
//     sequence: Vec<i32>,
//     proof: Vec<u8>,
//     program_digest: String,
// }

fn main() {
    let n = sample_parameter(100000000); // WIP

    let (receipt, _) = do_collatz(n);

    // send your binary
    // send your input??
    // receive proof and output 

    // Just in case!
    receipt.verify(COLLATZ_ID.into()).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    // store_to_file(receipt.encode());

    // extract/gather our data
    println!("DEBUG: the COLLATZ_ID is {{{:?}}}", COLLATZ_ID);
    // let vec = receipt.get_journal();
    let outputs: Journal =
        from_slice(&receipt.get_journal()).expect("Journal didn't deserialize well.");

    let out = Output {
        sequence: outputs.sequence,
        receipt: receipt.encode(),
        program_digest: COLLATZ_ID,
    };

    let out_json = serde_json::to_string(&out).expect("Failed to serialize to JSON");
    // question: how is the serialization of the byte array in json?
    println!("Output in JSON format: {}", out_json);

    let url = env::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());
    upload(url, out_json).expect("Failed to upload to API");
}

// Random sample from uniform distribution: integer in [1, upper_bound].
pub fn sample_parameter(upper_bound: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let rv = Uniform::new_inclusive(1, upper_bound);
    rv.sample(&mut rng)
}

fn upload(url: String, data: String) -> std::result::Result<(), std::io::Error> {
    let client = reqwest::Client::new();
    let res = block_on(
        client
            .post(url)
            .body(data)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send(),
    )
    .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;
    Ok(())
}

// /// Upload body to a given URL
// fn put_data<T: Into<reqwest::blocking::Body>>(&self, url: &str, body: T) ->
// Result<()> {     let res = self
//         .client
//         .put(url)
//         .body(body)
//         .send()
//         .context("Failed to PUT data to destination")?;
//     if !res.status().is_success() {
//         bail!("Failed to PUT to provided URL");
//     }

//     Ok(())
// }

// fn store_to_file(receipt_bytes: Vec<u8>, file_path: &str) -> Result<()> {
//     let mut file = File::create(file_path)?;

//     file.write_all(&receipt_bytes)?;

//     // It's a good practice to flush any buffered data to the underlying
// medium     file.flush()?;

//     Ok(())
// }
