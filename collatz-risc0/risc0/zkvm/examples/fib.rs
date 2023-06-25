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

use std::rc::Rc;

use clap::Parser;
use risc0_zkvm::{
    prove::{default_prover, Prover},
    Executor, ExecutorEnv,
};
use risc0_zkvm_methods::FIB_ELF;
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Parser)]
#[command()]
struct Args {
    /// Number of iterations.
    #[arg(short, long)]
    iterations: u32,

    #[arg(short, long, default_value_t = false)]
    skip_prover: bool,
}

#[derive(Debug)]
#[allow(unused)]
struct Metrics {
    segments: usize,
    insn_cycles: usize,
    cycles: usize,
    seal: usize,
}

fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_forest::ForestLayer::default())
        .init();

    let args = Args::parse();
    let prover = default_prover();
    let metrics = top(prover, args.iterations, args.skip_prover);
    println!("{metrics:?}");
}

#[tracing::instrument(skip_all)]
fn top(prover: Rc<dyn Prover>, iterations: u32, skip_prover: bool) -> Metrics {
    let env = ExecutorEnv::builder()
        .add_input(&[iterations])
        .build()
        .unwrap();
    let mut exec = Executor::from_elf(env, FIB_ELF).unwrap();
    let session = exec.run().unwrap();
    let segments = session.resolve().unwrap();

    let (cycles, insn_cycles) = segments
        .iter()
        .fold((0, 0), |(cycles, insn_cycles), segment| {
            (
                cycles + (1 << segment.po2),
                insn_cycles + segment.insn_cycles,
            )
        });

    let seal = if skip_prover {
        0
    } else {
        prover.prove_session(&session).unwrap().get_seal_len()
    };

    Metrics {
        segments: segments.len(),
        insn_cycles,
        cycles,
        seal,
    }
}
