#[allow(dead_code)]

fn main() {
    println!("Hello, closure!");

    use std::env;
    let args: Vec<_> = env::args().collect();
    for (i, arg) in args.iter().enumerate() {
        println!("\targ{i}: {:#?}", arg);
   }
}