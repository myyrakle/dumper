use crate::command::clean::ConfigOptions;

pub fn run(option: ConfigOptions) {
    let base_path = option.path;

    let file_list = std::fs::read_dir(&base_path).unwrap();

    for file in file_list {
        let file = file.unwrap();
        let file_path = file.path();

        if file_path.extension().unwrap_or_default() == "trashfile" {
            println!("Removing file: {:?}", file_path);
            std::fs::remove_file(file_path).unwrap();
        }
    }
}
