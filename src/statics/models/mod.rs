mod lj;

use nalgebra::Vector3;

use crate::dynamics::neighbors::NeighborsList;
use crate::system::atom::Atom;

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
    pub fn update(&self, atom: &mut Atom, neighbors_list: &NeighborsList) -> () {
        atom.current.potential_energy = 0.0;
        atom.current.force = Vector3::new(0.0, 0.0, 0.0);
        neighbors_list
            .get_neighbors(atom.id as u64)
            .iter_mut()
            .for_each(|neighbor| {
                let current_pair_potential_energy = match self {
                    PotentialModel::LennardJones(model) => {
                        model.calculate_potential(neighbor.distance)
                    }
                };
                atom.current.potential_energy += current_pair_potential_energy;
                let force = match self {
                    PotentialModel::LennardJones(model) => {
                        model.calculate_force(neighbor.distance)
                    }
                };
                atom.current.force += -force * neighbor.distance_vector.normalize();
            });
    }
}

pub trait CalculatePotential {
    fn calculate_potential(&self, distance: f64) -> f64;
    fn calculate_force(&self, distance: f64) -> f64;
}
