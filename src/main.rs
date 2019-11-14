use std::env;
use notifica;

fn main() {
    let args: Vec<String> = env::args().collect();

    let message = &args[1];

    notifica::notify("Roar", message);
}
