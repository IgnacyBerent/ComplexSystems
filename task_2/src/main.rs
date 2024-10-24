use plotly::common::{Mode, Title};
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter};

struct Solution {
    x_0: f64,
    t_axis: Vec<f64>,
    x_axis: Vec<f64>,
}

fn euler_method(dt: f64, xn: f64) -> f64 {
    xn + dt*function(xn)
}

fn function(x: f64) -> f64 {
    x*(x-1.0)*(x-2.0)
}

fn task_1() {
    let dtvec = vec![0.1, 0.01, 0.001, 0.0001];
    let x_0vec = vec![-0.1, 0.1, 0.9, 1.1, 1.9, 2.1];

    for dt in dtvec {
        let mut plot = Plot::new();
        let mut solutions: Vec<Solution> = Vec::new();
        for x_0 in x_0vec.clone() {
            let mut t_axis = vec![0.0];
            let mut x_axis = vec![x_0];
            let mut t = 0.0;
            let mut x = x_0;
            while t < 5.0 {
                x = euler_method(dt, x);
                t += dt;
                t_axis.push(t);
                x_axis.push(x);
            }
            let solution = Solution {
                x_0: x_0,
                t_axis: t_axis,
                x_axis: x_axis,
            };
            solutions.push(solution);
        }
        for solution in solutions{
            let trace = Scatter::new(solution.t_axis, solution.x_axis)
                .mode(Mode::Lines)
                .name(format!("x_0 = {}", solution.x_0));
            plot.add_trace(trace);
        }
        let layout = Layout::new()
        .title(Title::from("Solutions for dt=".to_string() + &dt.to_string()))
        .x_axis(Axis::new().title(Title::from("t")))
        .y_axis(Axis::new().title(Title::from("x")).range(vec![-2.0, 3.0]));
        plot.set_layout(layout);
        plot.show();
    }
}

fn main() {
    task_1();
}
