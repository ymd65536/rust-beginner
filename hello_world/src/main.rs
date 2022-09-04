use std::env::var;

mod vars;

fn main() {
    println!("Hello, world!");
    vars::run();
    vars::sub_c::func_c();
}
