pub mod nve;

pub enum Ensemble {
    NVE(nve::NVE),
}

impl Ensemble {
    pub fn from(yaml: &yaml_rust::Yaml) -> Ensemble {
        match yaml["type"].as_str().unwrap() {
            "nve" => Ensemble::NVE(nve::NVE::from(&yaml)),
            _ => panic!("Unknown ensemble type"),
        }
    }
}
