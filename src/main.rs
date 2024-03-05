use print_dancing_letters::run;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    run(&args[1].to_lowercase(), &args[2]);
}
