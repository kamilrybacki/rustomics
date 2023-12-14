extern crate yaml_rust;

use crate::data::load::to_vec_f64;
use crate::data::load::load_atoms;
use crate::system::base::atom::Atom;

pub struct SystemDefinition {
  origin: [f64; 3],  // Origin of the simulation box
  vectors: [[f64; 3]; 3],  // Simulation box vectors
  atoms: Vec<Atom>,  // Atom type, position, velocity, etc.
  units: f64  // Unit conversion factor relative to nanometers
}

pub struct SimulationSetup {
  system: SystemDefinition,  // System definition
}

impl SimulationSetup {
  pub fn new() -> SimulationSetup {
    let system = SystemDefinition {
      origin: [0.0, 0.0, 0.0],
      vectors: [[0.0, 0.0, 0.0]; 3],
      atoms: Vec::new(),
      units: 1.0
    };
    SimulationSetup {
      system: system
    }
  }
  pub fn from(yaml: yaml_rust::Yaml) -> SimulationSetup {
    let mut setup = SimulationSetup::new();
    let system_definition = &yaml["system"];
    setup.system.origin = to_vec_f64::<3>(&system_definition["origin"]);
    setup.system.vectors = system_definition["vectors"].as_vec().unwrap()
      .iter()
      .map(|x| to_vec_f64::<3>(x))
      .collect::<Vec<[f64; 3]>>()
      .try_into()
      .unwrap();
    setup.system.atoms = load_atoms(&system_definition["atoms"]);
    setup
  }
}
