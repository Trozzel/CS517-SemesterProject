pub mod error;
pub mod temp_mat;

pub const INTERP: fn(f64, f64, f64) -> f64 = |n1, n2, dt| -> f64 { (n2 - n1) / dt };

