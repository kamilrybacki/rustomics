mod lj;

use crate::dynamics::neighbours::NeighboursList;
use crate::logic::algebra::euclidean_norm;
use crate::system::base::atom::Atom;

pub enum PotentialModel {
    LennardJones(lj::LennardJonesModel),
}

impl PotentialModel {
    pub fn from(yaml: &yaml_rust::Yaml) -> PotentialModel {
        match &yaml["potential"] {
            yaml_rust::Yaml::String(potential) => match potential.as_str() {
                "LennardJones" => {
                    PotentialModel::LennardJones(lj::LennardJonesModel::initialize(&yaml))
                }
                _ => panic!("Potential model not implemented"),
            },
            _ => panic!("Potential model not implemented"),
        }
    }
    pub fn update(&self, atoms: &Vec<Atom>, neighbours_list: &NeighboursList) -> () {
        atoms.iter().enumerate().for_each(|(i, atom)| {
            atom.force = [0.0; 3];
            atom.potential_energy = 0.0;
            neighbours_list
                .get_neighbours(i)
                .iter()
                .for_each(|neighbour| {
                    atom.potential_energy += match self {
                        PotentialModel::LennardJones(model) => {
                            model.calculate_potential(neighbour.distance)
                        }
                    };
                    let force = match self {
                        PotentialModel::LennardJones(model) => {
                            model.calculate_force(neighbour.distance)
                        }
                    };
                    atom.force = [
                        atom.force[0] + force * neighbour.distance_vector[0] / neighbour.distance,
                        atom.force[1] + force * neighbour.distance_vector[1] / neighbour.distance,
                        atom.force[2] + force * neighbour.distance_vector[2] / neighbour.distance,
                    ];
                })
        })
    }
}

pub trait CalculatePotential {
    fn calculate_potential(&self, distance: f64) -> f64;
    fn calculate_force(&self, distance: f64) -> f64;
}
