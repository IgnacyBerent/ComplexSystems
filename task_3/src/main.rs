#[cfg(not(target_arch = "wasm32"))]
use plotters::prelude::BitMapBackend;
#[cfg(target_arch = "wasm32")]
use plotters::prelude::CanvasBackend;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

struct Site {
    height: u32,
    treshold: u8,
}

struct OsloModel {
    sites: Vec<Site>,
    smax: u32,
}

impl OsloModel {
    fn new(size: u32) -> OsloModel {
        let mut rng = rand::thread_rng();
        let mut sites: Vec<Site> = (0..=size)
            .map(|_| Site {
                height: rng.gen_range(0..=size),
                treshold: rng.gen_range(1..=2),
            })
            .collect();
        sites.push(Site {
            height: 0,
            treshold: 0,
        });
        OsloModel { sites, smax: size }
    }

    fn drive(&mut self) {
        self.sites[0].height += 1;
    }

    fn will_move(&self, i: u32) -> bool {
        if self.sites[i as usize].height as i32 - self.sites[i as usize + 1].height as i32
            >= self.sites[i as usize].treshold as i32
        {
            return true;
        }
        return false;
    }

    fn relax(&mut self) -> f32 {
        let mut rng = rand::thread_rng();
        let mut s = 0;
        for i in 0..=self.smax {
            if self.will_move(i) {
                // Move grain from current site
                self.sites[i as usize].height -= 1;
                // Update treshold
                self.sites[i as usize].treshold = rng.gen_range(1..=2);
                // Move grain to next site
                self.sites[i as usize + 1].height += 1;
                s += 1;
            }
        }
        self.sites[self.smax as usize + 1].height = 0;
        return s as f32 / self.smax as f32;
    }

    fn print(&self) {
        for site in &self.sites {
            println!("{}", site.height);
        }
        println!();
    }
}

fn make_simulation(size: u32) {
    let mut sim = OsloModel::new(size);

    #[cfg(not(target_arch = "wasm32"))]
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    #[cfg(target_arch = "wasm32")]
    let root = CanvasBackend::new("plot").unwrap().into_drawing_area();

    for _ in 0..1000 {
        sim.print();
        // Clear the drawing area by filling it with white color
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("Simulation", ("sans-serif", 50).into_font())
            .build_cartesian_2d(0..size, 0..(size as f32 * 1.2) as i32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        sim.drive();
        sim.relax();

        let heights = sim
            .sites
            .iter()
            .map(|site| site.height as i32)
            .collect::<Vec<i32>>();

        chart
            .draw_series(LineSeries::new((0..).zip(heights.iter().cloned()), &RED))
            .unwrap();

        root.present().unwrap();
        sleep(Duration::from_millis(350));
    }
}

fn main() {
    let size = 50;
}
