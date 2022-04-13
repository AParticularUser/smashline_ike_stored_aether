#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod ike;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    ike::install();
}