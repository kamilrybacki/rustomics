pub mod base;
pub mod body;
pub mod bounds;
pub mod thermodynamics;

use yaml_rust::Yaml;

use crate::data::load::to_vec_f64;
use crate::data::load::load_atoms;
use crate::system::base::atom::Atom;
use crate::data::metrics::UnitSystem;

pub struct SystemDefinition {
    pub origin: [f64; 3],       // Origin of the simulation box
    pub vectors: [[f64; 3]; 3], // Simulation box vectors
    pub periodicity: [bool; 3], // Periodicity of the simulation box
    pub atoms: Vec<Atom>,       // Atom type, position, velocity, etc.
    pub units: UnitSystem,      // Unit systems i.e. conversion factors
}

impl SystemDefinition {
    pub fn from(system_definition: &Yaml) -> SystemDefinition {
        let box_origin = to_vec_f64::<3>(&system_definition["origin"]);
        let box_vectors = system_definition["vectors"]
            .as_vec()
            .unwrap()
            .iter()
            .map(|x| to_vec_f64::<3>(x))
            .collect::<Vec<[f64; 3]>>()
            .try_into()
            .unwrap();

        SystemDefinition {
            origin: box_origin,
            vectors: box_vectors,
            atoms: load_atoms(&system_definition["atoms"]),
            units: UnitSystem::new(&system_definition["units"]),
            periodicity: match &system_definition["periodicity"] {
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
            },
        }
    }
}

impl std::fmt::Display for SystemDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let definition = format!(
            "\n  Origin: {:?}\n  Vectors: {:?}\n  {:?}",
            self.origin, self.vectors, self.units
        );
        let atoms_description: String = self
            .atoms
            .iter()
            .map(|x| format!("  {}", x))
            .collect::<Vec<String>>()
            .join("\n");
        write!(
            f,
            "System definition: {}\nAtoms:\n{}",
            definition, atoms_description
        )
    }
}
