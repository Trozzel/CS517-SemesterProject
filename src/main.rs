use temperature_parser::{read_temperature_file, MyRes, split_into_cores, interpolate};

fn main() -> MyRes<()> {
    let fname = std::env::args()
        .nth(3)
        .unwrap_or("Data/sensors-2018.12.26-no-labels.txt".to_string());

    println!("The file: {}", &fname);
    println!("Full path: {:?}", std::path::absolute(&fname));

    let data = read_temperature_file(fname.as_str())?;

    let all_cores = split_into_cores::<4>(data)?;

    println!("INTERPOLATING...");

    let interp = interpolate(all_cores.get_vec(0).unwrap().as_slice());

    println!("all_cores.len(): {}, interp.len(): {}", all_cores.size().unwrap().1, interp.len());

    Ok(())
}

