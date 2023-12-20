pub mod integrators;
pub mod neighbours;

use crate::system::base::atom::Atom;

pub enum DynamicsIntegrator {
    Verlet(integrators::verlet::VerletIntegrator),
}

impl DynamicsIntegrator {
    pub fn next_positions(
        &mut self,
        atoms: &mut Vec<Atom>,
    ) -> () {
        match self {
            DynamicsIntegrator::Verlet(x) => x.next_positions(atoms),
        }
    }
    pub fn next_velocities(
        &mut self,
        atoms: &mut Vec<Atom>,
    ) -> () {
        match self {
            DynamicsIntegrator::Verlet(x) => x.next_velocities(atoms),
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
    fn next_positions(
        &mut self,
        atoms: &mut Vec<Atom>,
    ) -> () {
        panic!("Not implemented");
    }
    fn next_velocities(
        &mut self,
        atoms: &mut Vec<Atom>,
    ) -> () {
        panic!("Not implemented");
    }
}
