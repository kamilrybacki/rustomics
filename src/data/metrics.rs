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
    force: f64,
    pressure: f64,
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
                distance: 1.0,    // Angstroms
                time: 1.0,        // femtoseconds
                mass: 1.0,        // units
                charge: 1.0,      // electron charge
                temperature: 1.0, // Kelvin
                energy: 1.0,      // eV
                force: 1.0,       // Newtons
                pressure: 1.0,    // Pascals
            },
            "SI" => UnitSystem {
                name: String::from("Standard International"),
                distance: 1e10,           // Meters
                time: 1e15,               // Seconds
                mass: 6.0221366516752e26, // Kilograms
                charge: 6.24150975e18,    // Coulombs
                temperature: 1.0,         // Kelvin
                energy: 6.24150907e18,    // Joules,
                force: 1.0,               // Newtons
                pressure: 1.0,            // Pascals
            },
            _ => panic!("Unknown unit system"),
        };

        // Just calculate the rest of the units from the base units
        units.force = units.mass * units.distance / units.time.powi(2);
        units.pressure = units.force / units.distance.powi(2);
        units
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
