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
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


pub fn main() {
    // // Load the first number from the host
    // let a: u64 = env::read();
    // // Load the second number from the host
    // let b: u64 = env::read();
    // // Verify that neither of them are 1 (i.e. nontrivial factors)
    // if a == 1 || b == 1 {
        //     panic!("Trivial factors")
        // }
        // // Compute the product while being careful with integer overflow
        // let product = a.checked_mul(b).expect("Integer overflow");
        // env::commit(&product);

    // collatz sequence from a given n

    let n: u64 = env::read();
    let v : []u64 = collatz(n);
    // for now we don't check for cyclic behavior or for diverging behavior

    println!("Collatz sequence that started from n converged to 1", c);

    // TODO cylic detection
    // TODO divergence detection (through the VM crash)

    // two cases:
    // - one is a cycle that is not 1 2 4
    // - sequence diverges 
    // or something else??? cosmic rays or zk bugs otherwise 
    // 

    // end is good mention 


    env::commit(&v);
}

pub fn collatz(num: usize) -> usize {
    let (mut num, mut count) = (num, 0);
    while num != 1 {
        (num, count) = match num {
            num if num % 2 == 0 => (num / 2, count + 1)jj,
            _ => ((3 * num + 1) / 2, count + 2), // +2 accounts for skipped step
        };
    }
    count
}