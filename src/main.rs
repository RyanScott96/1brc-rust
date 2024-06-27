use std::collections::HashMap;

struct WeatherData {
    _min: f32,
    _max: f32,
    _total: f32,
    _count: u32,
}

fn main() {
    let text: Vec<String> =
        read_file(String::from("measurements.txt")).expect("unable to read file");
    let data: HashMap<String, WeatherData> = process_data(text).expect("unable to process file");
    output_data(data);
}

fn read_file(_path: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(vec![])
}

fn process_data(
    _text: Vec<String>,
) -> Result<HashMap<String, WeatherData>, Box<dyn std::error::Error>> {
    Ok(HashMap::<String, WeatherData>::new())
}

fn output_data(_data: HashMap<String, WeatherData>) {}
