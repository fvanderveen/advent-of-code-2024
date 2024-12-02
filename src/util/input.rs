use std::fs::{read_to_string, exists};

pub fn read_input(day: i32) -> Result<String, String> {
    let input_path = format!("resources/day{:02}.txt", day);

    match exists(&input_path) {
        Ok(_) => read_to_string(&input_path).map_err(|e| format!("{}", e)),
        Err(_) => Err(format!("Input for day {} not found in resources directory!", day)),
    }
}
