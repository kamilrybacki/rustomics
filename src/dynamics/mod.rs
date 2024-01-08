pub mod integrators;
pub mod neighbours;

use crate::statics::models::PotentialModel;
use crate::dynamics::neighbours::NeighboursList;
use crate::system::base::atom::Atom;

pub enum DynamicsIntegrator {
    Verlet(integrators::verlet::VerletIntegrator),
}

impl DynamicsIntegrator {
    pub fn next_step(&mut self, atoms: &mut Vec<Atom>, potential: &PotentialModel, neighbours: &mut NeighboursList) -> () {
        match self {
            DynamicsIntegrator::Verlet(x) => x.next_step(atoms, potential, neighbours),
        }
    }
}

impl std::fmt::Display for DynamicsIntegrator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = 2;
        let integrator_description = match self {
            DynamicsIntegrator::Verlet(x) => format!("{:indent$}{}", "", x, indent = indent),
        };
        write!(f, "Integrator:\n{}", integrator_description)
    }
}

#[allow(unused_variables)]
trait NextStepCalculation {
    fn next_step(&mut self, atoms: &mut Vec<Atom>, potential: &PotentialModel, neighbours: &mut NeighboursList) -> () {
        panic!("Not implemented");
    }
}
