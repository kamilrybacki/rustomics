use nalgebra::Matrix3;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct UnitCell {
    pub vectors: Matrix3<f64>,   // Simulation box vectors
    pub constants: Vector3<f64>, // Unit cell lattice constants
    pub volume: f64,                       // Volume of the unit cell
}

impl UnitCell {
    pub fn new(vectors: Matrix3<f64>) -> UnitCell {
        let mut initialized_cell = UnitCell {
            vectors,
            constants: Vector3::zeros(),
            volume: 0.0,
        };
        initialized_cell.constants = Vector3::new(
            vectors.row(0).norm(),
            vectors.row(1).norm(),
            vectors.row(2).norm(),
        );
        initialized_cell.volume = vectors.determinant();
        initialized_cell
    }
}

impl std::fmt::Display for UnitCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let definition = format!("\n  Vectors: {:?}\n ", self.vectors);
        write!(f, "{}", definition)
    }
}
