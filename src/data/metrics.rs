use yaml_rust::Yaml;

// Each unit system has a name and a set of conversion factors
// to the atomic unit system e.g. distance in meters corresponds
// to 10^10 Angstroms.
pub struct UnitSystem {
    name: String,
    distance: f64,
    time: f64,
    mass: f64,
    charge: f64,
    temperature: f64,
    energy: f64,
}

impl UnitSystem {
    pub fn new(name_yaml_entry: &Yaml) -> UnitSystem {
        let name = match name_yaml_entry {
            Yaml::String(x) => x,
            _ => panic!("Unit system name must be a string"),
        };
        match name.to_uppercase().as_str() {
            "SI" => UnitSystem {
                name: String::from("Standard International"),
                distance: 1e10,
                time: 1.0,
                mass: 1.0,
                charge: 1.0,
                temperature: 1.0,
                energy: 1.0,
            },
            "ATOMIC" => UnitSystem {
                name: String::from("Atomic units"),
                distance: 1.0,
                time: 1.0,
                mass: 1.0,
                charge: 1.0,
                temperature: 1.0,
                energy: 1.0,
            },
            _ => panic!("Unknown unit system"),
        }
    }
}

impl std::fmt::Display for UnitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = 4;
        let unit_system_description = format!(
            "{:indent$}Name: {}\n{:indent$}Distance: {}\n{:indent$}Time: {}\n{:indent$}Mass: {}\n{:indent$}Charge: {}\n{:indent$}Temperature: {}\n{:indent$}Energy: {}",
            "", self.name, "", self.distance, "", self.time, "", self.mass, "", self.charge, "", self.temperature, "", self.energy, indent=indent
        );
        write!(f, "Unit system:\n{}", unit_system_description)
    }
}

impl std::fmt::Debug for UnitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = 4;
        let unit_system_description = format!(
            "{:indent$}Name: {}\n{:indent$}Distance: {}\n{:indent$}Time: {}\n{:indent$}Mass: {}\n{:indent$}Charge: {}\n{:indent$}Temperature: {}\n{:indent$}Energy: {}",
            "", self.name, "", self.distance, "", self.time, "", self.mass, "", self.charge, "", self.temperature, "", self.energy, indent=indent
        );
        write!(f, "Unit system:\n{}", unit_system_description)
    }
}
