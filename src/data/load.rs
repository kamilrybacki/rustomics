use std::fs::read_to_string;
use yaml_rust::Yaml;

use crate::data::setup::Simulation;
use crate::system::base::atom::Atom;


fn parse_yaml(filepath: &String) -> yaml_rust::Yaml {
  let script_file = read_to_string(&filepath)
    .expect("Failed to read script file");
  let script_yaml = yaml_rust::YamlLoader::load_from_str(&script_file)
    .expect("Failed to parse script file");
  script_yaml[0].clone()
}

pub fn parse(filepath: &String) -> Simulation {
  let extension = filepath.split(".").last().unwrap();
  let script: yaml_rust::Yaml = match extension {
    "yaml" => parse_yaml(filepath),
    _ => panic!("Unknown file extension")
  };
  let initialized_simulation_box = Simulation::from(script);
  initialized_simulation_box
}

pub fn to_vec_f64<const SIZE: usize>(yaml: &yaml_rust::Yaml) -> [f64; SIZE] {
  let vectorized_yaml_entry: &Vec<Yaml> = yaml.as_vec().unwrap();
  if vectorized_yaml_entry.len() != SIZE {
    panic!("Expected {} elements, found {}", SIZE, vectorized_yaml_entry.len());
  }
  vectorized_yaml_entry 
    .iter()
    .map(
      |x| match x {
        Yaml::Real(x) => x.parse::<f64>().unwrap(),
        Yaml::String(x) => x.parse::<f64>().unwrap(),
        Yaml::Integer(x) => *x as f64,
        _ => panic!("Unknown type")
      }
    )
    .collect::<Vec<f64>>()
    .try_into()
    .unwrap()
}

pub fn load_atoms(yaml: &yaml_rust::Yaml) -> Vec<Atom> {
  yaml
    .as_vec()
    .unwrap()
    .iter()
    .map(|x| Atom::from(x))
    .collect::<Vec<Atom>>()
}
