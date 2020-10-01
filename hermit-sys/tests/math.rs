#![feature(test)]
// #![test_runner(common::test_runner)]
// #![feature(custom_test_frameworks)]
// //#![reexport_test_harness_main = "test_main"]
use std;
use core::hint::black_box;
// mod common;

// #[test]
// #[should_panic(expected = "reason")]
// fn divide_by_zero(){
//     let v = black_box(57) / black_box(0);
//     println!("This line should never be executed: {}", black_box(v));
// }

//#[test]
fn normal() {
    let a = black_box(1) + black_box( 2);
    assert_eq!(a, 3);
}
// For some reason this bootloops. Entrypoint is not working properly :(
fn main() {
    println!("Main function");
    normal();
    println!("Finished tests");
}