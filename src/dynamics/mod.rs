pub mod integrators;
pub mod neighbours;

use rayon::prelude::*;

use crate::dynamics::neighbours::NeighboursList;
use crate::statics::models::PotentialModel;
use crate::system::atom::Atom;
use crate::utils::metrics::UnitSystem;

pub enum DynamicsIntegrator {
    Verlet(integrators::verlet::VerletIntegrator),
}

impl DynamicsIntegrator {
    fn convert(&self, atom: &mut Atom, unit_system: &UnitSystem) -> () {
        atom.current.position *= unit_system.distance.0;
        atom.current.velocity *= unit_system.distance.0 / unit_system.time.0;
        atom.current.force *= unit_system.force.0;
        atom.current.potential_energy *= unit_system.energy.0;
    }
    fn revert(&self, atom: &mut Atom, unit_system: &UnitSystem) -> () {
        atom.current.position /= unit_system.distance.0;
        atom.current.velocity /= unit_system.distance.0 / unit_system.time.0;
        atom.current.force /= unit_system.force.0;
        atom.current.potential_energy /= unit_system.energy.0;
    }
    pub fn next_step(
        &mut self,
        atoms: &mut Vec<Atom>,
        potential: &PotentialModel,
        neighbours: &mut NeighboursList,
        unit_system: &UnitSystem,
    ) -> () {
        atoms.
            par_iter_mut().
            for_each(|atom| {
                self.convert(atom, unit_system);
            });
        match self {
            DynamicsIntegrator::Verlet(x) => x.next_step(atoms, potential, neighbours),
        };
        atoms.
            par_iter_mut().
            for_each(|atom| {
                self.revert(atom, unit_system);
            });
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
    fn next_step(
        &mut self,
        atoms: &mut Vec<Atom>,
        potential: &PotentialModel,
        neighbours: &mut NeighboursList,
    ) -> () {
        panic!("Not implemented");
    }
}
