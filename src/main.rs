mod data;
mod system;

fn main() {
    // Get filename from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No script file specified");
    }
    let script_filepath = String::from(&args[1]);
    data::load::load(&script_filepath);
}
