
mod mem;
mod registers;
mod cpu;
mod instructions;
mod gpu;
mod control;
mod interrupts;

pub use cpu::CPU;

pub fn alliswell() {
    println!("This works");
}