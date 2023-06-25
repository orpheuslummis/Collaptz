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
use collatz_methods::COLLATZ_ELF;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv, SessionReceipt,
};

// #[doc = include_str!("../README.md")]

pub fn do_collatz(n: i32) -> (Box<dyn SessionReceipt>, i32) {
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&n).unwrap())
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, COLLATZ_ELF).unwrap();

    let session = exec.run().unwrap();

    let receipt = session.prove().unwrap();

    // TBD
    let c: i32 = from_slice(receipt.get_journal()).expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    (receipt, c)
}

// #[cfg(test)]
// mod tests {
//
// use super::*;
//
// #[test]
// fn test_factors() {
// const TEST_FACTOR_ONE: u64 = 17;
// const TEST_FACTOR_TWO: u64 = 23;
// let (_, result) = multiply_factors(17, 23);
// assert_eq!(
// result,
// TEST_FACTOR_ONE * TEST_FACTOR_TWO,
// "We expect the zkVM output to be the product of the inputs"
// )
// }
// }
