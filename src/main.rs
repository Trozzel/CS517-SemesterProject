use temperature_parser::{read_temperature_file, MyRes, split_into_cores};

fn main() -> MyRes<()> {
    let fname = std::env::args()
        .nth(3)
        .unwrap_or("Data/sensors-2019.01.26-no-labels.txt".to_string());

    println!("Full path: {:?}", std::path::absolute(&fname));

    let _data = read_temperature_file(fname.as_str())?;

    let all_cores = split_into_cores(_data)?;

    for data in &all_cores.0 {
        println!("{:?}", data);
    }

    println!("{}", all_cores.description());

    Ok(())
}

