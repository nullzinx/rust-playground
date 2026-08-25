use std::collections::HashMap;
use std::env;

fn parse_args() -> HashMap<String, String> {

    let args: Vec<String> = env::args().collect();
    let mut hmargs: HashMap<String, String> = HashMap::new();

    let mut i = 1;

    while i + 1 < args.len() {
        let flag = args[i].clone();
        let value = args[i + 1].clone();

        hmargs.insert(flag, value);

        i += 2;

    }

    hmargs
}

fn main() {
    let args = parse_args();
    let name = args.get("--name").unwrap();
    println!("hello {} ",name);

}
