use rayon::prelude::*;

use crate::dynamics::neighbours::NeighboursList;
use crate::dynamics::NextStepCalculation;
use crate::statics::models::PotentialModel;
use crate::system::base::atom::Atom;

pub struct VerletIntegrator {
    pub timestep: f64,
}

impl VerletIntegrator {
    pub fn from(yaml: &yaml_rust::Yaml) -> VerletIntegrator {
        let timestep = yaml["timestep"].as_f64().unwrap();
        VerletIntegrator::new(timestep)
    }
    pub fn new(timestep: f64) -> VerletIntegrator {
        VerletIntegrator { timestep }
    }
}

impl std::fmt::Display for VerletIntegrator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Velocity Verlet integrator")
    }
}

impl NextStepCalculation for VerletIntegrator {
    fn next_step(&mut self, atoms: &mut Vec<Atom>, potential: &PotentialModel, neighbours: &mut NeighboursList) -> () {
        atoms.par_iter_mut().for_each(|atom| {
            atom.previous = atom.current.cache();
            let half_step_velocity = atom.previous.velocity
                + 0.5 * self.timestep * atom.previous.force / atom.mass;
            atom.current.absolute_position = atom.previous.absolute_position + self.timestep * half_step_velocity;
            potential.update(atom, neighbours);
            atom.current.velocity = half_step_velocity + 0.5 * self.timestep * atom.current.force / atom.mass;
        });
    }
}
