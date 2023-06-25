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
use rand::distributions::{Distribution, Uniform};
use reqwest::{self};
use risc0_zkvm::serde::from_slice;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Journal {
    pub sequence: Vec<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub output_sequence: Vec<u32>,
    pub proof: Vec<u32>,
    pub image_id: [u32; 8],
}

const DEFAULT_API_URL: &'static str = "http://localhost:8000/public/data/actions/create";
const DEFAULT_N: i32 = 100_000_000;

fn main() {
    let n = sample_parameter(DEFAULT_N);

    let (receipt, _) = do_collatz(n);

    receipt.verify(COLLATZ_ID.into()).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    let outputs: Journal = from_slice(&receipt.get_journal()).expect("Journal didn't deserialize well.");

    let out = Output {
        output_sequence: outputs.sequence,
        proof: receipt.encode(),
        image_id: COLLATZ_ID,
    };

    let out_json = serde_json::to_string(&out).expect("Failed to serialize to JSON");
    println!("Output in JSON format: {}", out_json);

    let url = env::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());
    upload(url, out_json).expect("Failed to upload to API");
}

// Random sample from uniform distribution: integer in [1, upper_bound].
pub fn sample_parameter(upper_bound: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let rv = Uniform::new_inclusive(1, upper_bound);
    rv.sample(&mut rng)
}

fn upload(url: String, data: String) -> std::result::Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let res = client
        .post(&url)
        .body(data)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send();

    match res {
        Ok(res) => {
            println!("Status: {}", res.status());
            println!("Headers: {:?}", res.headers());
        }
        Err(_) => {}
    }
    Ok(())
}
