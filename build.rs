use std::{env, path::Path, fs::File, io::Write as _};

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let haze_path = Path::new(&outdir).join("hazedumper.rs");
    let mut haze_file = File::create(&haze_path).unwrap();

    let to_write = include_str!("build-src/hazedumper.rs");
    let to_write = to_write.replace("{toml}", include_str!("build-src/hazedumper_csgo.toml"));
    haze_file.write_all(to_write.as_bytes()).unwrap();
}
