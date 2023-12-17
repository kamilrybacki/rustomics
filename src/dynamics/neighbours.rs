use std::collections::HashMap;

use crate::logic::algebra::euclidean_norm;
use crate::system::base::atom::Atom;

pub struct NeighboursList {
    pub neighbours: HashMap<u64, Vec<(u64, [f64; 3], f64)>>,
    pub log: bool,
    cutoff: f64,
    frequency: u64,
}

pub struct NeighboursListEntry {
    pub index: u64,
    pub distance_vector: [f64; 3],
    pub distance: f64,
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
    fn update_for_atom(&mut self, index: usize, atoms: &Vec<Atom>) -> () {
        self.neighbours
            .insert(index.try_into().unwrap(), Vec::new());
        atoms.iter().enumerate().for_each(|(j, neighbour)| {
            if j == index {
                return;
            }
            let distance_vector = [
                neighbour.position[0] - atoms[index as usize].position[0],
                neighbour.position[1] - atoms[index as usize].position[1],
                neighbour.position[2] - atoms[index as usize].position[2],
            ];
            let distance = euclidean_norm(&distance_vector);
            if distance < self.cutoff {
                self.neighbours.get_mut(&(index as u64)).unwrap().push((
                    j as u64,
                    distance_vector,
                    distance,
                ));
            }
        })
    }
    pub fn update(&mut self, atoms: &Vec<Atom>) -> () {
        self.neighbours.clear();
        atoms
            .iter()
            .enumerate()
            .for_each(|(index, _)| self.update_for_atom(index, atoms));
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
