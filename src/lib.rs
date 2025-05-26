use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use thiserror::Error;
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

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

//------------------------------------------------------------------------------
const TIME_STEP_SIZE: u64 = 30;

//const LINE_DELIM_RE: LazyCell<Regex> =
//    LazyCell::new(|| Regex::new(r"[^0-9]*\s+|[^0-9]*$").unwrap());

pub fn read_temperature_file(filename: &str) -> Result<Vec<TemperatureLine>, ParseError> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    println!("Reading contenxts of {}", filename);
    read_temperatures(reader)
}

pub fn read_temperatures<R>(reader: R) -> Result<Vec<TemperatureLine>, ParseError>
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

pub struct CoreTemp {
    pub core_id: u8,
    pub points: Vec<TempPoint>,
}

pub struct AllCores(
    pub Vec<TempPoint>,
    pub Vec<TempPoint>,
    pub Vec<TempPoint>,
    pub Vec<TempPoint>,
);

impl AllCores {
    pub fn description(&self) -> String {
        let len = self.0.len();
        let max0 = self.0.iter().map(|p| p.temp_c).fold(f64::MIN, f64::max);
        let max1 = self.1.iter().map(|p| p.temp_c).fold(f64::MIN, f64::max);
        let max2 = self.2.iter().map(|p| p.temp_c).fold(f64::MIN, f64::max);
        let max3 = self.3.iter().map(|p| p.temp_c).fold(f64::MIN, f64::max);
        let avg0 = self
            .0
            .iter()
            .map(|p| p.temp_c)
            .fold(0.0, |n1: f64, n2: f64| (n1 + n2))
            / len as f64;
        let avg1 = self
            .1
            .iter()
            .map(|p| p.temp_c)
            .fold(0.0, |n1: f64, n2: f64| (n1 + n2))
            / len as f64;
        let avg2 = self
            .2
            .iter()
            .map(|p| p.temp_c)
            .fold(0.0, |n1: f64, n2: f64| (n1 + n2))
            / len as f64;
        let avg3 = self
            .3
            .iter()
            .map(|p| p.temp_c)
            .fold(0.0, |n1: f64, n2: f64| (n1 + n2))
            / len as f64;

        let t0avg = self.0.iter().map(|p| p.time_s).fold(0, |n0, n1| n0 + n1) / len as u64;
        let t1avg = self.1.iter().map(|p| p.time_s).fold(1, |n1, n1| n1 + n1) / len as u64;
        let t2avg = self.2.iter().map(|p| p.time_s).fold(2, |n2, n1| n2 + n1) / len as u64;
        let t3avg = self.3.iter().map(|p| p.time_s).fold(3, |n3, n1| n3 + n1) / len as u64;

        format!(
            "Len: {}\nMax temps: {:.2}°C, {:.2}°C, {:.2}°C, {:.2}°C\nAvg temps: {:.2}°C, {:.2}°C, {:.2}°C, {:.2}°C",
            len,
            max0,
            max1,
            max2,
            max3,
            avg0,
            avg1,
            avg2,
            avg3
        )
    }
}

pub fn split_into_cores(orig_data: Vec<TemperatureLine>) -> MyRes<AllCores> {
    let len = orig_data.len();
    let mut all_cores = AllCores(
        Vec::with_capacity(len),
        Vec::with_capacity(len),
        Vec::with_capacity(len),
        Vec::with_capacity(len),
    );
    for line in orig_data {
        all_cores.0.push(TempPoint {
            time_s: line.time_step,
            temp_c: line.readings.0,
        });
        all_cores.1.push(TempPoint {
            time_s: line.time_step,
            temp_c: line.readings.1,
        });
        all_cores.2.push(TempPoint {
            time_s: line.time_step,
            temp_c: line.readings.2,
        });
        all_cores.3.push(TempPoint {
            time_s: line.time_step,
            temp_c: line.readings.3,
        });
    }

    Ok(all_cores)
}
