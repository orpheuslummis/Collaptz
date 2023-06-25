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

use std::{process::Command, rc::Rc, time::Instant};

use clap::Parser;
use human_repr::{HumanCount, HumanDuration};
use risc0_zkvm::{
    prove::{default_prover, Prover},
    receipt::SessionReceipt,
    serde::to_vec,
    Executor, ExecutorEnv, Session,
};
use risc0_zkvm_methods::{
    bench::{BenchmarkSpec, SpecWithIters},
    BENCH_ELF,
};
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Parser)]
struct Args {
    /// Number of iterations.
    #[arg(long, short)]
    iterations: Option<u64>,

    #[arg(long, short)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();
    if let Some(iterations) = args.iterations {
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(tracing_forest::ForestLayer::default())
            .init();

        let prover = default_prover();

        let start = Instant::now();
        let (session, receipt) = top(prover.clone(), iterations);
        let segments = session.resolve().unwrap();
        let duration = start.elapsed();

        let cycles = segments
            .iter()
            .fold(0, |acc, segment| acc + (1 << segment.po2));

        let seal = receipt.get_seal_len();
        let usage = prover.get_peak_memory_usage();
        let throughput = (cycles as f64) / duration.as_secs_f64();

        if !args.quiet {
            println!(
                "| {:>9}k | {:>10} | {:>10} | {:>10} | {:>8}hz |",
                cycles / 1024,
                duration.human_duration().to_string(),
                usage.human_count_bytes().to_string(),
                seal.human_count_bytes().to_string(),
                throughput.human_count_bare().to_string()
            );
        }
    } else {
        println!(
            "| {:>10} | {:>10} | {:>10} | {:>10} | {:>10} |",
            "Cycles", "Duration", "RAM", "Seal", "Speed"
        );

        for iterations in [
            0,           // warm-up
            1,           // 16, 64K
            4 * 1024,    // 17, 128K
            16 * 1024,   // 18, 256K
            32 * 1024,   // 19, 512K
            64 * 1024,   // 20, 1M
            200 * 1024,  // 21, 2M
            400 * 1024,  // 22, 4M
            900 * 1024,  // 23, 8M
            1400 * 1024, // 24, 16M
        ] {
            run_with_iterations(iterations);
        }
    }
}

fn run_with_iterations(iterations: usize) {
    let mut cmd = Command::new(std::env::current_exe().unwrap());
    if iterations == 0 {
        cmd.arg("--quiet");
    }
    let ok = cmd
        .arg("--iterations")
        .arg(iterations.to_string())
        .status()
        .unwrap()
        .success();
    assert!(ok);
}

#[tracing::instrument(skip_all)]
fn top(prover: Rc<dyn Prover>, iterations: u64) -> (Session, Box<dyn SessionReceipt>) {
    let spec = SpecWithIters(BenchmarkSpec::SimpleLoop, iterations);
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&spec).unwrap())
        .build()
        .unwrap();
    let mut exec = Executor::from_elf(env, BENCH_ELF).unwrap();
    let session = exec.run().unwrap();
    let receipt = prover.prove_session(&session).unwrap();
    (session, receipt)
}
