mod data;
mod dynamics;
mod engine;
mod logic;
mod setup;
mod statics;
mod system;
mod utils;

use engine::SimulationRunnerEngine;

fn main() {
    // Get filename from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No script file specified");
    }
    let mut new_simulation = SimulationRunnerEngine::from_script(&args[1]);
    new_simulation.run();
}
