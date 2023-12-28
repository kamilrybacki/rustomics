mod lj;

use rayon::prelude::*;

use crate::dynamics::neighbours::NeighboursList;
use crate::system::base::atom::Atom;

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
    pub fn update(&self, atoms: &mut Vec<Atom>, neighbours_list: &NeighboursList) -> () {
        atoms.par_iter_mut().enumerate().for_each(|(i, atom)| {
            atom.previous = atom.current.cache();
            atom.current.force = [0.0; 3];
            atom.current.potential_energy = 0.0;
            neighbours_list
                .get_neighbours(i as u64)
                .iter_mut()
                .for_each(|neighbour| {
                    atom.current.potential_energy += match self {
                        PotentialModel::LennardJones(model) => {
                            model.calculate_potential(neighbour.distance)
                        }
                    };
                    let force = match self {
                        PotentialModel::LennardJones(model) => {
                            model.calculate_force(neighbour.distance)
                        }
                    };
                    atom.current.force = [
                        -atom.current.force[0]
                            + force * neighbour.distance_vector[0] / neighbour.distance,
                        -atom.current.force[1]
                            + force * neighbour.distance_vector[1] / neighbour.distance,
                        -atom.current.force[2]
                            + force * neighbour.distance_vector[2] / neighbour.distance,
                    ];
                })
        })
    }
}

pub trait CalculatePotential {
    fn calculate_potential(&self, distance: f64) -> f64;
    fn calculate_force(&self, distance: f64) -> f64;
}
