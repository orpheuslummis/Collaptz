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

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let n: i32 = env::read();

    let seq = collatz(n);

    env::commit(&seq);
}

fn collatz(mut n: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    output.push(n);

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }

        output.push(n);
    }

    // TODO cylic detection
    // TODO divergence detection (through the VM crash)

    // two cases:
    // - one is a cycle that is not 1 2 4
    // - sequence diverges
    // or something else??? cosmic rays or zk bugs otherwise
    //

    output
}
