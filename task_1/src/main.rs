use std::collections::HashMap;
use std::fs::{self};
use plotters::prelude::*;
use polars::prelude::*;

fn read_bookfile(file_path: &str) -> String {
    let contents = std::fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
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

fn process_words(text: &str) -> DataFrame {
    let total_words = text.split_whitespace().count();
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let counter = word_count.entry(word.to_string()).or_insert(0);
        *counter += 1;
    }
    let mut word_count_vec: Vec<(String, i64)> = word_count.into_iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let words = word_count_vec.iter().map(|(x, _)| x.clone()).collect::<Vec<String>>();
    let count = word_count_vec.iter().map(|(_, y)| *y).collect::<Vec<i64>>();
    let mut frequency = word_count_vec.iter().map(|(_, y)| *y as f64 / total_words as f64).collect::<Vec<f64>>();
    let rank = (1..=words.len() as i64).collect::<Vec<i64>>();

    // normalze frequency
    let freq_sum: f64 = frequency.iter().sum();
    frequency = frequency.iter().map(|x| x / freq_sum).collect::<Vec<f64>>();
    

    let df = df!(
        "Rank" => &rank,
        "Word" => &words,
        "Count" => &count,
        "Frequency" => &frequency
    ).unwrap();
    df
}

fn save_results(df: &mut DataFrame, file_path: &str) {
    let mut file = std::fs::File::create(file_path).unwrap();
    CsvWriter::new(&mut file).finish(df).unwrap();
}

fn process_files_in_folder(folder_path: &str) {
    let paths = fs::read_dir(folder_path).expect("Could not read directory");
    for path in paths {
        let path = path.expect("Could not read path").path();
        if path.is_file() {
            let file_path = path.to_str().unwrap();
            let clear_content = remove_punctuation(&read_bookfile(file_path));
            let mut processed_df = process_words(&clear_content);
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let csv_file_path = format!("results/csv/{}.csv", file_name);
            save_results(&mut processed_df, &csv_file_path);            
            let plots_file_path = format!("results/plots/{}.png", file_name);
            plot_results(&processed_df, &plots_file_path);
        }
    }
}

// read each file from the results folder and do zipf's law plot on log scale plot
fn plot_results(df: &DataFrame, file_path: &str) {
    let root = BitMapBackend::new(file_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let rank = df
        .column("Rank").unwrap().i64().unwrap().into_iter().map(|x| x.unwrap()).collect::<Vec<i64>>();
    let frequency = df.column("Frequency").unwrap().f64().unwrap().into_iter().map(|x| x.unwrap()).collect::<Vec<f64>>();
    let max_rank = rank.iter().max().unwrap() + 1;
    let max_frequency = 1.0;
    let freq_sum: f64 = frequency.iter().sum();
    println!("Sum of frequencies: {}", freq_sum);
    let mut chart = ChartBuilder::on(&root)
        .caption("Zipf's Law", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d((1..max_rank).log_scale(), (0.00001..max_frequency).log_scale()).unwrap();
    chart.configure_mesh().draw().unwrap();
    chart.draw_series(LineSeries::new(
        rank.into_iter().zip(frequency.into_iter()).map(|(x, y)| (x, y)),
        &RED,
    )).unwrap();

}

fn main() {
    let folder_path = "books";
    process_files_in_folder(folder_path);
}