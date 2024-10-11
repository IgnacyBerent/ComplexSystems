use std::collections::HashMap;
use std::fs::{self};
use plotters::prelude::*;
mod book_data;
use book_data::book_data::BookData;

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

fn process_words(text: &str) -> BookData {
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
    

    let bd = BookData {
        ranks: rank,
        words: words,
        counts: count,
        frequencies: frequency,
    };
    bd
}

fn find_teoretical_zipflaw_c(rank_count: i64) -> f64 {
    let mut r_sum = 0.0;
    (1..rank_count).for_each(|r| {r_sum += 1_f64 / r as f64});
    let c = 1_f64 / r_sum;
    c   
}

fn process_files_in_folder(folder_path: &str) {
    let paths = fs::read_dir(folder_path).expect("Could not read directory");
    for path in paths {
        let path = path.expect("Could not read path").path();
        if path.is_file() {
            let file_path = path.to_str().unwrap();
            let clear_content = remove_punctuation(&read_bookfile(file_path));
            let bd = process_words(&clear_content);
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let words_sum: i64 = bd.counts.clone().iter().sum();
            let csv_file_path = format!("results/csv/{}_{}.csv", file_name, words_sum);
            bd.save_results(&csv_file_path).unwrap();
            let lin_plots_file_path = format!("results/plots/{}_lin.png", file_name);
            plot_results(&bd, &lin_plots_file_path, false);
            let log_plots_file_path = format!("results/plots/{}_log.png", file_name);
            plot_results(&bd, &log_plots_file_path, true);

        }
    }
}

fn plot_results(bd: &BookData, file_path: &str, in_log: bool) {
    let root = BitMapBackend::new(file_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let rank = bd.ranks.clone();
    let frequency = bd.frequencies.clone();
    let max_rank = rank.iter().max().unwrap() + 1;
    let c = find_teoretical_zipflaw_c(max_rank);
    if in_log {
        let mut chart = ChartBuilder::on(&root)
            .caption("Zipf's Law", ("Arial", 50).into_font())
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d((1..max_rank).log_scale(), (0.00001..1_f64).log_scale()).unwrap();
            chart.configure_mesh()
            .x_desc("Rank log")
            .y_desc("Frequency log")
            .draw().unwrap();
        chart.draw_series(LineSeries::new(
            rank.clone().into_iter().zip(frequency.into_iter()).map(|(x, y)| (x, y)),
            &BLUE,
        )).unwrap().label("freq/rank");
        chart.draw_series(LineSeries::new(
            rank.into_iter().map(|r| (r, c/r as f64)),
            &RED,
        )).unwrap().label("Theoretical Zipf Law");
        chart.configure_series_labels()
            .border_style(&BLACK)
            .label_font(("Arial", 20).into_font())
            .draw().unwrap();
    } else {
        let mut chart = ChartBuilder::on(&root)
            .caption("Zipf's Law", ("Arial", 50).into_font())
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(1..max_rank, 0.00001..0.12).unwrap();
            chart.configure_mesh()
            .x_desc("Rank")
            .y_desc("Frequency")
            .draw().unwrap();
        chart.draw_series(LineSeries::new(
            rank.clone().into_iter().zip(frequency.into_iter()).map(|(x, y)| (x, y)),
            &BLUE,
        )).unwrap().label("freq/rank");
        chart.draw_series(LineSeries::new(
            rank.into_iter().map(|r| (r, c/r as f64)),
            &RED,
        )).unwrap().label("Theoretical Zipf Law");
        chart.configure_series_labels()
            .border_style(&BLACK)
            .label_font(("Arial", 20).into_font())
            .draw().unwrap();
    };
}

fn main() {
    let folder_path = "books";
    process_files_in_folder(folder_path);
}