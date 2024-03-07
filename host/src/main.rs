use methods::{METHOD_ELF, METHOD_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let env = ExecutorEnv::builder().build().unwrap();

    let prover = default_prover();

    let timer = std::time::Instant::now();
    let receipt = prover.prove(env, METHOD_ELF).unwrap();
    println!("time: {}", timer.elapsed().as_secs_f64());
    receipt.verify(METHOD_ID).unwrap();
}
