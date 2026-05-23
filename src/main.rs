use std::{collections::HashMap, fs::File, thread::available_parallelism};

use memmap2::Mmap;

struct WeatherData {
    min: i16,
    max: i16,
    total: i64,
    count: usize,
}

fn main() {
    let file = File::open("1000.txt").expect("failed to open file");
    let mmap = unsafe { Mmap::map(&file).expect("failed to memory map file") };
    let chunks: Vec<(usize, usize)> = get_chunks(&mmap).expect("unable to read file");

    let mut hashmaps = Vec::<HashMap<&[u8], WeatherData>>::new();

    crossbeam::scope(|s| {
        let mut handles = vec![];

        for chunk in chunks.iter() {
            handles.push(s.spawn(|_| process_data(&mmap, *chunk).expect("unable to process file")));
        }

        handles.into_iter().for_each(|h| {
            hashmaps.push(h.join().expect("unable to join thread"));
        });
    })
    .expect("unable to spawn threads");

    output_data(hashmaps);
}

fn get_chunks(mmap: &Mmap) -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    let thread_count = available_parallelism().expect("unable to get thread count");
    let chunk_size_estimate = mmap.len() / thread_count;

    let mut chunks = vec![];
    let mut start: usize = 0;
    let mut end: usize = chunk_size_estimate;
    while end < mmap.len() {
        while end < mmap.len() && mmap[end] != b'\n' {
            end += 1;
        }
        chunks.push((start, end));
        start = end + 1;
        end = start + chunk_size_estimate;
    }
    if start < mmap.len() {
        chunks.push((start, mmap.len()));
    }
    Ok(chunks)
}

fn process_data(
    mmap: &Mmap,
    chunk: (usize, usize),
) -> Result<HashMap<&[u8], WeatherData>, Box<dyn std::error::Error>> {
    let mut map = HashMap::<&[u8], WeatherData>::new();

    let mut cursor = chunk.0;
    let mut line_start = cursor;
    let mut line_pivot = cursor;

    let mut accumulator = 0;
    let mut is_negative = false;

    let mut key: Option<&[u8]> = None;

    while cursor < chunk.1 {
        // Key is the first part of the line
        if mmap[cursor] == b';' {
            line_pivot = cursor;
            key = Some(&mmap[line_start..line_pivot]);
            if mmap[cursor + 1] == b'-' {
                is_negative = true;
            }
        }
        // end of the line
        else if mmap[cursor] == b'\n' && key.is_some() {
            if is_negative {
                accumulator = -accumulator;
            }

            map.entry(key.unwrap())
                .and_modify(|e| {
                    e.min = e.min.min(accumulator);
                    e.max = e.max.max(accumulator);
                    e.total += accumulator as i64;
                    e.count += 1;
                })
                .or_insert(WeatherData {
                    min: accumulator,
                    max: accumulator,
                    total: accumulator as i64,
                    count: 1,
                });

            key = None;
            line_start = cursor + 1;
            line_pivot = line_start;
            accumulator = 0;
            is_negative = false;
        }
        // process the value in flight
        else if line_pivot != line_start {
            if mmap[cursor] >= b'0' && mmap[cursor] <= b'9' {
                accumulator = accumulator * 10 + (mmap[cursor] - b'0') as i16;
            }
        }

        cursor += 1;
    }

    Ok(map)
}

fn output_data(data: Vec<HashMap<&[u8], WeatherData>>) {
    let mut combined_map = HashMap::<&[u8], WeatherData>::new();

    for map in data.iter() {
        for (key, value) in map.iter() {
            combined_map
                .entry(*key)
                .and_modify(|e| {
                    e.min = e.min.min(value.min);
                    e.max = e.max.max(value.max);
                    e.total += value.total;
                    e.count += value.count;
                })
                .or_insert(WeatherData {
                    min: value.min,
                    max: value.max,
                    total: value.total,
                    count: value.count,
                });
        }
    }

    let mut keys = combined_map.keys().cloned().collect::<Vec<&[u8]>>();
    keys.sort();

    for key in keys.iter() {
        let value = combined_map.get(key).unwrap();
        println!(
            "{};{};{};{}",
            std::str::from_utf8(key).unwrap(),
            value.min as f64 / 10.0,
            value.max as f64 / 10.0,
            value.total / value.count as i64
        );
    }
}
