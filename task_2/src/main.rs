use std::vec;

use plotly::color::Color;
use plotly::common::{Line, Mode, Title};
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter, color};
use nalgebra::{Matrix2, Vector2};

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

fn task_2() {
    let systems = vec![system_1, system_2, system_3, system_4];
    let systems_names = vec!["x''-x=0", "x''+sin(x)=0", "x''+x-x^3=0", "x''-x+x^3=0"];
    let dt = 0.1;
    // make fucntion to make initial condidiotns for grid from -10, -10 to 10, 10
    let initial_conditions = (-10..=10).step_by(2).flat_map(|x| {
        (-10..=10).step_by(2).map(move |y| (x as f64, y as f64))
    }).collect::<Vec<(f64, f64)>>();
    for i in 0..systems.len() {
        let mut plot = Plot::new();
        for condition in initial_conditions.clone() {
            let mut t = 0.0;
            let mut x = condition.0;
            let mut y = condition.1;
            let mut x_axis = vec![x];
            let mut y_axis = vec![y];
            while t < 10.0 {
                let (next_x, next_y) = midpoint_method(x, y, systems[i], dt);
                t += dt;
                x = next_x;
                y = next_y;
                x_axis.push(x);
                y_axis.push(y);
            }
            let trace = Scatter::new(x_axis, y_axis)
                .mode(Mode::Lines)
                .name(format!("x_0 = ({}, {})", condition.0, condition.1))
                .line(Line::new().color(color::NamedColor::Black))
                ;
            plot.add_trace(trace);       
        } 
        let layout = Layout::new()
        .title(Title::from(systems_names[i]))
        .x_axis(Axis::new().title(Title::from("x")).range(vec![-10.0, 10.0]))
        .y_axis(Axis::new().title(Title::from("y")).range(vec![-10.0, 10.0]));
        plot.set_layout(layout);
        plot.show();
    }
}

fn system_1() -> (fn(f64, f64) -> f64, fn(f64, f64) -> f64) {
    let fx: fn(f64, f64) -> f64 = |x, y| y;
    let fy: fn(f64, f64) -> f64 = |x, y| -x;
    (fx, fy)
}

fn system_2() -> (fn(f64, f64) -> f64, fn(f64, f64) -> f64) {
    let fx: fn(f64, f64) -> f64 = |x, y| y;
    let fy: fn(f64, f64) -> f64 = |x, y| -x.sin();
    (fx, fy)
}

fn system_3() -> (fn(f64, f64) -> f64, fn(f64, f64) -> f64) {
    let fx: fn(f64, f64) -> f64 = |x, y| y;
    let fy: fn(f64, f64) -> f64 = |x, y| -x + x.powi(3);
    (fx, fy)
}

fn system_4() -> (fn(f64, f64) -> f64, fn(f64, f64) -> f64) {
    let fx: fn(f64, f64) -> f64 = |x, y| y;
    let fy: fn(f64, f64) -> f64 = |x, y| x - x.powi(3);
    (fx, fy)
}

fn midpoint_method(
    xn: f64,
    yn: f64,
    f: fn() -> (fn(f64, f64) -> f64, fn(f64, f64) -> f64), 
    dt: f64
) -> (f64, f64) {
    let (fx, fy) = f();
    let kx = dt*fx(xn, yn);
    let ky = dt*fy(xn, yn);
    let next_x = xn + dt*fx(xn+ 0.5*kx, yn + 0.5*ky);
    let next_y = yn + dt*fy(xn+ 0.5*kx, yn + 0.5*ky);
    (next_x, next_y)
}

fn task_3(){
    let A_matrixies = vec![
        Matrix2::new(-2.0, 1.0, 0.0, 2.0),
        Matrix2::new(3.0, -4.0, 2.0, -1.0),
        Matrix2::new(-3.0, -2.0, -1.0, -3.0),
        Matrix2::new(2.0, 0.0, 2.0, 0.0), 
    ];
    for A in A_matrixies {
        let mut plot = Plot::new();
        
    }
}

fn main() {
    task_1();
    task_2();
}
