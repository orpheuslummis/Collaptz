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

use std::{
    fs::File,
    io::{prelude::*, Result},
};

use factors::multiply_factors;
use factors_methods::MULTIPLY_ID;

fn main() {
    // Pick two numbers
    let (receipt, _) = multiply_factors(17, 23);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(MULTIPLY_ID.into()).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    store_to_file(receipt.encode());

    // TODO then will upload via HTTP POST to API
    
}

fn store_to_file(receipt_bytes: Vec<u8>) -> Result<()> {
    // Define the path of the file
    let path = "path_to_your_file";

    // Open a file in write-only mode
    let mut file = File::create(path)?;

    // Write the Vec<u8> data into the file
    file.write_all(&receipt_bytes)?;

    // It's a good practice to flush any buffered data to the underlying medium
    file.flush()?;

    Ok(())
}
