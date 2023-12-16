use std::io::{self, Write};

use crate::setup::Simulation;

const DEFAULT_LOGGER_FORMAT: &str = "id type x y z";

pub struct SimulationLogger {
    pub frequency: i64,
    pub format: String,
    pub redirects: Vec<fn(&str)>,
}

fn print_to_stdout(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}

fn validate_format(yaml: &yaml_rust::Yaml) -> String {
    match yaml {
        yaml_rust::Yaml::BadValue => {
            println!("No format specified, using default");
            DEFAULT_LOGGER_FORMAT.to_string()
        }
        _ => match yaml.as_str() {
            Some(x) => x.to_string(),
            _ => {
                println!("Unknown format {:?}, using default", yaml);
                DEFAULT_LOGGER_FORMAT.to_string()
            }
        },
    }
}

fn validate_redirect(yaml: &yaml_rust::Yaml) -> Option<fn(&str)> {
    let redirect = yaml.as_str().unwrap();
    match redirect {
        "stdout" => return Some(print_to_stdout),
        "file" => {
            println!("File redirect not implemented yet, skipping");
            return None;
        }
        _ => {
            println!("Unknown redirect, skipping");
            return None;
        }
    }
}

impl SimulationLogger {
    pub fn from(yaml: &yaml_rust::Yaml) -> SimulationLogger {
        let frequency = match yaml["frequency"] {
            yaml_rust::Yaml::BadValue => 1,
            _ => yaml["frequency"].as_i64().unwrap(),
        };
        let format = validate_format(&yaml["format"]);

        let mut valid_redirects: Vec<fn(&str)> = Vec::new();
        match yaml["redirects"].as_vec() {
            Some(redirects) => {
                for redirect in redirects.iter() {
                    match validate_redirect(redirect) {
                        Some(valid_redirect) => valid_redirects.push(valid_redirect),
                        None => continue,
                    }
                }
            }
            None => println!("No redirects found!"),
        }

        match valid_redirects.len() {
            0 => {
                println!("No redirects found, using default (STDOUT)");
                valid_redirects.push(print_to_stdout)
            }
            _ => {
                println!("Redirects found: {}", valid_redirects.len());
            }
        }

        SimulationLogger {
            frequency,
            format,
            redirects: valid_redirects,
        }
    }
    pub fn default() -> SimulationLogger {
        SimulationLogger {
            frequency: 1,                              // Print every step
            format: DEFAULT_LOGGER_FORMAT.to_string(), // Print only positions
            redirects: vec![print_to_stdout],          // Print to STDOUT
        }
    }
    pub fn log(&self, simulation: &Simulation) -> () {
        if simulation.clock.current_step % self.frequency == 0 {
            let log_entry = self.construct_log_message(simulation);
            println!(
                "Logging step {}\n\n{}",
                simulation.clock.current_step, log_entry
            );
        }
    }
    fn construct_log_message(&self, simulation: &Simulation) -> String {
        let mut message = String::new();
        for atom in simulation.system.atoms.iter() {
            message.push_str(&format!("{} ", atom.id));
            message.push_str(&format!("{} ", atom.name));
            message.push_str(&format!("{} ", atom.position[0]));
            message.push_str(&format!("{} ", atom.position[1]));
            message.push_str(&format!("{} ", atom.position[2]));
            message.push_str("\n");
        }
        return message;
    }
}
