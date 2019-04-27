include!(concat!(env!("OUT_DIR"), "/hazedumper.rs"));

pub static HAZEDUMPER: self::HazeDumper = self::HazeDumper::new();
