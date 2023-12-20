extern crate periodic_table_on_an_enum;

use crate::data::load::to_vec_f64;

#[derive(Debug)]
struct CurrentState {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub force: [f64; 3],
    pub potential_energy: f64,
    pub kinetic_energy: f64,
    pub total_energy: f64,
}

#[derive(Debug)]
pub struct PreviousState {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub force: [f64; 3],
    pub potential_energy: f64,
    pub kinetic_energy: f64,
    pub total_energy: f64,
}

#[derive(Debug)]
pub struct Atom {
    pub id: u64,
    pub name: String,
    pub previous: PreviousState,
    pub current: CurrentState,
    pub mass: f64,
    pub charge: f64,
}

impl Atom {
    pub fn new() -> Atom {
        Atom {
            id: 0,
            previous: PreviousState {
                position: [0.0, 0.0, 0.0],
                velocity: [0.0, 0.0, 0.0],
                force: [0.0, 0.0, 0.0],
                potential_energy: 0.0,
                kinetic_energy: 0.0,
                total_energy: 0.0,
            },
            current: CurrentState {
                position: [0.0, 0.0, 0.0],
                velocity: [0.0, 0.0, 0.0],
                force: [0.0, 0.0, 0.0],
                potential_energy: 0.0,
                kinetic_energy: 0.0,
                total_energy: 0.0,
            },
            mass: 1.0,
            charge: 0.0,
            name: String::from("NaN"),
        }
    }
    pub fn from(yaml: &yaml_rust::Yaml) -> Atom {
        let mut atom = Atom::new();
        atom.current.position = to_vec_f64::<3>(&yaml["position"]);
        atom.current.velocity = match &yaml["velocity"] {
            yaml_rust::Yaml::Array(_x) => to_vec_f64::<3>(&yaml["velocity"]),
            _ => [0.0, 0.0, 0.0],
        };
        atom.current.force = match &yaml["force"] {
            yaml_rust::Yaml::Array(_x) => to_vec_f64::<3>(&yaml["force"]),
            _ => [0.0, 0.0, 0.0],
        };
        atom.name = String::from(yaml["name"].as_str().unwrap());
        atom.mass = match &yaml["mass"] {
            yaml_rust::Yaml::Real(x) => x.parse::<f64>().unwrap(),
            yaml_rust::Yaml::String(x) => x.parse::<f64>().unwrap(),
            yaml_rust::Yaml::Integer(x) => *x as f64,
            yaml_rust::Yaml::BadValue => {
                match periodic_table_on_an_enum::Element::from_symbol(atom.name.as_str()) {
                    Some(x) => x.get_atomic_mass().into(),
                    None => panic!("Failed to find mass for element {}!", atom.name),
                }
            }
            _ => panic!("Failed to find mass for element {}!", atom.name),
        };
        atom.charge = match &yaml["charge"] {
            yaml_rust::Yaml::Integer(x) => *x as f64,
            yaml_rust::Yaml::Real(x) => x.parse::<f64>().unwrap(),
            yaml_rust::Yaml::String(x) => x.parse::<f64>().unwrap(),
            yaml_rust::Yaml::BadValue => 0.0,
            _ => panic!("Incorrect charge for element {}!", atom.name),
        };
        atom
    }
    pub fn clone(&self) -> Atom {
        Atom {
            id: self.id,
            name: self.name.clone(),
            current: CurrentState {
                position: self.current.position,
                velocity: self.current.velocity,
                force: self.current.force,
                potential_energy: self.current.potential_energy,
                kinetic_energy: self.current.kinetic_energy,
                total_energy: self.current.total_energy,
            },
            previous: PreviousState {
                position: self.previous.position,
                velocity: self.previous.velocity,
                force: self.previous.force,
                potential_energy: self.previous.potential_energy,
                kinetic_energy: self.previous.kinetic_energy,
                total_energy: self.previous.total_energy,
            },
            mass: self.mass,
            charge: self.charge,
        }
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} [{:.2}u] @ ({:.3}, {:.3}, {:.3})",
            self.name, self.mass, self.current.position[0], self.current.position[1], self.current.position[2]
        )
    }
}
