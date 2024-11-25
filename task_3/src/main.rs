#[cfg(not(target_arch = "wasm32"))]
use plotters::prelude::BitMapBackend;
use plotters::prelude::*;
use rand::Rng;

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

    fn run_once(&mut self) -> f32 {
        self.drive();
        return self.relax();
    }
}

fn make_simulation(size: u32) {
    const OUT_FILE_NAME: &str = "oslo_model.gif";
    let mut sim = OsloModel::new(size);
    let axis_size = (size as f32 * 1.1) as i32;

    let root = BitMapBackend::gif(OUT_FILE_NAME, (800, 600), 200)
        .unwrap()
        .into_drawing_area();
    for _ in 0..500 {
        root.fill(&WHITE).unwrap();
        sim.run_once();
        let mut chart = ChartBuilder::on(&root)
            .caption("Oslo Model", ("sans-serif", 30).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0..size as i32, 0..axis_size)
            .unwrap();

        let mut data = vec![];
        for (i, site) in sim.sites.iter().enumerate() {
            data.push((i as i32, site.height as i32));
        }

        chart
            .draw_series(LineSeries::new(data.iter().map(|(x, y)| (*x, *y)), &RED))
            .unwrap();
        chart
            .configure_mesh()
            .x_label_formatter(&|x| format!("{}", x))
            .draw()
            .unwrap();

        root.present().unwrap();
    }
}

fn main() {
    let size = 50;
    make_simulation(size);
}
