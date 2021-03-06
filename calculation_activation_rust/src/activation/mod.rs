/// * @Process: complete
/// * @Project_Name: module
/// * @Package_Name: calculation_activation_rust
/// * @Made_By: JS
/// * @The_creation_time: --
/// * @File_Name: mod.rs
/// * @Version : rustc 1.57.0-nightly
/// * @contents: -
pub mod activation;

use common::plot;
use rulinalg::matrix::{BaseMatrix, Matrix};

fn test_function(f: fn(Matrix<f32>) -> Matrix<f32>) {
    let data: Vec<_> = (-50..50).map(|n| n as f32 / 10.0).collect();
    let out = f(Matrix::new(1, data.len(), data));
    let graph_data: Vec<_> = out.iter().map(|v| *v).collect();
    plot::print_graph(&graph_data[..], 50, 20);
}

pub fn tests() {
    println!("Step function");
    test_function(activation::step_function);
    println!("Sigmoid");
    test_function(activation::sigmoid);
    println!("ReLU");
    test_function(activation::relu);
}
