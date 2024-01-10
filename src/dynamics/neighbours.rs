use std::collections::HashMap;

use nalgebra::Vector3;

use crate::system::SystemDefinition;

use rayon::prelude::*;

pub struct NeighboursList {
    pub neighbours: HashMap<u64, Vec<(u64, Vector3<f64>, f64)>>,
    pub log: bool,
    cutoff: f64,
}

#[derive(Debug)]
pub struct NeighboursListEntry {
    pub index: u64,
    pub distance_vector: Vector3<f64>,
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
            log: match &neighbours_settings["log"] {
                yaml_rust::Yaml::Boolean(log) => *log,
                _ => panic!("Log must be a boolean"),
            },
        }
    }
    fn update_for_atom(&mut self, index: usize, system: &SystemDefinition) -> () {
        let new_neighbours = system
            .atoms
            .par_iter()
            .enumerate()
            .filter(|(j, _)| *j != index)
            .map(|(neighbour_index, neighbour)| {
                let mut distance_vector = Vector3::<f64>::new(
                    neighbour.current.position[0]
                        - system.atoms[index as usize].current.position[0],
                    neighbour.current.position[1]
                        - system.atoms[index as usize].current.position[1],
                    neighbour.current.position[2]
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
                            (relative_coordinate <= -0.5) as i64 as f64 * -1.0;
                        if minimum_image_coefficient != 0.0 {
                            distance_vector += minimum_image_coefficient * basis_vector;
                        };
                    });
                (
                    neighbour_index as u64,
                    distance_vector,
                    distance_vector.norm(),
                )
            })
            .filter(|(_, _, distance)| *distance < self.cutoff)
            .collect::<Vec<(u64, Vector3<f64>, f64)>>();
        self.neighbours
            .insert(index.try_into().unwrap(), new_neighbours);
    }
    pub fn update(&mut self, system: &mut SystemDefinition) -> () {
        system.wrap_atom_positions();
        self.neighbours.clear();
        system
            .atoms
            .iter()
            .enumerate()
            .for_each(|(index, _)| self.update_for_atom(index, system));
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
