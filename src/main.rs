use std::path::PathBuf;
use std::fs;
use std::io::Read;
use std::str::FromStr;

use temperature_parser::temp_mat::{TempMat, write_temp_output};

const TIME_STEP_SIZE: f64 = 30.0;
const NUM_CORES: usize = 4;

fn main() -> anyhow::Result<()> {
    let fname = std::env::args()
        .last()
        .expect("Usage: cargo run <filename>");

    let mut content = String::new();
    fs::File::open(&fname)?.read_to_string(&mut content)?;
    let tm = TempMat::<NUM_CORES>::from_str(content.as_str())?;

    println!("Shape of original: {}", tm.shape());

    let interp = tm.interp(TIME_STEP_SIZE);
    println!("Shape of interpolated: {}", interp.shape());

    let fpath = PathBuf::from(fname);
    write_temp_output(&tm, &interp, TIME_STEP_SIZE, &fpath)?;

    Ok(())
}

