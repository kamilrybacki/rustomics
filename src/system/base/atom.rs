extern crate periodic_table_on_an_enum;

use crate::data::load::to_vec_f64;

pub struct Atom {
    pub id: u64,
    pub name: String,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub force: [f64; 3],
    pub mass: f64,
    pub charge: f64,
    pub potential_energy: f64,
}

impl Atom {
    pub fn new() -> Atom {
        Atom {
            id: 0,
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            force: [0.0, 0.0, 0.0],
            mass: 1.0,
            charge: 0.0,
            potential_energy: 0.0,
            name: String::from("NaN"),
        }
    }
    pub fn from(yaml: &yaml_rust::Yaml) -> Atom {
        let mut atom = Atom::new();
        atom.position = to_vec_f64::<3>(&yaml["position"]);
        atom.velocity = match &yaml["velocity"] {
            yaml_rust::Yaml::Array(_x) => to_vec_f64::<3>(&yaml["velocity"]),
            _ => [0.0, 0.0, 0.0],
        };
        atom.force = match &yaml["force"] {
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
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} [{:.2}u] @ ({:.3}, {:.3}, {:.3})",
            self.name, self.mass, self.position[0], self.position[1], self.position[2]
        )
    }
}
