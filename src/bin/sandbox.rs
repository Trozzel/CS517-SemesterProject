use std::{fs, path::{Path, PathBuf}, str::FromStr};

fn main() -> anyhow::Result<()> {
    let fname = std::env::args()
        .last()
        .expect("Needs file in cmd");

    let fpath = PathBuf::from(&fname);

    let dd = get_new_fname(&fpath, 0)?;

    println!("Is this a success? {}", dd.display());

    Ok(())
}

fn get_new_fname(orig_path: &Path, core_num: usize) -> anyhow::Result<PathBuf> {
    let basepath = match orig_path.parent() {
        Some(parent) => PathBuf::from(parent),
        None => PathBuf::from(""),
    };

    let fname = PathBuf::from(orig_path.file_name().unwrap_or_else(|| panic!("whoa")));

    let new_fname = format!(
        "{}-core-{core_num}.txt",
        fname.file_stem().unwrap().to_str().unwrap()
    );
    Ok(basepath.join(new_fname))
}
