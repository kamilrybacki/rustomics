extern crate yaml_rust;

use core::fmt;

use crate::data::load::to_vec_f64;
use crate::data::load::load_atoms;
use crate::data::metrics::UnitSystem;
use crate::system::base::atom::Atom;

use crate::dynamics::DynamicsIntegrator;
use crate::dynamics::equations::verlet::VerletIntegrator;

pub struct SystemDefinition {
  origin: [f64; 3],  // Origin of the simulation box
  vectors: [[f64; 3]; 3],  // Simulation box vectors
  atoms: Vec<Atom>,  // Atom type, position, velocity, etc.
  units: UnitSystem  // Unit systems i.e. conversion factors
}

pub struct Simulation {
  system: SystemDefinition,  // System definition
  integrator: DynamicsIntegrator  // Equations of motion numerical integrator
}

impl Simulation {
  pub fn from(yaml: yaml_rust::Yaml) -> Simulation {
    let system_definition = &yaml["system"];
    let box_origin = to_vec_f64::<3>(&system_definition["origin"]);
    let box_vectors = system_definition["vectors"].as_vec().unwrap()
      .iter()
      .map(|x| to_vec_f64::<3>(x))
      .collect::<Vec<[f64; 3]>>()
      .try_into()
      .unwrap();
    let atoms_in_system = load_atoms(&system_definition["atoms"]);
    let dynamics_setup = &yaml["dynamics"];
    Simulation {
      system: SystemDefinition {
        origin: box_origin,
        vectors: box_vectors,
        atoms: atoms_in_system,
        units: UnitSystem::new(&yaml["units"])
      },
      integrator: match dynamics_setup["integrator"].as_str().unwrap() {
        "verlet" => DynamicsIntegrator::Verlet(
          VerletIntegrator::from(&dynamics_setup)
        ),
        _ => panic!("Unknown integrator")
      }
    }
  }
}

impl fmt::Display for SystemDefinition {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let definition = format!(
      "\n  Origin: {:?}\n  Vectors: {:?}\n  {:?}",
      self.origin, self.vectors, self.units
    );
    let atoms_description: String = self.atoms
      .iter()
      .map(|x| format!("  {}", x))
      .collect::<Vec<String>>()
      .join("\n");
    write!(f, "System definition: {}\nAtoms:\n{}", definition, atoms_description)
  }
}

impl fmt::Display for Simulation {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\n{}", self.system, self.integrator)
  }
}