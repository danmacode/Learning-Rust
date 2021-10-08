extern crate dirs; // lowlevel lib for all OS

fn somefun() {
    match dirs::home_dir() {
        Some(x) => println!("{}", x.display().to_string()),
        None => println!("Cannot divide by 0"),
    }
    if let Some(x) = dirs::config_dir() {
        println!("{}", x.display().to_string())
    }
}

fn main() {
    println!("Hello, world!");
    somefun();
}
