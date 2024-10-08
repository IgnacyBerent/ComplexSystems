use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use plotters::prelude::*;
use polars::prelude::*;
use std::path::PathBuf;

fn read_bookfile(file_path: &str) -> String {
    let contents = std::fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    // content now is in form of lines of text but I want it to be a one line string
    let mut one_line_content = String::new();
    for line in contents.lines() {
        one_line_content.push_str(line);
        one_line_content.push_str(" ");
    }
    one_line_content
}

fn remove_punctuation(text: &str) -> String {
    let mut cleaned_text = String::new();
    for c in text.chars() {
        if c.is_alphabetic() || c.is_whitespace() {
            cleaned_text.push(c);
        }
    }
    cleaned_text.to_ascii_lowercase()
}

fn count_words(text: &str) -> Vec<(usize, String, usize, f32)> {
    let total_words = text.split_whitespace().count();
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let counter = word_count.entry(word.to_string()).or_insert(0);
        *counter += 1;
    }
    let mut word_count_vec: Vec<(String, usize)> = word_count.into_iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let mut rank = 1;
    let mut final_vec: Vec<(usize, String, usize, f32)> = Vec::new();
    for (word, count) in word_count_vec {
        let frequency = count as f32 / total_words as f32;
        final_vec.push((rank, word, count, frequency));
        rank += 1;
    }
    final_vec
}

fn save_results(results: &Vec<(usize, String, usize, f32)>, file_name: &str) {
    let file_path = format!("results/{}.csv", file_name);
    let mut file = File::create(file_path).expect("Could not create file");
    file.write_all("Rank, Word, Count, Frequency\n".as_bytes()).expect("Could not write to file");
    for (rank, word, count, frequency) in results {
        let line = format!("{}, {}, {}, {}\n", rank, word, count, frequency);
        file.write_all(line.as_bytes()).expect("Could not write to file");
    }
}

fn process_files_in_folder(folder_path: &str) {
    let paths = fs::read_dir(folder_path).expect("Could not read directory");
    for path in paths {
        let path = path.expect("Could not read path").path();
        if path.is_file() {
            let file_path = path.to_str().unwrap();
            let clear_content = remove_punctuation(&read_bookfile(file_path));
            let sorted_word_count = count_words(&clear_content);
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            save_results(&sorted_word_count, file_name);
        }
    }
}

fn read_csv(file_path: PathBuf) -> DataFrame {
    CsvReadOptions::with_has_header(CsvReadOptions::default(), true)
    .try_into_reader_with_file_path(Some(file_path))
    .unwrap()
    .finish()
    .unwrap()
}

// read each file from the results folder and do zipf's law plot
fn plot_results(folder_path: &str) {
    let paths = fs::read_dir(folder_path).expect("Could not read directory");
    for path in paths {
        let path = path.expect("Could not read path").path();
        if path.is_file() {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let df = read_csv(path);
            let rank = df.column("Rank").unwrap().i32().unwrap();
            let frequency = df.column("Frequency").unwrap().f32().unwrap();
            let destination_path = format!("results/{}.png", file_name);
            let root = BitMapBackend::new(&destination_path, (800, 600)).into_drawing_area();
            root.fill(&WHITE).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .caption(file_name, ("Arial", 30).into_font())
                .margin(5)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_2d(0..rank.len() as i32, 0.0..1.0).unwrap();
            chart.configure_mesh().draw().unwrap();
            chart.draw_series(LineSeries::new(
                rank.iter().zip(frequency.iter()).map(|(x, y)| (*x, *y)),
                &RED,
            )).unwrap();
        }
    }
}

fn main() {
    let folder_path = "books";
    process_files_in_folder(folder_path);
    plot_results("results");
}