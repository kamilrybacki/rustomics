pub fn euclidean_norm(vector: &[f64]) -> f64 {
    vector.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
}
