use ark_bn254::Fr as Fr1;
use ark_bn254_r0::Fr as Fr2;
use std::str::FromStr;
use risc0_zkvm::guest::env;
use ark_ff::Field;

risc0_zkvm::guest::entry!(main);

fn main() {
    let a = Fr1::from_str("14226294082152339227274917010873249985458047913852393750281719691125978796426").unwrap();
    let b = Fr1::from_str("17421865113601003971520074901102628465552788678455457825738060633073358930214").unwrap();

    let begin = env::cycle_count();
    let c = a * b;
    let end = env::cycle_count();

    println!("OLD: a * b = c, c = {}, cycle = {}", c, end - begin);

    let a = Fr2::from_str("14226294082152339227274917010873249985458047913852393750281719691125978796426").unwrap();
    let b = Fr2::from_str("17421865113601003971520074901102628465552788678455457825738060633073358930214").unwrap();

    let begin = env::cycle_count();
    let c = a * b;
    let end = env::cycle_count();

    println!("NEW: a * b = c, c = {}, cycle = {}", c, end - begin);

    let a = Fr1::from_str("14226294082152339227274917010873249985458047913852393750281719691125978796426").unwrap();

    let begin = env::cycle_count();
    let c = a.inverse().unwrap();
    let end = env::cycle_count();

    println!("OLD: a ^ -1 = c, c = {}, cycle = {}", c, end - begin);

    let a = Fr2::from_str("14226294082152339227274917010873249985458047913852393750281719691125978796426").unwrap();
    let begin = env::cycle_count();
    let c = a.inverse().unwrap();
    let end = env::cycle_count();

    println!("NEW: a ^ -1 = c, c = {}, cycle = {}", c, end - begin);*/

    let b = Fr1::from_str("17421865113601003971520074901102628465552788678455457825738060633073358930214").unwrap();

    let begin = env::cycle_count();
    let c = b.sqrt().unwrap();
    let end = env::cycle_count();

    println!("OLD: sqrt(b) = c, c = {}, cycle = {}", c, end - begin);

    let b = Fr2::from_str("17421865113601003971520074901102628465552788678455457825738060633073358930214").unwrap();

    let begin = env::cycle_count();
    let c = b.sqrt().unwrap();
    let end = env::cycle_count();

    println!("NEW: sqrt(b) = c, c = {}, cycle = {}", c, end - begin);
}