use plotly::common::{Anchor, Font, Mode, Title};
use plotly::layout::{self, Annotation, Axis, AxisType, Layout};
use plotly::{HeatMap, ImageFormat, Plot, Scatter};
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Write;
use std::path::Iter;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
struct Site {
    value: u32,
    // left = 0, up = 1, right = 2, down = 3
    neighbours: Vec<Option<Weak<RefCell<Site>>>>,
}

impl Site {
    fn new(p: f32) -> Self {
        let mut rng = rand::thread_rng();
        let value = {
            let r = rng.gen::<f32>();
            if r < p {
                1
            } else {
                0
            }
        };
        return Site {
            value: value,
            neighbours: Vec::with_capacity(4),
        };
    }
}

struct PercolationLattice {
    sites: Vec<Vec<Rc<RefCell<Site>>>>,
    l: usize,
    p: f32,
}

impl PercolationLattice {
    fn new(l: usize, p: f32) -> Self {
        let mut sites = Vec::with_capacity(l);
        for _ in 0..l {
            let mut row = Vec::with_capacity(l);
            for _ in 0..l {
                row.push(Rc::new(RefCell::new(Site::new(p))));
            }
            sites.push(row);
        }

        return PercolationLattice {
            sites: sites,
            l: l,
            p: p,
        };
    }

    fn initialize_neighbours(&self) {
        for i in 0..self.l {
            for j in 0..self.l {
                if i > 0 {
                    self.sites[i][j]
                        .borrow_mut()
                        .neighbours
                        .push(Some(Rc::downgrade(&self.sites[i - 1][j])));
                } else {
                    self.sites[i][j].borrow_mut().neighbours.push(None);
                }
                if j > 0 {
                    self.sites[i][j]
                        .borrow_mut()
                        .neighbours
                        .push(Some(Rc::downgrade(&self.sites[i][j - 1])));
                } else {
                    self.sites[i][j].borrow_mut().neighbours.push(None);
                }
                if i < self.l - 1 {
                    self.sites[i][j]
                        .borrow_mut()
                        .neighbours
                        .push(Some(Rc::downgrade(&self.sites[i + 1][j])));
                } else {
                    self.sites[i][j].borrow_mut().neighbours.push(None);
                }
                if j < self.l - 1 {
                    self.sites[i][j]
                        .borrow_mut()
                        .neighbours
                        .push(Some(Rc::downgrade(&self.sites[i][j + 1])));
                } else {
                    self.sites[i][j].borrow_mut().neighbours.push(None);
                }
            }
        }
    }

    fn burning_method(&self) -> bool {
        let mut changed = true;
        let mut n = 2;
        // set the first row to 2
        for j in 0..self.l {
            if self.sites[0][j].borrow().value == 1 {
                self.sites[0][j].borrow_mut().value = 2;
            }
        }

        while changed {
            changed = false;
            for i in 0..self.l {
                for j in 0..self.l {
                    if self.sites[i][j].borrow().value == n {
                        for neighbour in &self.sites[i][j].borrow().neighbours {
                            if let Some(neighbour) = neighbour {
                                if let Some(neighbour) = neighbour.upgrade() {
                                    if neighbour.borrow().value == 1 {
                                        neighbour.borrow_mut().value = n + 1;
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            n += 1;
        }
        return self.sites[self.l - 1]
            .iter()
            .any(|site| site.borrow().value > 1);
    }

    fn plot_lattice(&self, title: &str) {
        let mut x = vec![];
        let mut y = vec![];
        let mut z = vec![];
        let mut text_values = vec![];

        for i in 0..self.l {
            for j in 0..self.l {
                x.push(j);
                y.push(i);
                z.push(self.sites[i][j].borrow().value);
                text_values.push(
                    Annotation::new()
                        .text(format!("{}", self.sites[i][j].borrow().value))
                        .x(j as f64)
                        .y(i as f64)
                        .show_arrow(false)
                        .font(Font::new().size(14))
                        .x_anchor(Anchor::Center)
                        .y_anchor(Anchor::Middle),
                );
            }
        }

        let trace = HeatMap::new(x, y, z);
        let layout = Layout::new()
            .title(Title::from(title))
            .annotations(text_values);
        let mut plot = Plot::new();
        plot.add_trace(trace);
        plot.set_layout(layout);
        plot.show_image(ImageFormat::PNG, 600, 600);
    }

    fn dfs(&self, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> usize {
        if i >= self.l || j >= self.l || visited[i][j] || self.sites[i][j].borrow().value == 0 {
            return 0;
        }
        visited[i][j] = true;
        let mut size = 1;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for &(di, dj) in &directions {
            let ni = i.wrapping_add(di as usize);
            let nj = j.wrapping_add(dj as usize);
            if ni < self.l && nj < self.l {
                size += self.dfs(ni, nj, visited);
            }
        }
        size
    }

    fn max_cluster_size(&self) -> usize {
        let mut visited = vec![vec![false; self.l]; self.l];
        let mut max_size = 0;
        for i in 0..self.l {
            for j in 0..self.l {
                if !visited[i][j] && self.sites[i][j].borrow().value >= 1 {
                    let size = self.dfs(i, j, &mut visited);
                    if size > max_size {
                        max_size = size;
                    }
                }
            }
        }
        max_size
    }

    fn hoshen_kopelman(&self, plot: bool) -> Vec<usize> {
        let n = PercolationLattice {
            sites: self
                .sites
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|site| {
                            Rc::new(RefCell::new(Site {
                                value: site.borrow().value,
                                neighbours: vec![],
                            }))
                        })
                        .collect()
                })
                .collect(),
            l: self.l,
            p: self.p,
        };
        n.initialize_neighbours();
        let mut k = 2;
        let mut m = HashMap::new();
        for i in 0..n.l {
            for j in 0..n.l {
                if n.sites[i][j].borrow().value == 1 {
                    n.sites[i][j].borrow_mut().value = k;
                    m.insert(k, 1);
                    let mut q = VecDeque::new();
                    for neighbour in &n.sites[i][j].borrow().neighbours {
                        if let Some(neighbour) = neighbour {
                            if let Some(neighbour) = neighbour.upgrade() {
                                if neighbour.borrow().value == 1 {
                                    q.push_back(neighbour);
                                }
                            }
                        }
                    }
                    while !q.is_empty() {
                        let site = q.pop_front().unwrap();
                        m.entry(k).and_modify(|e| *e += 1);
                        site.borrow_mut().value = k;
                        for neighbour in &site.borrow().neighbours {
                            if let Some(neighbour) = neighbour {
                                if let Some(neighbour) = neighbour.upgrade() {
                                    if neighbour.borrow().value == 1 {
                                        q.push_back(neighbour);
                                    }
                                }
                            }
                        }
                    }
                    k += 1;
                }
            }
        }
        if plot {
            n.plot_lattice(format!("Hoshen-Kopelman Clusters for p={}", self.p).as_str());
        }
        let m_vec = m.into_values().collect();
        return m_vec;
    }
}

fn percolation_examples() {
    let l = 10;
    let p_vec = vec![0.4, 0.6, 0.8];
    for p in p_vec {
        let pl = PercolationLattice::new(l, p);
        pl.initialize_neighbours();
        pl.hoshen_kopelman(true);
        pl.burning_method();
        pl.plot_lattice(format!("Percolation at p = {}", p).as_str());
    }
}

fn monte_carlo(t: i32, l: usize, p: f32) -> (f32, f32) {
    let mut percolations = 0;
    let mut s_maxes = vec![];
    for _ in 0..t {
        let pl = PercolationLattice::new(l, p);
        pl.initialize_neighbours();
        s_maxes.push(pl.max_cluster_size());
        if pl.burning_method() {
            percolations += 1;
        }
    }
    let percolation_probability = percolations as f32 / t as f32;
    let s_max_avg = s_maxes.iter().sum::<usize>() as f32 / t as f32;
    return (percolation_probability, s_max_avg);
}

fn monte_carlo_examples() {
    let po = 50;
    let pk = 70;
    let dp = 2;
    let t = 1000;
    let l_vec = vec![16, 32, 64];
    let p_vals = (po..=pk).step_by(dp).map(|x| x as f32 / 100.0);
    let mut plot1 = Plot::new();
    let mut plot2 = Plot::new();
    for l in l_vec {
        let mut percolation_probabilities = vec![];
        let mut s_max_avgs = vec![];
        for p in p_vals.clone() {
            let (percolation_probability, s_max_avg) = monte_carlo(t, l, p);
            percolation_probabilities.push(percolation_probability);
            s_max_avgs.push(s_max_avg);
        }
        let trace_p = Scatter::new(p_vals.clone().collect(), percolation_probabilities)
            .mode(Mode::LinesMarkers)
            .name(format!("l = {}", l).as_str());
        let trace_s = Scatter::new(p_vals.clone().collect(), s_max_avgs)
            .mode(Mode::LinesMarkers)
            .name(format!("l = {}", l).as_str());
        plot1.add_trace(trace_p);
        plot2.add_trace(trace_s);
    }
    let layout1 = Layout::new()
        .title("Percolation Probability vs p")
        .x_axis(Axis::new().title("p"))
        .y_axis(Axis::new().title("P(p)"));
    let layout2 = Layout::new()
        .title("Average Cluster Size vs p")
        .x_axis(Axis::new().title("p"))
        .y_axis(Axis::new().title("<S_max>"));
    plot1.set_layout(layout1);
    plot2.set_layout(layout2);

    // add vertical line at p = 0.592746
    let trace = Scatter::new(vec![0.592746, 0.592746], vec![0.0, 1.0])
        .mode(Mode::Lines)
        .name("Theoretical p_c = 0.592746");
    plot1.add_trace(trace);
    plot1.show_image(ImageFormat::PNG, 1000, 800);
    plot2.show_image(ImageFormat::PNG, 1000, 800);
}

fn occupation_probability_examples() {
    let pc = vec![0.592746];
    let t = 1000;
    let l = 64;
    let p_low = vec![0.3, 0.4, 0.5];
    let p_high = vec![0.7, 0.8, 0.9];

    let layout = Layout::new()
        .title("Cluster Size Distribution")
        .x_axis(Axis::new().title("s"))
        .y_axis(Axis::new().title("ln(ns)"));

    let p_options = vec![p_low, pc, p_high];
    for ps in p_options {
        let mut plot = Plot::new();
        for p in ps {
            let mut sizes = vec![];
            for _ in 0..t {
                let pl = PercolationLattice::new(l, p);
                pl.hoshen_kopelman(false);
                sizes.extend(pl.hoshen_kopelman(false));
            }
            let mut m = HashMap::new();
            let s_occurencies = sizes.len();
            for s in sizes {
                m.entry(s).and_modify(|e| *e += 1).or_insert(1);
            }
            let mut x = vec![];
            let mut y = vec![];
            for (size, count) in m {
                x.push(size as f64);
                (y.push((count as f64 / s_occurencies as f64).ln()));
            }
            let trace = Scatter::new(x, y)
                .mode(Mode::Markers)
                .name(format!("p = {}", p).as_str());
            plot.add_trace(trace);
        }
        plot.set_layout(layout.clone());
        plot.show_image(ImageFormat::PNG, 1000, 800);
    }
}

fn main() {
    percolation_examples();
    monte_carlo_examples();
    occupation_probability_examples();
}
