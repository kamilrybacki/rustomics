use std::io::{self, Write};

use crate::dynamics::neighbours::NeighboursList;
use crate::simulation::Simulation;

const DEFAULT_LOGGER_FORMAT: &str = "id type x y z";
const DEFAULT_THERMODYNAMICS_TO_LOG: &str =
    "temperature potential_energy kinetic_energy total_energy";

pub struct SimulationLogger {
    pub frequency: u64,
    pub format: std::collections::HashMap<String, Vec<String>>,
    pub redirects: Vec<fn(&str)>,
    pub sections: Vec<String>,
    pub thermo: String,
}

fn print_to_stdout(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}

fn construct_format(yaml: &yaml_rust::Yaml) -> String {
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

fn create_format_fields_map(format: String) -> std::collections::HashMap<String, Vec<String>> {
    let fields_map = format
      .split_whitespace()
      .into_iter()
      .map(|x| (x.to_string(), Vec::new()))
      .collect::<std::collections::HashMap<String, Vec<String>>>();
    return fields_map;
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
            format: create_format_fields_map(
              construct_format(&yaml["format"])
            ),
            redirects: valid_redirects,
            sections: match information_sections.len() {
                0 => vec!["thermo".to_string()],
                _ => information_sections,
            },
            thermo: match yaml["thermo"].as_str() {
                Some(x) => x.to_string(),
                None => DEFAULT_THERMODYNAMICS_TO_LOG.to_string(),
            },
        }
    }

    pub fn default() -> SimulationLogger {
        SimulationLogger {
            frequency: 1,                              // Print every step
            format: create_format_fields_map(
              DEFAULT_LOGGER_FORMAT.to_string()
            ), // Print only positions
            redirects: vec![print_to_stdout],          // Print to STDOUT
            sections: vec!["thermo".to_string()],      // Print only thermo
            thermo: DEFAULT_THERMODYNAMICS_TO_LOG.to_string(),
        }
    }

    pub fn log_simulation_state(&mut self, simulation: &Simulation) -> () {
        if simulation.clock.current_step % self.frequency == 0 {
            let log_entry = self.construct_simulation_log_message(simulation);
            println!(
                "Logging step {}\n\n{}",
                simulation.clock.current_step, log_entry
            );
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

    fn construct_simulation_log_message(&mut self, simulation: &Simulation) -> String {
      for atom in simulation.system.atoms.iter() {
        self.format
            .iter_mut()
            .for_each(|(section, fields)| {
              let field_value = match section.as_str() {
                "id" => atom.id.to_string(),
                "type" => atom.name.to_string(),
                "x" => atom.position[0].to_string(),
                "y" => atom.position[1].to_string(),
                "z" => atom.position[2].to_string(),
                "vx" => atom.velocity[0].to_string(),
                "vy" => atom.velocity[1].to_string(),
                "vz" => atom.velocity[2].to_string(),
                "fx" => atom.force[0].to_string(),
                "fy" => atom.force[1].to_string(),
                "fz" => atom.force[2].to_string(),
                _ => "".to_string(),
              };
              fields.push(field_value);
            })
      }
      let message = String::new();
      return message
    }
}
