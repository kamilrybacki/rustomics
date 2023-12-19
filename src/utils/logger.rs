use std::io::{self, Write};

use rayon::prelude::*;

use crate::dynamics::neighbours::NeighboursList;
use crate::simulation::Simulation;

const DEFAULT_LOGGER_FORMAT: &str = "id type x y z";
const DEFAULT_THERMODYNAMICS_TO_LOG: &str =
    "temperature potential_energy kinetic_energy total_energy";

pub struct SimulationLogger {
    pub frequency: u64,
    pub redirects: Vec<fn(&str)>,
    pub sections: Vec<String>,
    pub thermo: String,
    format: Vec<String>,
    precision: usize,
}

fn print_to_stdout(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}

fn construct_format(yaml: &yaml_rust::Yaml) -> Vec<String> {
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
    .split_whitespace()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
}

fn contruct_redirects(yaml: &yaml_rust::Yaml) -> Option<fn(&str)> {
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
            yaml_rust::Yaml::Integer(frequency) => match frequency > 0 {
                true => frequency as u64,
                false => panic!("Frequency must be a positive integer"),
            },
            _ => panic!("Frequency must be an integer"),
        };

        let mut valid_redirects: Vec<fn(&str)> = Vec::new();
        match yaml["redirects"].as_vec() {
            Some(redirects) => {
                for redirect in redirects.iter() {
                    match contruct_redirects(redirect) {
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

        let information_sections = match yaml["sections"].as_vec() {
            Some(sections) => sections
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect::<Vec<String>>(),
            None => vec![],
        };

        SimulationLogger {
            frequency,
            format: construct_format(&yaml["format"]),
            redirects: valid_redirects,
            sections: match information_sections.len() {
                0 => vec!["thermo".to_string()],
                _ => information_sections,
            },
            thermo: match yaml["thermo"].as_str() {
                Some(x) => x.to_string(),
                None => DEFAULT_THERMODYNAMICS_TO_LOG.to_string(),
            },
            precision: match yaml["precision"].as_i64() {
                Some(x) => x as usize,
                None => 3,
            },
        }
    }

    pub fn default() -> SimulationLogger {
        SimulationLogger {
            frequency: 1,                              // Print every step
            format: construct_format(
              &yaml_rust::Yaml::String(DEFAULT_LOGGER_FORMAT.to_string())
            ),
            redirects: vec![print_to_stdout],          // Print to STDOUT
            sections: vec!["thermo".to_string()],      // Print only thermo
            thermo: DEFAULT_THERMODYNAMICS_TO_LOG.to_string(),
            precision: 3,
        }
    }

    pub fn log_simulation_state(&mut self, simulation: &Simulation) -> () {
        if simulation.clock.current_step % self.frequency == 0 {
            println!(
                "\nLogging step: {}\n\n{}\n",
                simulation.clock.current_step,
                self.format.join(" ")
            );
            self.print_simulation_log_entry(simulation);
        }
    }

    pub fn log_neighbours_list(&self, neighbours_list: &NeighboursList) -> () {
        if neighbours_list.log {
            println!("Logging neighbours list");
            let current_neighbours_list = &neighbours_list.neighbours;
            for (atom_id, neighbours) in current_neighbours_list.iter() {
                println!("Atom {} has neighbours \n  {:?}", atom_id, neighbours);
            }
        }
    }

    fn print_simulation_log_entry(&mut self, simulation: &Simulation) -> () {
      let messages = simulation.system.atoms
        .par_iter()
        .map(|atom| {
          let mut atom_message = String::new();
          for format in self.format.iter() {
            match format.as_str() {
              "id" => atom_message.push_str(&format!("{:} ", atom.id + 1)),
              "x" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.position[0])),
              "y" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.position[1])),
              "z" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.position[2])),
              "type" => atom_message.push_str(&format!("{:} ", atom.name)),
              "vx" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.velocity[0])),
              "vy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.velocity[1])),
              "vz" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.velocity[2])),
              "fx" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.force[0])),
              "fy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.force[1])),
              "fz" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.force[2])),
              "mass" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.mass)),
              "charge" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.charge)),
              "potential_energy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.potential_energy)),
              // "kinetic_energy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.kinetic_energy())),
              // "total_energy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.total_energy())),
              _ => continue,
            }
          }
          atom_message
        })
        .collect::<Vec<String>>();
      println!("{}\n", messages.join("\n"));
    }
}
