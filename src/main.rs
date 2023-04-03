mod modules;

use modules::transpiler;
use std::env;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Insufficient arguments!");
    }
    

    let res: Vec<String>;

    match transpiler::read_file(args[1].to_string()) {
        Ok(v) => res = v,
        Err(x) => panic!("{}", x)
    }

    let binaries = transpiler::eval_numbers(res);

    match transpiler::write_file(args[2].to_string(), binaries) {
        Ok(_) => {},
        Err(e) => panic!("{}", e)
    }
 
    return;
}
