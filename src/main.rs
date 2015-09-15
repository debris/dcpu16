#[macro_use] extern crate matches;

mod dcpu;

use dcpu::cpu::cpu::Cpu as Cpu;

fn main() {
    let dcpu: Cpu = Default::default();
    println!("Hello, world!");
}
