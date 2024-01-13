mod lj;

use rayon::prelude::*;

use nalgebra::Vector3;

use crate::dynamics::neighbours::NeighboursList;
use crate::system::atom::Atom;
use crate::utils::metrics::UnitSystem;

pub enum PotentialModel {
    LennardJones(lj::LennardJonesModel),
}

impl PotentialModel {
    pub fn from(potential_definition: &yaml_rust::Yaml) -> PotentialModel {
        match &potential_definition["model"] {
            yaml_rust::Yaml::String(potential) => match potential.as_str() {
                "lj" => PotentialModel::LennardJones(lj::LennardJonesModel::initialize(
                    &potential_definition,
                )),
                _ => panic!("Potential model not implemented"),
            },
            _ => panic!("Potential model not implemented"),
        }
    }
    pub fn apply_units_system(&mut self, units: &UnitSystem) -> () {
        match self {
            PotentialModel::LennardJones(model) => model.apply_units_system(units),
        }
    }
    pub fn update(&self, atom: &mut Atom, neighbours_list: &NeighboursList) -> () {
        atom.current.potential_energy = 0.0;
        atom.current.force = Vector3::new(0.0, 0.0, 0.0);
        neighbours_list
            .get_neighbours(atom.id as u64)
            .iter_mut()
            .for_each(|neighbour| {
                let current_pair_potential_energy = match self {
                    PotentialModel::LennardJones(model) => {
                        model.calculate_potential(neighbour.distance)
                    }
                };
                atom.current.potential_energy += current_pair_potential_energy;
                let force = match self {
                    PotentialModel::LennardJones(model) => {
                        model.calculate_force(neighbour.distance)
                    }
                };
                atom.current.force += -force * neighbour.distance_vector.normalize();
            })
    }
}

pub trait CalculatePotential {
    fn apply_units_system(&mut self, units: &UnitSystem) -> ();
    fn calculate_potential(&self, distance: f64) -> f64;
    fn calculate_force(&self, distance: f64) -> f64;
}
