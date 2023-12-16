use std::collections::HashMap;

use crate::logic::algebra::euclidean_norm;
use crate::system::base::atom::Atom;

pub struct NeighboursList {
    neighbours: HashMap<u64, Vec<(u64, [f64; 3], f64)>>,
    cutoff: f64,
    refresh_rate: u64,
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
            refresh_rate: match &neighbours_settings["refresh_rate"] {
                yaml_rust::Yaml::Integer(refresh_rate) => *refresh_rate as u64,
                _ => panic!("Refresh rate must be an integer"),
            },
        }
    } 
    pub fn update(&mut self, atoms: &Vec<Atom>) -> () {
        self.neighbours.clear();
        atoms.iter().enumerate().for_each(|(i, atom)| {
            self.neighbours.insert(i as u64, Vec::new());
            atoms.iter().enumerate().for_each(|(j, neighbour)| {
                if i != j {
                    let distance_vector = [
                        neighbour.position[0] - atom.position[0],
                        neighbour.position[1] - atom.position[1],
                        neighbour.position[2] - atom.position[2],
                    ];
                    let distance = euclidean_norm(&distance_vector);
                    if distance < self.cutoff {
                        self.neighbours.get_mut(&(i as u64)).unwrap().push((
                            j as u64,
                            distance_vector,
                            distance,
                        ));
                    }
                }
            })
        })
    }
    pub fn get_neighbours(&self, index: u64) -> Vec<NeighboursListEntry> {
        self.neighbours
          .get(&(index as u64))
          .unwrap()
          .iter()
          .map(|(i, distance_vector, distance)| 
          {
            NeighboursListEntry {
              index: *i,
              distance_vector: *distance_vector,
              distance: *distance,
            }
          })
          .collect()
    }
}
