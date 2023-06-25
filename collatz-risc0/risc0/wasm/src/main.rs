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

#![no_main]

use risc0_zkvm::{
    sha::{Digest, DIGEST_WORDS},
    SessionFlatReceipt, SessionReceipt
};

// This binary is here as a way to check which deps are included
// when building for the wasm32-unknown-unknown target.
// Eventually this program should run a full verify with a real receipt.

#[no_mangle]
fn _start() {
    // TODO: use a real receipt and image_id
    let receipt = SessionFlatReceipt {
        segments: Vec::new(),
        journal: Vec::new(),
    };
    let image_id = Digest::from([0; DIGEST_WORDS]);
    receipt.verify(image_id).unwrap();
}
