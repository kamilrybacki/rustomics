use std::collections::HashMap;

use crate::logic::algebra::euclidean_norm;
use crate::system::base::atom::Atom;
use crate::system::r#box::SimulationBox;

use rayon::prelude::*;

pub struct NeighboursList {
    pub neighbours: HashMap<u64, Vec<(u64, [f64; 3], f64)>>,
    pub log: bool,
    pub frequency: u64,
    cutoff: f64,
}

#[derive(Debug)]
pub struct NeighboursListEntry {
    pub index: u64,
    pub distance_vector: [f64; 3],
    pub distance: f64,
}

impl std::fmt::Display for NeighboursListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(index: {}, distance_vector: {:?}, distance: {})",
            self.index, self.distance_vector, self.distance
        )
    }
}

impl NeighboursList {
    pub fn from(neighbours_settings: &yaml_rust::Yaml) -> NeighboursList {
        NeighboursList {
            neighbours: HashMap::new(),
            cutoff: match &neighbours_settings["cutoff"] {
                yaml_rust::Yaml::Real(cutoff) => match cutoff.parse::<f64>() {
                    Ok(cutoff) => cutoff,
                    Err(_) => panic!("Cutoff must be a real number"),
                },
                _ => panic!("Cutoff must be a real number"),
            },
            frequency: match &neighbours_settings["frequency"] {
                yaml_rust::Yaml::Integer(frequency) => *frequency as u64,
                _ => panic!("Refresh rate must be an integer"),
            },
            log: match &neighbours_settings["log"] {
                yaml_rust::Yaml::Boolean(log) => *log,
                _ => panic!("Log must be a boolean"),
            },
        }
    }
    fn update_for_atom(&mut self, index: usize, atoms: &Vec<Atom>, simbox: &SimulationBox) -> () {
        let new_neighbours = atoms
          .par_iter()
          .enumerate()
          .filter(|(j, _)| *j != index)
          .map(|(j, neighbour)| {
            let mut distance_vector = [
              neighbour.current.position[0] - atoms[index as usize].current.position[0],
              neighbour.current.position[1] - atoms[index as usize].current.position[1],
              neighbour.current.position[2] - atoms[index as usize].current.position[2],
            ];
            // Calculate projections on basis vectors and apply minimum image conventions
            simbox.cell.vectors
              .iter()
              .enumerate()
              .for_each(|(i, &basis_vector)| {
                if !simbox.periodicity[i] {
                  return;
                }
                let basis_vector_norm = euclidean_norm(&basis_vector);
                let mut projection_scaling_factor_dot_product = 0.0;
                for dimension in 0..3 {
                  projection_scaling_factor_dot_product += distance_vector[dimension] *basis_vector[dimension];
                }
                let projection_scaling_factor = projection_scaling_factor_dot_product / basis_vector_norm.powi(2);
                let projection = [
                  basis_vector[0] * projection_scaling_factor,
                  basis_vector[1] * projection_scaling_factor,
                  basis_vector[2] * projection_scaling_factor,
                ];
                let projection_norm = euclidean_norm(&projection);
                let norms_ratio = projection_norm / basis_vector_norm;
                if norms_ratio > 0.5 {
                  distance_vector[0] -= basis_vector[0];
                  distance_vector[1] -= basis_vector[1];
                  distance_vector[2] -= basis_vector[2];
                } else if norms_ratio <= -0.5 {
                  distance_vector[0] += basis_vector[0];
                  distance_vector[1] += basis_vector[1];
                  distance_vector[2] += basis_vector[2];
                }
              });
            let distance = euclidean_norm(&distance_vector);
            if distance < self.cutoff {
              return (j.try_into().unwrap(), distance_vector, distance);
            }
            return (0, [0.0, 0.0, 0.0], 0.0)
          })
          .filter(|(j, d, _)| {
            *j != 0 && *d != [0.0, 0.0, 0.0]
          });
        self.neighbours
            .insert(index.try_into().unwrap(), new_neighbours.collect());
    }
    pub fn update(&mut self, atoms: &Vec<Atom>, simbox: &SimulationBox) -> () {
        self.neighbours.clear();
        atoms
            .iter()
            .enumerate()
            .for_each(|(index, _)| self.update_for_atom(index, atoms, simbox));
    }
    pub fn get_neighbours(&self, index: u64) -> Vec<NeighboursListEntry> {
        match self.neighbours.get(&(index as u64)) {
            None => {
                panic!("No neighbours for index {}", index)
            }
            Some(neighbours) => neighbours
                .iter()
                .map(|(i, distance_vector, distance)| NeighboursListEntry {
                    index: *i,
                    distance_vector: *distance_vector,
                    distance: *distance,
                })
                .collect(),
        }
    }
}

impl std::fmt::Display for NeighboursList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.neighbours)
    }
}