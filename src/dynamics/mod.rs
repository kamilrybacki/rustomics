pub mod equations;
pub mod neighbours;

use crate::system::base::atom::Atom;

pub enum DynamicsIntegrator {
    Verlet(equations::verlet::VerletIntegrator),
}

impl DynamicsIntegrator {
    pub fn next(
        &mut self,
        atoms: &mut Vec<Atom>,
        box_vectors: &[[f64; 3]; 3],
        periodicity: &[bool; 3],
    ) -> () {
        match self {
            DynamicsIntegrator::Verlet(x) => x.next(atoms, box_vectors, periodicity),
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

trait NextStepCalculation {
    fn next(
        &mut self,
        atoms: &mut Vec<Atom>,
        box_vectors: &[[f64; 3]; 3],
        periodicity: &[bool; 3],
    ) -> () {
        panic!("Not implemented");
    }
}
