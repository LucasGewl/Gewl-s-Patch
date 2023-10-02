#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]

mod inkling;
mod gekkouga;
mod toonlink;
mod wolf;
mod marth;
mod sheik;
mod captain;
mod falco;
mod donkey;

#[skyline::main(name = "my_first_plugin")]
pub fn main() {
    inkling::install();
    gekkouga::install();
    wolf::install();
    toonlink::install();
    marth::install();
    falco::install();
    sheik::install();
    captain::install();
    donkey::install();
}