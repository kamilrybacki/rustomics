use std::collections::HashMap;
use std::io::{self, Write};

use rayon::prelude::*;

use crate::dynamics::neighbours::NeighboursList;
use crate::simulation::Simulation;

const DEFAULT_FORMATS_FOR_SECTIONS: HashMap<&str, &str> = HashMap::from([
    ("thermo", "step temperature potential_energy kinetic_energy total_energy"),
    ("neighbours", "id type x y z"),
    ("atoms", "id type x y z vx vy vz fx fy fz mass charge potential_energy kinetic_energy total_energy"),
]);
const DEFAULT_PRECISION: usize = 3;

pub struct LogsRedirect {
    pub name: String,
    pub sections: HashMap<String, Vec<String>>,
    pub precision: usize,
    pub handler: fn(&str),
    options: HashMap<String, String>,
}

pub struct SimulationLogger {
    pub frequency: u64,
    pub redirects: Vec<LogsRedirect>,
    precision: usize,
}

fn print_to_stdout(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}

fn construct_format(section_yaml: &yaml_rust::Yaml, section_type: &str) -> Vec<String> {
    match section_yaml["format"] {
        yaml_rust::Yaml::BadValue => {
            println!("No format specified, using default");
            DEFAULT_FORMATS_FOR_SECTIONS
              .get(section_type)
              .unwrap()
              .split_whitespace()
              .map(|x| x.to_string())
              .collect::<Vec<String>>()
        }
        yaml_rust::Yaml::Array(x) => x
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect::<Vec<String>>(),
        yaml_rust::Yaml::String(x) => x
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
        _ => match section_yaml.as_str() {
            Some(x) => x
                .split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            _ => {
                println!("Unknown format {:?}, using default", section_yaml);
                DEFAULT_FORMATS_FOR_SECTIONS
                  .get(section_type)
                  .unwrap()
                  .split_whitespace()
                  .map(|x| x.to_string())
                  .collect::<Vec<String>>()
            }
        },
    }
}

fn construct_redirect(redirect_definition: &yaml_rust::Yaml) -> Option<LogsRedirect> {
    let redirect_type = match redirect_definition["section"].as_str() {
        Some(x) => x,
        None => {
            println!("No section specified, skipping");
            return None;
        }
    };
    match redirect_type {
        "console" => {
          let new_redirect = LogsRedirect {
            name: "console".to_string(),
            sections: match redirect_definition["sections"].as_vec() {
              Some(sections) => sections
                .iter()
                .map(|section_definition| {
                  let section_name = section_definition.as_str().unwrap();
                  let format = construct_format(section_definition, section_name);
                  (section_name.to_string(), format)
                })
                .collect::<HashMap<String, Vec<String>>>(),
              None => HashMap::new(),
            },
            precision: match redirect_definition["precision"].as_i64() {
              Some(x) => x as usize,
              None => DEFAULT_PRECISION,
            },
            handler: print_to_stdout,
            options: HashMap::new(),
          };
          return Some(new_redirect);
        }
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
                    match construct_redirect(redirect) {
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

    pub fn log_simulation_header(&self, simulation: &Simulation) -> () {
        let header: String = "# Starting simulation".to_string();
        for redirect in self.redirects.iter() {
            redirect(&header);
        }
    }

    pub fn log_simulation_state(&self, simulation: &Simulation) -> () {
        if simulation.clock.current_step % self.frequency == 0 {
            for redirect in self.redirects.iter() {
              println!(
                  "\nLogging step: {}\n\n{}\n",
                  simulation.clock.current_step,
                  self.format.join(" ")
              );
              let log_entry = self.construct_simulation_state_log(simulation);
              redirect(&log_entry.join("\n"));
            }
        }
    }

    pub fn construct_neighbours_list_log(&self, neighbours_list: &NeighboursList) -> () {
        if neighbours_list.log {
            println!("Logging neighbours list");
            let current_neighbours_list = &neighbours_list.neighbours;
            for (atom_id, neighbours) in current_neighbours_list.iter() {
                println!("Atom {} has neighbours \n  {:?}", atom_id, neighbours);
            }
        }
    }

    fn construct_simulation_state_log(&self, simulation: &Simulation) -> Vec<String> {
      let mut messages = simulation.system.atoms
        .par_iter()
        .map(|atom| {
          let mut atom_message = String::new();
          for format in self.format.iter() {
            match format.as_str() {
              "id" => atom_message.push_str(&format!("{:} ", atom.id + 1)),
              "x" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.position[0])),
              "y" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.position[1])),
              "z" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.position[2])),
              "type" => atom_message.push_str(&format!("{:} ", atom.name)),
              "vx" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.velocity[0])),
              "vy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.velocity[1])),
              "vz" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.velocity[2])),
              "fx" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.force[0])),
              "fy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.force[1])),
              "fz" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.force[2])),
              "mass" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.mass)),
              "charge" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.charge)),
              "potential_energy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.potential_energy)),
              // "kinetic_energy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.kinetic_energy())),
              // "total_energy" => atom_message.push_str(&format!("{:.*} ", self.precision, atom.current.total_energy())),
              _ => continue,
            }
          }
          atom_message
        })
        .collect::<Vec<String>>();
      messages.push("\n\n".to_string());
      messages
    }
}
