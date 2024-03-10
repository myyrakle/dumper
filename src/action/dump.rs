use crate::command::dump::ConfigOptions;

pub fn run(option: ConfigOptions) {
    let byte_size = if option.size.ends_with("g") {
        let giga_size = option.size.trim_end_matches("g").parse::<u64>().unwrap();

        giga_size * 1024 * 1024 * 1024
    } else if option.size.ends_with("m") {
        let mega_size = option.size.trim_end_matches("m").parse::<u64>().unwrap();

        mega_size * 1024 * 1024
    } else if option.size.ends_with("k") {
        let killo_size = option.size.trim_end_matches("k").parse::<u64>().unwrap();

        killo_size * 1024
    } else {
        option.size.parse::<u64>().unwrap()
    };
}
