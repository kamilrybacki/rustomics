use nalgebra::Matrix3;
use nalgebra::Vector3;

use crate::system::cell::UnitCell;

#[derive(Debug)]
pub struct SimulationBox {
    pub cell: UnitCell,
    pub vectors: Matrix3<f64>,                // Simulation box vectors
    pub versors: Matrix3<f64>,                // Simulation box versors
    pub dimensions: Vector3<f64>,             // Dimensions of the simulation box
    pub replicas: [usize; 3],                 // Number of replicas in each direction
    pub periodicity: [bool; 3],               // Periodicity of the simulation box
    pub change_of_basis_matrix: Matrix3<f64>, // Matrix mapping between global coordinates and simulation box coordinates
}

impl SimulationBox {
    pub fn new(
        vectors: Matrix3<f64>,
        periodicity: [bool; 3],
        replicas: [usize; 3],
    ) -> SimulationBox {
        let mut new_box = SimulationBox {
            cell: UnitCell::new(vectors),
            vectors: Matrix3::zeros(),
            versors: Matrix3::zeros(),
            dimensions: Vector3::zeros(),
            replicas,
            periodicity,
            change_of_basis_matrix: Matrix3::zeros(),
        };
        new_box.calculate_box_vectors();
        new_box.calculate_mapping_matrix();
        new_box
    }

    fn calculate_box_vectors(&mut self) -> () {
        let mut new_vectors = self.cell.vectors.clone();
        for i in 0..3 {
            new_vectors[(i, i)] *= self.replicas[i] as f64;
        }
        let new_dimensions: Vector3<f64> = Vector3::new(
            new_vectors.row(0).norm(),
            new_vectors.row(1).norm(),
            new_vectors.row(2).norm(),
        );
        let mut new_versors = Matrix3::<f64>::zeros();
        for i in 0..3 {
            for j in 0..3 {
                new_versors[(i, j)] = new_vectors[(i, j)] * (1.0 / new_dimensions[i]);
            }
        }
        self.vectors = new_vectors;
        self.dimensions = new_dimensions;
        self.versors = new_versors;
    }

    fn calculate_mapping_matrix(&mut self) -> () {
        let change_of_basis_matrix = self.vectors.normalize();
        match change_of_basis_matrix.try_inverse() {
            Some(_) => self.change_of_basis_matrix = change_of_basis_matrix,
            None => panic!("Could not invert change of basis matrix"),
        }
    }

    pub fn wrap_position(&self, position: Vector3<f64>) -> Vector3<f64> {
        let wrapped_position = position.clone();
        wrapped_position
    }

    pub fn map_vector_to_box_basis(&self, vector: &Vector3<f64>) -> Vector3<f64> {
        self.change_of_basis_matrix * vector
    }

    pub fn map_vector_to_system_basis(&self, vector: &Vector3<f64>) -> Vector3<f64> {
        self.change_of_basis_matrix.try_inverse().unwrap() * vector
    }
}

impl std::fmt::Display for SimulationBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let definition = format!(
            "Vectors: {:?}\n  Replicas: {:?}\n  Periodicity: {:?}\n",
            self.vectors, self.replicas, self.periodicity
        );
        write!(f, "{}", definition)
    }
}
