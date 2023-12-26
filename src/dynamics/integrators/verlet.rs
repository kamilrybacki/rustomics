use rayon::prelude::*;

use crate::system::base::atom::Atom;

use crate::dynamics::NextStepCalculation;

pub struct VerletIntegrator {
    pub timestep: f64,
    pub flavor: String,
}

impl VerletIntegrator {
    pub fn from(yaml: &yaml_rust::Yaml) -> VerletIntegrator {
        let timestep = yaml["timestep"].as_f64().unwrap();
        let flavor = match yaml["flavor"].as_str() {
            Some(x) => String::from(x),
            None => String::from("velocity"),
        };
        VerletIntegrator::new(timestep, flavor)
    }
    pub fn new(timestep: f64, flavor: String) -> VerletIntegrator {
        VerletIntegrator { timestep, flavor }
    }
}

impl std::fmt::Display for VerletIntegrator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Verlet integrator ({} flavor)", self.flavor)
    }
}

impl NextStepCalculation for VerletIntegrator {
    fn next_positions(&mut self, atoms: &mut Vec<Atom>) -> () {
        atoms.par_iter_mut().for_each(|atom| {
            for dimension in 0..3 {
                atom.current.position[dimension] = match self.flavor.as_str() {
                    "velocity" => {
                        atom.current.position[dimension]
                            + atom.current.velocity[dimension] * self.timestep
                            + 0.5 * atom.current.force[dimension] / atom.mass
                                * self.timestep.powi(2)
                    }
                    _ => {
                        println!("Unknown flavor");
                        atom.current.position[dimension]
                    }
                };
            }
        });
    }
    fn next_velocities(&mut self, atoms: &mut Vec<Atom>) -> () {
        atoms.par_iter_mut().for_each(|atom| {
            for dimension in 0..3 {
                atom.current.velocity[dimension] = match self.flavor.as_str() {
                    "velocity" => {
                        atom.current.velocity[dimension]
                            + ((atom.current.force[dimension] + atom.previous.force[dimension])
                                / 2.0
                                * atom.mass)
                                * self.timestep
                    }
                    _ => {
                        println!("Unknown flavor");
                        atom.current.velocity[dimension]
                    }
                };
            }
        });
    }
}
