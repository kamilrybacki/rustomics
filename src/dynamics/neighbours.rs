use std::collections::HashMap;

use crate::logic::algebra::euclidean_norm;
use crate::system::base::atom::Atom;

struct NeighboursList {
    neighbours: HashMap<u64, Vec<(u64, [f64; 3], f64)>>,
    cutoff: f64,
}

impl NeighboursList {
    pub fn new(cutoff: f64) -> NeighboursList {
        NeighboursList {
            neighbours: HashMap::new(),
            cutoff: cutoff,
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
    pub fn get_neighbours(&self, index: usize) -> &Vec<(u64, [f64; 3], f64)> {
        self.neighbours.get(&(index as u64)).unwrap()
    }
}
