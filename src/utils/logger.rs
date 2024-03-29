use std::collections::HashMap;
use std::io::{self, Write};

use rayon::prelude::*;

use crate::dynamics::neighbors::NeighborsList;
use crate::simulation::Simulation;

const DEFAULT_PRECISION: usize = 3;

fn get_header_label(field_name: &str) -> String {
    match field_name {
        "step" => "Step".to_string(),
        "name" => "Name".to_string(),
        "time" => "Time".to_string(),
        "id" => "ID".to_string(),
        "x" => "X".to_string(),
        "y" => "Y".to_string(),
        "z" => "Z".to_string(),
        "type" => "Type".to_string(),
        "vx" => "Vx".to_string(),
        "vy" => "Vy".to_string(),
        "vz" => "Vz".to_string(),
        "fx" => "Fx".to_string(),
        "fy" => "Fy".to_string(),
        "fz" => "Fz".to_string(),
        "mass" => "Mass".to_string(),
        "charge" => "Charge".to_string(),
        "potential_energy" => "PotEn".to_string(),
        "kinetic_energy" => "KinEn".to_string(),
        "total_energy" => "TotEn".to_string(),
        "temperature" => "Temp".to_string(),
        _ => panic!("Unknown field name {}", field_name),
    }
}

pub struct LogsRedirect {
    pub name: String,
    pub sections: HashMap<String, Vec<String>>,
    pub precision: usize,
    pub handler: fn(&str),
    _options: HashMap<String, String>,
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

fn print_to_file(message: &str) {
    let (filename, message) = match message.split("|").collect::<Vec<&str>>().as_slice() {
        [filename, message] => (*filename, *message),
        _ => panic!("Invalid message format"),
    };
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    file.write_all(message.as_bytes()).unwrap();
}

fn construct_format(section_yaml: &yaml_rust::Yaml, section_type: &str) -> Vec<String> {
    let default_formats: HashMap<&str, &str> = HashMap::from([
        ("thermodynamics", "step temperature potential_energy kinetic_energy total_energy"),
        ("neighbors", "id type x y z"),
        ("atoms", "id type x y z vx vy vz fx fy fz mass charge potential_energy kinetic_energy total_energy"),
        ("xyz", "name x y z")
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
                _options: HashMap::new(),
            };
            return Some(new_redirect);
        }
        "file" => {
            println!("File redirect not implemented yet, skipping");
            return None;
        }
        "xyz" => {
            let _ = std::fs::remove_file(redirect_definition["filename"].as_str().unwrap());
            Some(LogsRedirect {
                name: "xyz".to_string(),
                sections: HashMap::from([(
                    "xyz".to_string(),
                    vec!["name", "x", "y", "z"]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                )]),
                precision: match redirect_definition["precision"].as_i64() {
                    Some(x) => x as usize,
                    None => DEFAULT_PRECISION,
                },
                handler: print_to_file,
                _options: {
                    let mut options = HashMap::new();
                    options.insert(
                        "filename".to_string(),
                        redirect_definition["filename"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    );
                    options
                },
            })
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
                        "thermodynamics".to_string(),
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
                    _options: HashMap::new(),
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
                    "thermodynamics".to_string(),
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
                _options: HashMap::new(),
            }], // print to STDOUT
            precision: 3,
        }
    }

    pub fn log_simulation_state(&self, simulation: &Simulation) -> () {
        if simulation.clock.current_step % self.frequency == 0 || simulation.clock.current_step == 1
        {
            self.redirects.iter().for_each(|redirect| {
                let collected_logs =
                    self.construct_current_state_log(simulation, &redirect.sections);
                let serialized_logs = self.serialize_collected_logs(collected_logs);
                if redirect.name == "xyz" {
                    (redirect.handler)(&format!(
                        "{}|{}",
                        redirect._options["filename"], serialized_logs
                    ));
                } else {
                    (redirect.handler)(&serialized_logs);
                }
            })
        }
    }

    pub fn serialize_collected_logs(
        &self,
        collected_logs: HashMap<String, Vec<Vec<(String, String)>>>,
    ) -> String {
        let mut serialized_log = String::new();
        for (section_name, section_values) in collected_logs.iter() {
            let mut header = String::new();
            match section_name.as_str() {
                "xyz" => {
                    header.push_str(&format!("{}\n", section_values.len().to_string().as_str()));
                    header.push_str("Generated via Rustomics");
                }
                _ => {
                    serialized_log.push_str(&format!("\n[{}]\n", section_name.to_uppercase()));
                    // Print header for columns
                    let mut header = String::new();
                    for field in section_values
                        .first()
                        .unwrap()
                        .iter()
                        .map(|(field_name, _)| field_name)
                    {
                        header.push_str(&format!("{} ", get_header_label(field)));
                    }
                }
            }
            serialized_log.push_str(&format!("{}\n", header));
            let serialized_values: Vec<String> = section_values
                .iter()
                .map(|values| {
                    let mut serialized_values = String::new();
                    for (_, field_value) in values.iter() {
                        serialized_values.push_str(&format!("{} ", field_value));
                    }
                    serialized_values
                })
                .collect::<Vec<String>>();
            for serialized_value in serialized_values {
                serialized_log.push_str(&format!("{}\n", serialized_value));
            }
        }
        serialized_log
    }

    pub fn construct_neighbors_list_log(&self, neighbors_list: &NeighborsList) -> () {
        if neighbors_list.log {
            println!("Logging neighbors list");
            let current_neighbors_list = &neighbors_list.neighbors;
            for (atom_id, neighbors) in current_neighbors_list.iter() {
                println!("Atom {} has neighbors \n  {:?}", atom_id, neighbors);
            }
        }
    }

    fn format_value(&self, value: f64) -> String {
        format!("{0:1.prec$e}", value, prec = self.precision).replace("e0", "")
    }

    fn construct_current_state_log(
        &self,
        simulation: &Simulation,
        sections: &HashMap<String, Vec<String>>,
    ) -> HashMap<String, Vec<Vec<(String, String)>>> {
        sections
            .iter()
            .map(|(section_name, section_fields)| {
                let section_values = match section_name.as_str() {
                    "atoms" | "xyz" => simulation
                        .system
                        .atoms
                        .par_iter()
                        .map(|atom| {
                            let mut found_values = Vec::new();
                            if section_fields.contains(&"step".to_string()) {
                                found_values.push((
                                    "step".to_string(),
                                    format!("{:}", simulation.clock.current_step),
                                ));
                            };
                            if section_fields.contains(&"time".to_string()) {
                                found_values.push((
                                    "time".to_string(),
                                    format!("{0:1.2e}", simulation.clock.current_time),
                                ));
                            };
                            for field in section_fields {
                                if field == "step" || field == "time" {
                                    continue;
                                };
                                let field_value: Option<String> = match field.as_str() {
                                    "name" => Some(format!("{:}", atom.name)),
                                    "id" => Some(format!("{:}", atom.id + 1)),
                                    "x" => Some(self.format_value(atom.current.position[0])),
                                    "y" => Some(self.format_value(atom.current.position[1])),
                                    "z" => Some(self.format_value(atom.current.position[2])),
                                    "type" => Some(format!("{:}", atom.name)),
                                    "vx" => Some(self.format_value(atom.current.velocity[0])),
                                    "vy" => Some(self.format_value(atom.current.velocity[1])),
                                    "vz" => Some(self.format_value(atom.current.velocity[2])),
                                    "fx" => Some(self.format_value(atom.current.force[0])),
                                    "fy" => Some(self.format_value(atom.current.force[1])),
                                    "fz" => Some(self.format_value(atom.current.force[2])),
                                    "mass" => Some(self.format_value(atom.mass)),
                                    "charge" => Some(self.format_value(atom.charge)),
                                    _ => None,
                                };
                                match field_value {
                                    Some(value) => {
                                        found_values
                                            .push((field.to_string(), value.replace("e0", "")));
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
                                "step" => Some(format!("{:}", simulation.clock.current_step)),
                                "time" => Some(self.format_value(simulation.clock.current_time)),
                                "potential_energy" => {
                                    Some(self.format_value(simulation.energetics.potential_energy))
                                }
                                "kinetic_energy" => {
                                    Some(self.format_value(simulation.energetics.kinetic_energy))
                                }
                                "total_energy" => {
                                    Some(self.format_value(simulation.energetics.total_energy))
                                }
                                "temperature" => {
                                    Some(self.format_value(simulation.energetics.temperature))
                                }
                                _ => None,
                            };
                            match found_value {
                                Some(value) => found_values.push((field.to_string(), value)),
                                None => continue,
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
