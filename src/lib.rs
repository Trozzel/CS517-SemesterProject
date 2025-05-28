use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::usize;
pub type MyErr = Box<dyn std::error::Error + Sync + Send + 'static>;
pub type MyRes<T> = Result<T, MyErr>;

#[derive(Debug)]
pub struct TempRow(pub f64, pub f64, pub f64, pub f64);

impl From<String> for TempRow {
    fn from(value: String) -> Self {
        let mut iter = value
            .split_whitespace()
            .map(|numstr| numstr.parse::<f64>().expect("Error parsing &str to f64"));
        TempRow(
            iter.next().expect("Error parsing from str to f64"),
            iter.next().expect("Error parsing from str to f64"),
            iter.next().expect("Error parsing from str to f64"),
            iter.next().expect("Error parsing from str to f64"),
        )
    }
}

#[derive(Debug)]
pub struct TemperatureLine {
    pub time_step: u64,
    pub readings: TempRow,
}

//------------------------------------------------------------------------------
const TIME_STEP_SIZE: u64 = 30;

pub fn read_temperature_file(filename: &str) -> MyRes<Vec<TemperatureLine>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    println!("Reading contenxts of {}", filename);
    read_temperatures(reader)
}

pub fn read_temperatures<R>(reader: R) -> MyRes<Vec<TemperatureLine>>
where
    R: BufRead,
{
    let mut readings: Vec<TemperatureLine> = Vec::new();

    for (idx, wrapped_line) in reader.lines().enumerate() {
        let line = wrapped_line?;
        let time = (idx as u64) * TIME_STEP_SIZE;

        let new_reading = TemperatureLine {
            time_step: time,
            readings: TempRow::from(line),
        };

        readings.push(new_reading);
    }

    Ok(readings)
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct TempPoint {
    pub time_s: u64,
    pub temp_c: f64,
}

impl Display for TempPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time: {}s, Temp: {:.2}°C", self.time_s, self.temp_c)
    }
}

// CORE TEMPS
//****************************************************************************//
pub struct CoreTemps<const N: usize>([Vec<TempPoint>; N]);

impl<const N: usize> CoreTemps<N> {
    pub fn new() -> Self {
        CoreTemps(std::array::from_fn(|_| Vec::new()))
    }

    pub fn get_vec(&self, idx: usize) -> MyRes<&Vec<TempPoint>> {
        Ok(&self.get_vecs()[idx])
    }

    pub fn get_mut_vec(&mut self, idx: usize) -> MyRes<&Vec<TempPoint>> {
        Ok(&mut self.0[idx])
    }

    pub fn get_vecs(&self) -> &[Vec<TempPoint>] {
        &self.0
    }

    pub fn get_mut_vecs(&mut self) -> &mut [Vec<TempPoint>] {
        &mut self.0
    }

    pub fn size(&self) -> MyRes<(usize, usize)> {
        Ok((N, self.get_vec(0)?.len()))
    }

    pub fn describe(&self) -> MyRes<String> {
        Ok(format!("Size: ({}x{})", self.size()?.0, self.size()?.1))
    }
}

pub fn split_into_cores<const N: usize>(orig_data: Vec<TemperatureLine>) -> MyRes<CoreTemps<N>> {
    let mut ncore_temps = CoreTemps::<N>::new();

    for line in orig_data {
        for core in ncore_temps.get_mut_vecs() {
            core.push(TempPoint {
                time_s: line.time_step,
                temp_c: line.readings.0,
            });
        }
    }

    Ok(ncore_temps)
}

pub fn interpolate(arr: &[TempPoint]) -> Vec<f64> {
    let f_interp: Box<dyn Fn(&[TempPoint]) -> f64> = Box::new(|tpt: &[TempPoint]| -> f64 {
        (tpt[1].temp_c - tpt[0].temp_c) / (tpt[1].time_s as f64 - tpt[0].time_s as f64)
    });

    arr.windows(2).map(|tpt| f_interp(tpt)).collect()
}
