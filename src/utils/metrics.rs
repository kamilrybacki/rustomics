use yaml_rust::Yaml;

#[derive(Debug)]
pub struct UnitSystem {
    pub name: String,
    pub distance: (f64, String),
    pub time: (f64, String),
    pub mass: (f64, String),
    pub charge: (f64, String),
    pub temperature: (f64, String),
    pub energy: (f64, String),
    pub force: (f64, String),
    pub pressure: (f64, String),
}

impl UnitSystem {
    pub fn new(name_yaml_entry: &Yaml) -> UnitSystem {
        let name = match name_yaml_entry {
            Yaml::String(x) => x,
            _ => panic!("Unit system name must be a string"),
        };
        let mut units = match name.to_uppercase().as_str() {
            // Atomic is the reference unit system here
            "ATOMIC" => UnitSystem {
                name: String::from("Atomic units"),
                distance: (1e-10, String::from("Ang.")),
                time: (1e-9, String::from("ns")),
                mass: (1.66053907e-27, String::from("amu")),
                charge: (1.60217663e-19, String::from("e")),
                temperature: (1.0, String::from("K")),
                energy: (1.602176634e-19, String::from("eV")),
                force: (1.0, String::from("N")),
                pressure: (1.0, String::from("Pa")),
            },
            "SI" => UnitSystem {
                name: String::from("Standard International"),
                distance: (1.0, String::from("m")),
                time: (1.0, String::from("s")),
                mass: (1.0, String::from("kg")),
                charge: (1.0, String::from("C")),
                temperature: (1.0, String::from("K")),
                energy: (1.0, String::from("J")),
                force: (1.0, String::from("N")),
                pressure: (1.0, String::from("Pa")),
            },
            _ => panic!("Unknown unit system"),
        };

        // Just calculate the rest of the units from the base units
        units.force.0 = units.mass.0 * units.distance.0 / units.time.0.powi(2);
        units.pressure.0 = units.force.0 / units.distance.0.powi(2);
        units
    }
}

impl std::fmt::Display for UnitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = 4;
        let unit_system_description = format!(
            "{:indent$}Name: {}\n{:indent$}Distance: {}\n{:indent$}Time: {}\n{:indent$}Mass: {}\n{:indent$}Charge: {}\n{:indent$}Temperature: {}\n{:indent$}Energy: {}",
            "", self.name, "", self.distance.1, "", self.time.1, "", self.mass.1, "", self.charge.1, "", self.temperature.1, "", self.energy.1, indent=indent
        );
        write!(f, "Unit system:\n{}", unit_system_description)
    }
}
