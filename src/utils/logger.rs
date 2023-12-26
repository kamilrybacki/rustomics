use std::collections::HashMap;
use std::io::{self, Write};

use rayon::prelude::*;
use yaml_rust::Yaml;

use crate::dynamics::neighbours::NeighboursList;
use crate::simulation::Simulation;

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
    let default_formats: HashMap<&str, &str> = HashMap::from([
        ("thermo", "step temperature potential_energy kinetic_energy total_energy"),
        ("neighbours", "id type x y z"),
        ("atoms", "id type x y z vx vy vz fx fy fz mass charge potential_energy kinetic_energy total_energy"),
    ]);

    match &section_yaml["format"] {
        yaml_rust::Yaml::BadValue => {
            println!("No format specified, using default");
            default_formats
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
                default_formats
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
    let redirect_type = match redirect_definition["type"].as_str() {
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
                            let section_type = section_definition["type"].as_str().unwrap();
                            let format = construct_format(section_definition, section_type);
                            (section_type.to_string(), format)
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

        let mut valid_redirects: Vec<LogsRedirect> = Vec::new();
        match &yaml["redirects"] {
            yaml_rust::Yaml::Array(redirects_array) => {
                for redirect in redirects_array.iter() {
                    match construct_redirect(redirect) {
                        Some(valid_redirect) => valid_redirects.push(valid_redirect),
                        None => continue,
                    }
                }
            }
            _ => {
                println!("Redirects must be an array, using default (STDOUT)");
                valid_redirects.push(LogsRedirect {
                    name: "console".to_string(),
                    sections: HashMap::from([(
                        "default_thermo".to_string(),
                        vec![
                            "step",
                            "temperature",
                            "potential_energy",
                            "kinetic_energy",
                            "total_energy",
                        ]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                    )]),
                    precision: 3,
                    handler: print_to_stdout,
                    options: HashMap::new(),
                })
            }
        }

        SimulationLogger {
            frequency,
            redirects: valid_redirects,
            precision: match yaml["precision"].as_i64() {
                Some(x) => x as usize,
                None => 3,
            },
        }
    }

    pub fn default() -> SimulationLogger {
        SimulationLogger {
            frequency: 1, // Print every step
            redirects: vec![LogsRedirect {
                name: "console".to_string(),
                sections: HashMap::from([(
                    "default_thermo".to_string(),
                    vec![
                        "step",
                        "temperature",
                        "potential_energy",
                        "kinetic_energy",
                        "total_energy",
                    ]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                )]),
                precision: 3,
                handler: print_to_stdout,
                options: HashMap::new(),
            }], // print to STDOUT
            precision: 3,
        }
    }

    pub fn log_simulation_state(&self, simulation: &Simulation) -> () {
        if simulation.clock.current_step % self.frequency == 0 {
            let collected_logs: Vec<HashMap<String, Vec<Vec<(String, String)>>>> = self
                .redirects
                .iter()
                .map(|redirect| self.construct_current_state_log(simulation, &redirect.sections))
                .collect::<Vec<HashMap<String, Vec<Vec<(String, String)>>>>>();
            // todo!("Logs only last atom!");
            for log in collected_logs {
                println!("{:?}", log);
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

    fn construct_current_state_log(
        &self,
        simulation: &Simulation,
        sections: &HashMap<String, Vec<String>>,
    ) -> HashMap<String, Vec<Vec<(String, String)>>> {
        sections.iter().map(|(section_name, section_fields)| {
            let section_values = match section_name.as_str() {
                "atoms" => simulation
                    .system
                    .atoms
                    .par_iter()
                    .map(|atom| {
                        let mut found_values = Vec::new();
                        if section_fields.contains(&"step".to_string()) {
                            found_values.push((
                                "step".to_string(),
                                format!("{:} ", simulation.clock.current_step),
                            ));
                        };
                        if section_fields.contains(&"time".to_string()) {
                            found_values.push((
                                "time".to_string(),
                                format!("{:.*} ", self.precision, simulation.clock.current_time),
                            ));
                        };
                        for field in section_fields {
                            if field == "step" || field == "time" {
                                continue;
                            };
                            let field_value: Option<String> = match field.as_str() {
                                "id" => Some(format!("{:} ", atom.id + 1)),
                                "x" => Some(format!(
                                    "{:.*} ",
                                    self.precision, atom.current.position[0]
                                )),
                                "y" => Some(format!(
                                    "{:.*} ",
                                    self.precision, atom.current.position[1]
                                )),
                                "z" => Some(format!(
                                    "{:.*} ",
                                    self.precision, atom.current.position[2]
                                )),
                                "type" => Some(format!("{:} ", atom.name)),
                                "vx" => Some(format!(
                                    "{:.*} ",
                                    self.precision, atom.current.velocity[0]
                                )),
                                "vy" => Some(format!(
                                    "{:.*} ",
                                    self.precision, atom.current.velocity[1]
                                )),
                                "vz" => Some(format!(
                                    "{:.*} ",
                                    self.precision, atom.current.velocity[2]
                                )),
                                "fx" => {
                                    Some(format!("{:.*} ", self.precision, atom.current.force[0]))
                                }
                                "fy" => {
                                    Some(format!("{:.*} ", self.precision, atom.current.force[1]))
                                }
                                "fz" => {
                                    Some(format!("{:.*} ", self.precision, atom.current.force[2]))
                                }
                                "mass" => Some(format!("{:.*} ", self.precision, atom.mass)),
                                "charge" => Some(format!("{:.*} ", self.precision, atom.charge)),
                                _ => None,
                            };
                            match field_value {
                                Some(value) => {
                                    found_values.push((field.to_string(), value));
                                }
                                None => continue,
                            };
                        }
                        found_values
                    })
                    .collect::<Vec<Vec<(String, String)>>>(),
                "thermodynamics" => {
                  let mut found_values: Vec<(String, String)> = Vec::new();
                  for field in section_fields {
                    let found_value = match field.as_str() {
                      "potential_energy" => Some(format!("{:.*} ", self.precision, simulation.energetics.potential_energy)),
                      "kinetic_energy" => Some(format!("{:.*} ", self.precision, simulation.energetics.kinetic_energy)),
                      "total_energy" => Some(format!("{:.*} ", self.precision, simulation.energetics.total_energy)),
                      "temperature" => Some(format!("{:.*} ", self.precision, simulation.energetics.temperature)),
                      _ => None
                    };
                    match found_value {
                      Some(value) => found_values.push((field.to_string(), value)),
                      None => continue
                    }
                  }
                  Vec::from([found_values])
                }
                _ => panic!("Unknown section type {}", section_name),
            };
            (section_name.to_string(), section_values)
        })
        .collect::<HashMap<String, Vec<Vec<(String, String)>>>>()
    }
}
