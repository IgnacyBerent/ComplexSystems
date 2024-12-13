use plotly::common::{Anchor, Font, Mode, Title};
use plotly::layout::{Annotation, Axis, AxisType, Layout};
use plotly::{HeatMap, ImageFormat, Plot};
use rand::Rng;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
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

        for i in 0..l {
            for j in 0..l {
                if i > 0 {
                    sites[i][j]
                        .borrow_mut()
                        .neighbours
                        .push(Some(Rc::downgrade(&sites[i - 1][j])));
                } else {
                    sites[i][j].borrow_mut().neighbours.push(None);
                }
                if j > 0 {
                    sites[i][j]
                        .borrow_mut()
                        .neighbours
                        .push(Some(Rc::downgrade(&sites[i][j - 1])));
                } else {
                    sites[i][j].borrow_mut().neighbours.push(None);
                }
                if i < l - 1 {
                    sites[i][j]
                        .borrow_mut()
                        .neighbours
                        .push(Some(Rc::downgrade(&sites[i + 1][j])));
                } else {
                    sites[i][j].borrow_mut().neighbours.push(None);
                }
                if j < l - 1 {
                    sites[i][j]
                        .borrow_mut()
                        .neighbours
                        .push(Some(Rc::downgrade(&sites[i][j + 1])));
                } else {
                    sites[i][j].borrow_mut().neighbours.push(None);
                }
            }
        }
        return PercolationLattice {
            sites: sites,
            l: l,
            p: p,
        };
    }

    fn burning_method(&self) {
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
    }

    fn plot_lattice(&self) {
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
            .title(Title::from("Percolation Lattice"))
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
}

fn main() {
    let l = 20;
    let p = 0.55;
    let pl = PercolationLattice::new(l, p);
    let s = pl.max_cluster_size();
    pl.burning_method();
    pl.plot_lattice();
}
