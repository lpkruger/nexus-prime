#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use bnum::types::U256;
use nexus_rt::{print, println, read_private_input, write_output};

#[nexus_rt::main]
fn main() {
    let input = read_private_input::<(U256, U256)>();

    let mut z: U256 = U256::ZERO;
    let expected = U256::parse_str_radix("79484266487585574680924259491585461266177510114289222263303228151162937255137", 10);
    if let Ok((x, y)) = input {
        println!("Read private input: ({}, {})", x, y);
 
        z = (x * y);
        println!("product is {}", z);

        if (x==U256::ONE || y==U256::ONE) {
            // trivial factor, don't allow
            println!("trivial factor, reject");
            write_output::<str>("reject");
        } else if (expected == z) {
            println!("matches, accept");
            write_output::<str>("accept");
        } else {
            println!("does not match, reject");
            write_output::<str>("reject");
        }
    } else {
        println!("No private input provided...");
    }
 
}
