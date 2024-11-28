use plotly::common::{Mode, Title};
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};
use rand::Rng;
use std::collections::HashMap;

use std::collections::VecDeque;

struct FixedSizeDeque<T> {
    deque: VecDeque<T>,
    max_size: usize,
}

impl<T> FixedSizeDeque<T> {
    fn new(max_size: usize) -> Self {
        Self {
            deque: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    fn push(&mut self, value: T) {
        if self.deque.len() == self.max_size {
            self.deque.pop_front();
        }
        self.deque.push_back(value);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.deque.pop_front()
    }

    fn len(&self) -> usize {
        self.deque.len()
    }
}

struct OsloModel {
    size: u32,
    slopes: Vec<u32>,
    tresholds: Vec<u8>,
}

impl OsloModel {
    fn new(size: u32) -> OsloModel {
        let mut rng = rand::thread_rng();

        // Initialize system in stable configuration where zi < ziT for all i
        let slopes = (0..size).map(|_| rng.gen_range(0..=1)).collect();
        let tresholds = (0..size).map(|_| rng.gen_range(1..=2)).collect();

        OsloModel {
            size,
            slopes,
            tresholds,
        }
    }

    fn drive(&mut self) {
        // Adding grain to the first site increases the slope by 1
        self.slopes[0] += 1;
    }

    fn relax(&mut self) -> (u32, u32) {
        let mut rng = rand::thread_rng();
        let mut s = 0;
        let mut efflux = 0;
        loop {
            let mut moved = false;
            for ii in 0..self.size {
                let i: usize = ii as usize;
                if self.slopes[i] > self.tresholds[i] as u32 {
                    if i == 0 {
                        self.slopes[i] -= 2;
                        self.slopes[i + 1] += 1;
                    } else if ii == self.size - 1 {
                        self.slopes[i] -= 1;
                        self.slopes[i - 1] += 1;
                        efflux += 1;
                    } else {
                        self.slopes[i] -= 2;
                        self.slopes[i + 1] += 1;
                        self.slopes[i - 1] += 1;
                    }
                    // Choose new treshold
                    self.tresholds[i] = rng.gen_range(1..=2);
                    s += 1;
                    moved = true;
                }
            }
            if !moved {
                break;
            }
        }
        return (s, efflux);
    }

    fn run(&mut self, n: u32) -> Vec<u32> {
        let mut sizes = vec![0];
        for _ in 0..n {
            self.drive();
            let (s, _) = self.relax();
            sizes.push(s);
        }
        sizes
    }

    fn run_with_treshold(&mut self, n: u32, treshold: f32) -> Vec<u32> {
        let mut sizes = vec![0];
        let mut influx_window: FixedSizeDeque<u32> = FixedSizeDeque::new(100);
        let mut efflux_window: FixedSizeDeque<u32> = FixedSizeDeque::new(100);
        let mut t = 0;
        loop {
            self.drive();
            let (_, e) = self.relax();
            efflux_window.push(e);
            influx_window.push(1);
            if influx_window.len() == 100 {
                let influx: u32 = influx_window.deque.iter().sum();
                let efflux: u32 = efflux_window.deque.iter().sum();
                if efflux as f32 / influx as f32 > treshold {
                    println!("Treshold reached at t = {}", t);
                    break;
                }
            }
            t += 1;
        }
        for _ in 0..n {
            self.drive();
            let (s, _) = self.relax();
            sizes.push(s);
        }

        return sizes;
    }

    fn plot_size_in_time(&self, scaled_sizes: Vec<f32>) {
        let mut plot = Plot::new();

        let trace =
            Scatter::new(Vec::from_iter(0..=scaled_sizes.len()), scaled_sizes).mode(Mode::Markers);
        let layout = Layout::new()
            .title(Title::from(format!(
                "Scaled Size in time for Oslo size = {}",
                self.size
            )))
            .x_axis(Axis::new().title(Title::from("Time")))
            .y_axis(Axis::new().title(Title::from("Scaled size of avalanche")));

        plot.add_trace(trace);
        plot.set_layout(layout);
        plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
    }

    fn plot_size_to_probability(&self, sizes: Vec<u32>) {
        let mut occurance = HashMap::new();
        let n_of_values = sizes.len();
        for size in sizes {
            let count = occurance.entry(size).or_insert(0);
            *count += 1;
        }

        let mut plot = Plot::new();
        let trace = Scatter::new(
            occurance.keys().cloned().collect(),
            occurance
                .values()
                .cloned()
                .map(|v| v as f32 / n_of_values as f32)
                .collect(),
        )
        .mode(Mode::Markers);
        let layout = Layout::new()
            .title(Title::from(format!(
                "Avalanche size probability for Oslo size = {}",
                self.size
            )))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Log)
                    .title(Title::from("Avalanche size")),
            )
            .y_axis(
                Axis::new()
                    .type_(AxisType::Log)
                    .title(Title::from("Probability")),
            );

        plot.add_trace(trace);
        plot.set_layout(layout);
        plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
    }

    fn run_and_analyse(&mut self, n: u32) {
        let sizes = self.run(n);

        // scale sizes
        let scaled_sizes: Vec<f32> = sizes.iter().map(|&s| s as f32 / self.size as f32).collect();

        self.plot_size_in_time(scaled_sizes);
        self.plot_size_to_probability(sizes);
    }

    fn run_and_analyse_with_treshold(&mut self, n: u32, treshold: f32) {
        let sizes = self.run_with_treshold(n, treshold);

        // scale sizes
        let scaled_sizes: Vec<f32> = sizes.iter().map(|&s| s as f32 / self.size as f32).collect();

        self.plot_size_in_time(scaled_sizes);
        self.plot_size_to_probability(sizes);
    }
}

fn main() {
    let mut model = OsloModel::new(64);
    model.run_and_analyse(50000);
    let sizes = vec![64, 128, 256, 512, 1024];
    for size in sizes {
        let mut model = OsloModel::new(size);
        model.run_and_analyse_with_treshold(50000, 0.9);
    }
}
