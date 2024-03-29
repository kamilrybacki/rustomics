pub mod r#box;
pub mod atom;
pub mod lattice;
pub mod cell;

use nalgebra::Matrix3;

use rayon::prelude::*;
use yaml_rust::Yaml;

use crate::io::input::load_atoms;
use crate::io::input::to_vec_f64;

use crate::system::r#box::SimulationBox;
use crate::system::atom::Atom;
use crate::system::lattice::generate_lattice;
use crate::system::lattice::scale_cell_basis;

use crate::utils::metrics::UnitSystem;

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
        new_system.atoms.par_iter_mut().for_each(|atom| {
            atom.previous = atom.current.cache();
        });
        new_system
    }
    fn initialize_system(config: &Yaml) -> SystemDefinition {
        let box_vectors = config["cell"]
            .as_vec()
            .unwrap()
            .par_iter()
            .map(|x| to_vec_f64::<3>(x))
            .flatten()
            .collect::<Vec<f64>>();
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
            simulation_box: SimulationBox::new(
                Matrix3::from_vec(box_vectors),
                box_periodicity,
                unit_cell_replications,
            ),
            atoms: load_atoms(&config["atoms"]),
            units: UnitSystem::new(&config["units"]),
        }
    }
    pub fn wrap_atom_positions(&mut self) -> () {
        self.atoms.par_iter_mut().for_each(|atom| {
            atom.current.position = self.simulation_box.wrap_position(atom.current.position)
        });
    }
}

impl std::fmt::Display for SystemDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let box_definition = self.simulation_box.to_string();
        let atoms_description: String = self
            .atoms
            .par_iter()
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
