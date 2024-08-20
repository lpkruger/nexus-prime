use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable, Viewable,
};

use bnum::types::U256;
use std::env;

const PACKAGE: &str = "guest";

type Input = (U256, U256);
type Output = i32;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input: Input = (
        U256::parse_str_radix("309668711173155080271448809039293680319", 10),
        U256::parse_str_radix("256675161615346299767856994751801776223", 10),
    );

    let mut opts = CompileOpts::new(PACKAGE);
    opts.set_memlimit(1024); // use 1GB memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Setting up Nova public parameters...");
    let mut pp: PP;
    //if (args[1]== "prove") {
        pp = PP::generate().expect("failed to generate parameters");
    //}

    println!("Running vm");
    let out = prover.run_with_input::<Input>(&input).expect("failed to run program");
    println!(">>>>> Logging\n{}<<<<<", out.logs().join(""));

    println!("Compiling guest program again...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Proving execution of vm...");
    let proof = prover.prove_with_input::<Input>(&pp, &input).expect("failed to prove program");

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");
}
