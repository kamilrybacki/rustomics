pub mod base;
pub mod r#box;
pub mod thermodynamics;

use yaml_rust::Yaml;
use rayon::prelude::*;

use crate::data::load::load_atoms;
use crate::data::load::to_vec_f64;
use crate::data::metrics::UnitSystem;
use crate::system::base::atom::Atom;
use crate::system::r#box::SimulationBox;

use crate::system::base::lattice::scale_cell_basis;
use crate::system::base::lattice::generate_lattice;

pub struct SystemDefinition {
    pub simulation_box: SimulationBox, // Box origin and vectors
    pub atoms: Vec<Atom>,              // Atom type, position, velocity, etc.
    pub units: UnitSystem,             // Unit systems i.e. conversion factors
}

impl SystemDefinition {
    pub fn from(system_definition: &Yaml) -> SystemDefinition {
        let mut new_system = SystemDefinition::initialize_system(system_definition);
        scale_cell_basis(&mut new_system.atoms, &new_system.simulation_box);
        generate_lattice(&mut new_system.atoms, &new_system.simulation_box);
        new_system
    }
    fn initialize_system(config: &Yaml) -> SystemDefinition {
        let box_origin = to_vec_f64::<3>(&config["origin"]);
        let box_vectors = config["vectors"]
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| to_vec_f64::<3>(x))
            .collect::<Vec<[f64; 3]>>()
            .try_into()
            .unwrap();
        let box_periodicity = match &config["periodicity"] {
            Yaml::BadValue => [false, false, false],
            Yaml::String(x) => match x.as_str() {
                "xyz" => [true, true, true],
                "xy" => [true, true, false],
                "xz" => [true, false, true],
                "yz" => [false, true, true],
                "x" => [true, false, false],
                "y" => [false, true, false],
                "z" => [false, false, true],
                _ => panic!("Unknown periodicity"),
            },
            Yaml::Array(x) => match x.len() {
                3 => [
                    x[0].as_bool().unwrap(),
                    x[1].as_bool().unwrap(),
                    x[2].as_bool().unwrap(),
                ],
                _ => panic!("Unknown periodicity"),
            },
            _ => panic!("Unknown periodicity"),
        };
        let unit_cell_replications = match &config["replicas"] {
            Yaml::BadValue => [1, 1, 1],
            Yaml::Array(x) => match x.len() {
                3 => [
                    x[0].as_i64().unwrap() as usize,
                    x[1].as_i64().unwrap() as usize,
                    x[2].as_i64().unwrap() as usize,
                ],
                _ => panic!("Unknown replicas"),
            },
            _ => panic!("Unknown replicas"),
        };
        SystemDefinition {
            simulation_box: SimulationBox::new(box_origin, box_vectors, box_periodicity, unit_cell_replications),
            atoms: load_atoms(&config["atoms"]),
            units: UnitSystem::new(&config["units"]),
        }
    }
    pub fn wrap_atom_positions(&mut self) -> () {
        self.atoms
            .par_iter_mut()
            .for_each(|atom| {
                for dimension in 0..3 {
                    if atom.current.position[dimension] < 0.0 {
                        atom.current.position[dimension] += self.simulation_box.cell.vectors[dimension][dimension];
                    } else if atom.current.position[dimension] > self.simulation_box.cell.vectors[dimension][dimension] {
                        atom.current.position[dimension] -= self.simulation_box.cell.vectors[dimension][dimension];
                    }
                }
            });
    }
}

impl std::fmt::Display for SystemDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let box_definition = self.simulation_box.to_string();
        let atoms_description: String = self
            .atoms
            .iter()
            .map(|x| format!("  {}", x))
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "System definition: {}\nUnits: {}\nAtoms:\n{}",
            box_definition, self.units, atoms_description
        )
    }
}
