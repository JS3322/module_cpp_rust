/// * @Process: complete
/// * @Project_Name: module
/// * @Package_Name: calculation_activation_rust
/// * @Made_By: JS
/// * @The_creation_time: --
/// * @File_Name: activation.rs
/// * @Version : rustc 1.57.0-nightly
/// * @contents: -
use rulinalg::matrix::{Axes, BaseMatrix, BaseMatrixMut, Matrix};

pub fn step_function(x: Matrix<f32>) -> Matrix<f32> {
    x.apply(&|value| if value > 0.0 { 1.0 } else { 0.0 })
}

pub fn sigmoid(x: Matrix<f32>) -> Matrix<f32> {
    x.apply(&|value| 1.0 / (1.0 + (-value).exp()))
}

pub fn relu(x: Matrix<f32>) -> Matrix<f32> {
    x.apply(&|value| if value > 0.0 { value } else { 0.0 })
}

pub fn softmax(mut x: Matrix<f32>) -> Matrix<f32> {
    for mut row in x.row_iter_mut() {
        let max = row.max(Axes::Row)[0];
        let mut sum_exp = 0.0;
        for val in row.iter_mut() {
            *val = (*val - max).exp();
            sum_exp += *val;
        }
        for val in row.iter_mut() {
            *val /= sum_exp
        }
    }
    x
}
