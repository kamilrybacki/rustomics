use std::{collections::HashMap, ops::Deref};

use nalgebra::Vector3;

use crate::system::SystemDefinition;

use rayon::prelude::*;


#[derive(Debug, Clone)]
pub struct NeighborsListEntry {
    pub index: u64,
    pub distance_vector: Vector3<f64>,
    pub distance: f64,
}

pub struct NeighborsList {
    pub neighbors: HashMap<u64, Vec<NeighborsListEntry>>,
    pub log: bool,
    cutoff: f64,
}

impl std::fmt::Display for NeighborsListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(index: {}, distance_vector: {:?}, distance: {})",
            self.index, self.distance_vector, self.distance
        )
    }
}

impl NeighborsList {
    pub fn from(neighbors_settings: &yaml_rust::Yaml) -> NeighborsList {
        NeighborsList {
            neighbors: HashMap::new(),
            cutoff: match &neighbors_settings["cutoff"] {
                yaml_rust::Yaml::Real(cutoff) => match cutoff.parse::<f64>() {
                    Ok(cutoff) => cutoff,
                    Err(_) => panic!("Cutoff must be a real number"),
                },
                _ => panic!("Cutoff must be a real number"),
            },
            log: match &neighbors_settings["log"] {
                yaml_rust::Yaml::Boolean(log) => *log,
                _ => panic!("Log must be a boolean"),
            },
        }
    }
    fn update_for_atom(&mut self, index: usize, system: &SystemDefinition) -> () {
        let new_neighbors = system
            .atoms
            .par_iter()
            .enumerate()
            .filter(|(j, _)| *j != index)
            .map(|(neighbor_index, neighbor)| {
                let mut distance_vector = Vector3::<f64>::new(
                    neighbor.current.position[0]
                        - system.atoms[index as usize].current.position[0],
                    neighbor.current.position[1]
                        - system.atoms[index as usize].current.position[1],
                    neighbor.current.position[2]
                        - system.atoms[index as usize].current.position[2],
                );
                system
                    .simulation_box
                    .vectors
                    .row_iter()
                    .for_each(|basis_vector| {
                        let basis_vector =
                            Vector3::<f64>::new(basis_vector[0], basis_vector[1], basis_vector[2]);
                        let distance_vector_projection =
                            distance_vector.dot(&basis_vector) / basis_vector.norm();
                        let relative_coordinate = distance_vector_projection / basis_vector.norm();
                        let minimum_image_coefficient =
                            (relative_coordinate <= -0.5) as i64 as f64 + (relative_coordinate > 0.5) as i64 as f64 * -1.;
                        if minimum_image_coefficient != 0.0 {
                            distance_vector += minimum_image_coefficient * basis_vector;
                        };
                    });
                NeighborsListEntry {
                    index: neighbor_index as u64,
                    distance_vector: distance_vector,
                    distance: distance_vector.norm(),
                }
            })
            .filter(|entry| entry.distance < self.cutoff)
            .collect::<Vec<NeighborsListEntry>>();
        self.neighbors
            .insert(index.try_into().unwrap(), new_neighbors);
    }
    pub fn update(&mut self, system: &mut SystemDefinition) -> () {
        system.wrap_atom_positions();
        self.neighbors.clear();
        system
            .atoms
            .iter()
            .enumerate()
            .for_each(|(index, _)| self.update_for_atom(index, system));
    }
    pub fn get_neighbors(&self, index: u64) -> Vec<NeighborsListEntry> {
        match self.neighbors.get(&(index as u64)) {
            None => {
                println!("No neighbors for index {}", index);
                vec![]
            }
            Some(neighbors) => neighbors.clone(),
        }
    }
}

impl std::fmt::Display for NeighborsList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.neighbors)
    }
}
