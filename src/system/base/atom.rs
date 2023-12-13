use crate::data::load::to_vec_f64;

pub struct Atom {
  position: [f64; 3],
  velocity: [f64; 3],
  force: [f64; 3],
  mass: f64,
  charge: f64,
  name: String
}

impl Atom {
  pub fn new() -> Atom {
    Atom {
      position: [0.0, 0.0, 0.0],
      velocity: [0.0, 0.0, 0.0],
      force: [0.0, 0.0, 0.0],
      mass: 1.0,
      charge: 0.0,
      name: String::from("NaN")
    }
  }
  pub fn from(yaml: &yaml_rust::Yaml) -> Atom {
    let mut atom = Atom::new();
    atom.position = to_vec_f64::<3>(&yaml["position"]);
    atom.velocity = to_vec_f64::<3>(&yaml["velocity"]);
    atom.force = to_vec_f64::<3>(&yaml["force"]);
    atom.mass = yaml["mass"].as_f64().unwrap();
    atom.charge = match yaml["charge"].as_f64() {
      Some(x) => x,
      None => 0.0
    };
    atom.name = yaml["name"].as_str().unwrap().to_string();
    atom
  }
}