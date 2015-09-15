#[macro_use] extern crate matches;

mod dcpu;

use dcpu::cpu::cpu::Cpu as Cpu;

fn main() {
    let cpu = Cpu::new();
    println!("Hello, world!");
}
