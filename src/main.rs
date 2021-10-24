use std::env;
use std::fs;

// This is a very basic script that will convert a bsm file into a bf file.
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut output = String::new();
    for line in fs::read_to_string(&args[1]).unwrap().lines() {
        let opt: Vec<&str> = line.split(" ").collect();
        match opt[0] {
            "add" => output.push_str(&"+".repeat(opt[1].parse::<usize>().unwrap())),
            "sub" => output.push_str(&"-".repeat(opt[1].parse::<usize>().unwrap())),
            "lshift" => output.push_str(&"<".repeat(opt[1].parse::<usize>().unwrap())),
            "rshift" => output.push_str(&">".repeat(opt[1].parse::<usize>().unwrap())),
            "in" => output.push_str(","),
            "out" => output.push_str("."),
            "bloop" => output.push_str("["),
            "eloop" => output.push_str("]"),
            _ => println!("Ignore opt...")
        }
    }

    fs::write("output.bf", &output).unwrap();
}
