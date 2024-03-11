use std::{io::Write, path::PathBuf};

use crate::command::dump::ConfigOptions;

use uuid::Uuid;

pub fn run(option: ConfigOptions) {
    let byte_size = if option.size.ends_with('g') {
        let giga_size = option.size.trim_end_matches('g').parse::<u64>().unwrap();

        giga_size * 1024 * 1024 * 1024
    } else if option.size.ends_with('m') {
        let mega_size = option.size.trim_end_matches('m').parse::<u64>().unwrap();

        mega_size * 1024 * 1024
    } else if option.size.ends_with('k') {
        let killo_size = option.size.trim_end_matches('k').parse::<u64>().unwrap();

        killo_size * 1024
    } else {
        option.size.parse::<u64>().unwrap()
    };

    let count = option.count.unwrap_or(1);

    let base_path = option.path;

    for _ in 0..count {
        let mut trash_path = PathBuf::new();
        trash_path.push(&base_path);

        let filename = format!("{}.trashfile", Uuid::new_v4());
        trash_path.push(filename);

        println!("Creating file: {:?}", trash_path);

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&trash_path)
            .unwrap();

        file.set_len(byte_size).unwrap();

        let trash = vec![1; byte_size as usize];
        file.write_all(trash.as_slice()).unwrap();
    }
}
