use std::fmt::Display;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::usize;

use crate::error::TMFromStrError;
use crate::INTERP;

#[derive(Debug)]
pub struct TempMat<const N: usize>([Vec<f64>; N]);

/// TEMPMAT - Deref<N>
/// Allow direct access to the the array of vectors without need to use:
/// Example:
/// ```rust
/// let td = TempData::from(content);
/// let vec0 = td[0] // as opposed to td.0[0]
/// ```
impl<const N: usize> Deref for TempMat<N> {
    type Target = [Vec<f64>; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// TEMPMAT::FromStr
/// Allow formation of `TempMat` from a string slice.
/// In practice, the string slice should be a whitespace-separated list of
/// numbers, that derives from a file read.
impl<const N: usize> FromStr for TempMat<N> {
    type Err = TMFromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this: [Vec<f64>; N] = std::array::from_fn(|_| Vec::new());
        //  Push all numbers into a 1D Vec
        let data_row: Vec<f64> = s
            .split_whitespace()
            .map(|numstr| {
                numstr
                    .parse::<f64>()
                    .expect("Error parsing `&str` to `f64`")
            })
            .collect();
        // Length of `data_row` must have N rows
        if data_row.len() % N != 0 {
            return Err(TMFromStrError::InvalidDimensions { col: N });
        }
        assert_eq!(data_row.len() % N, 0);
        // TODO: Is there a better way than O(n^2)
        let mut iter = data_row.iter();
        while let Some(num) = iter.next() {
            for i in 0..N {
                this[i].push(num.clone());
            }
        }
        Ok(TempMat(this))
    }
}

pub struct Dims(usize, usize);

impl Display for Dims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl PartialEq for Dims {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0) && self.1.eq(&other.1)
    }
    fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0) && self.1.ne(&other.1)
    }
}

impl<const N: usize> TempMat<N> {
    pub fn new() -> Self {
        TempMat(std::array::from_fn(|_| Vec::new()))
    }

    pub fn shape(&self) -> Dims {
        Dims(N, self.0[0].len())
    }

    pub fn interp(&self, dt: f64) -> Self {
        let mut this: [Vec<f64>; N] = std::array::from_fn(|_| Vec::new());
        for i in 0..N {
            this[i] = self.0[i]
                .windows(2)
                .map(|n| INTERP(n[0], n[1], dt))
                .collect()
        }
        TempMat(this)
    }
}

/// Create data file
pub fn write_temp_output<const N: usize>(
    orig: &TempMat<N>,
    interp: &TempMat<N>,
    dt: f64,
    orig_file: &Path,
) -> anyhow::Result<()> {
    assert_eq!(orig.shape().1, interp.shape().1 + 1);

    // Create interpolated `TempMat`
    for core_num in 0..N {
        // Create file path string
        let mut f = std::fs::File::create(get_new_fname(orig_file, core_num)?)?;

        // Get string and write it to file
        for i in 0..(interp[0].len()) {
            let t0 = i as f64 * dt;
            f.write(
                create_modline(orig[core_num][i], interp[core_num][i], t0, t0 + dt).as_bytes(),
            )?;
        }
    }

    Ok(())
}

// Create the file name for new contents
// TODO: rework the logic of creating the modified files names.
// The base path should be taken once instead of N times
fn get_new_fname(orig_path: &Path, core_num: usize) -> anyhow::Result<PathBuf> {
    let basepath = match orig_path.parent() {
        Some(parent) => PathBuf::from(parent),
        None => PathBuf::from(""),
    };

    let fname = PathBuf::from(orig_path.file_name().unwrap_or_else(|| panic!("whoa")));

    let new_fname = format!(
        "{}-core-{}.txt",
        fname.file_stem().unwrap().to_str().unwrap(),
        core_num
    );
    Ok(basepath.join(new_fname))
}

/// Create modified line
fn create_modline(y0: f64, interp: f64, t0: f64, t1: f64) -> String {
    format!(
        "{:>5} <= x <= {:>10} ; y = {:>8.3} + {:>8.3} x ; interpolation\n",
        t0, t1, y0, interp
    )
}
