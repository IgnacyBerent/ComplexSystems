use std::vec;

use nalgebra::{Matrix2, Vector2};
use plotly::color::{Color, Rgb};
use plotly::common::{ColorScale, Line, Mode, Title};
use plotly::layout::{Axis, Layout};
use plotly::{color, Plot, Scatter};

struct Solution {
    x_0: f64,
    t_axis: Vec<f64>,
    x_axis: Vec<f64>,
}

fn euler_method(dt: f64, xn: f64) -> f64 {
    xn + dt * function(xn)
}

fn function(x: f64) -> f64 {
    x * (x - 1.0) * (x - 2.0)
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
        for solution in solutions {
            let trace = Scatter::new(solution.t_axis, solution.x_axis)
                .mode(Mode::Lines)
                .name(format!("x_0 = {}", solution.x_0));
            plot.add_trace(trace);
        }
        let layout = Layout::new()
            .title(Title::from(
                "Solutions for dt=".to_string() + &dt.to_string(),
            ))
            .x_axis(Axis::new().title(Title::from("t")))
            .y_axis(Axis::new().title(Title::from("x")).range(vec![-2.0, 3.0]));
        plot.set_layout(layout);
        plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
    }
}

fn task_2() {
    let systems = vec![system_1, system_2, system_3, system_4];
    let systems_names = vec!["x''-x=0", "x''+sin(x)=0", "x''+x-x^3=0", "x''-x+x^3=0"];
    let dt = 0.1;
    // make fucntion to make initial condidiotns for grid from -10, -10 to 10, 10
    let initial_conditions = (-10..=10)
        .step_by(2)
        .flat_map(|x| (-10..=10).step_by(2).map(move |y| (x as f64, y as f64)))
        .collect::<Vec<(f64, f64)>>();
    for i in 3..=3 {
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
            add_gradient_traces(&mut plot, x_axis, y_axis);
        }
        let layout = Layout::new()
            .title(Title::from(systems_names[i]))
            .x_axis(Axis::new().title(Title::from("x")).range(vec![-10.0, 10.0]))
            .y_axis(Axis::new().title(Title::from("y")).range(vec![-10.0, 10.0]));
        plot.set_layout(layout);
        plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
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
    dt: f64,
) -> (f64, f64) {
    let (fx, fy) = f();
    let kx = dt * fx(xn, yn);
    let ky = dt * fy(xn, yn);
    let next_x = xn + dt * fx(xn + 0.5 * kx, yn + 0.5 * ky);
    let next_y = yn + dt * fy(xn + 0.5 * kx, yn + 0.5 * ky);
    (next_x, next_y)
}

fn task_3() {
    let mut sys_num = 1;
    let t_range = 4.0;
    let initial_con_range = 6;
    let dt = 0.1;
    let initial_conditions = (-initial_con_range..=initial_con_range)
        .step_by(2)
        .flat_map(|x1| {
            (-initial_con_range..=initial_con_range)
                .step_by(2)
                .map(move |x2| (x1 as f64, x2 as f64))
        })
        .collect::<Vec<(f64, f64)>>();
    let a_matrixies: Vec<
        nalgebra::Matrix<
            f64,
            nalgebra::Const<2>,
            nalgebra::Const<2>,
            nalgebra::ArrayStorage<f64, 2, 2>,
        >,
    > = vec![
        Matrix2::new(-2.0, 1.0, 0.0, 2.0),
        Matrix2::new(3.0, -4.0, 2.0, -1.0),
        Matrix2::new(-3.0, -2.0, -1.0, -3.0),
        Matrix2::new(2.0, 0.0, 0.0, 2.0),
    ];

    draw_state_graph(a_matrixies.clone());

    for A in a_matrixies {
        let mut plot = Plot::new();
        for condition in initial_conditions.clone() {
            let mut t = 0.0;
            let mut x = Vector2::new(condition.0, condition.1);
            let mut x1_axis = vec![x[0]];
            let mut x2_axis = vec![x[1]];
            while t < t_range {
                x = linear_midpoint_method(A, x, dt);
                t += dt;
                x1_axis.push(x[0]);
                x2_axis.push(x[1]);
            }
            add_gradient_traces(&mut plot, x1_axis, x2_axis);
        }
        let layout = Layout::new()
            .title(Title::from(
                "Linear system ".to_string() + &sys_num.to_string(),
            ))
            .x_axis(
                Axis::new()
                    .title(Title::from("x1"))
                    .range(vec![-20.0, 20.0]),
            )
            .y_axis(
                Axis::new()
                    .title(Title::from("x2"))
                    .range(vec![-20.0, 20.0]),
            );
        plot.set_layout(layout);
        //plot.show();
        plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
        sys_num += 1;
    }
}

fn draw_state_graph(
    a_matrixies: Vec<
        nalgebra::Matrix<
            f64,
            nalgebra::Const<2>,
            nalgebra::Const<2>,
            nalgebra::ArrayStorage<f64, 2, 2>,
        >,
    >,
) {
    // Draw parabole
    // make T_axis as range from -10 to 10 with step 0.1
    let T_axis = (-100..=100).map(|T| T as f32 * 0.1).collect::<Vec<f32>>();
    let D_curve = T_axis
        .clone()
        .into_iter()
        .map(|T| T * T / 4.0)
        .collect::<Vec<f32>>();
    let mut plot = Plot::new();
    let trace = Scatter::new(T_axis, D_curve)
        .mode(Mode::Lines)
        .name("D=T^2/4")
        .line(Line::new().color(color::NamedColor::Red));
    plot.add_trace(trace);
    // Draw each system position
    let mut sys_num = 1;
    for A in a_matrixies {
        let trace = A.trace();
        let det = A.determinant();
        let trace = Scatter::new(vec![trace], vec![det])
            .mode(Mode::Markers)
            .name(format!("System = {}", sys_num));
        plot.add_trace(trace);
        sys_num += 1;
    }
    let layout = Layout::new()
        .title(Title::from("System position"))
        .x_axis(
            Axis::new()
                .title(Title::from("trace"))
                .range(vec![-10.0, 10.0]),
        )
        .y_axis(
            Axis::new()
                .title(Title::from("det"))
                .range(vec![-10.0, 10.0]),
        );
    plot.set_layout(layout);
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn add_gradient_traces(plot: &mut Plot, x: Vec<f64>, y: Vec<f64>) {
    let num_segments = x.len();
    for i in 0..num_segments - 1 {
        let x_segment = vec![x[i], x[i + 1]];
        let y_segment = vec![y[i], y[i + 1]];

        let red = 255 - (i * 255 / num_segments) as u8;
        let blue = (i * 255 / num_segments) as u8;
        let color = Rgb::new(red, 0, blue);

        let trace = Scatter::new(x_segment, y_segment.clone())
            .mode(Mode::Lines)
            .line(Line::new().color(color));
        plot.add_trace(trace);
    }
}

fn linear_eq_system(a: Matrix2<f64>, x: Vector2<f64>) -> Vector2<f64> {
    let x_dot: Vector2<f64> = a * x;
    x_dot
}

fn linear_midpoint_method(a: Matrix2<f64>, x: Vector2<f64>, dt: f64) -> Vector2<f64> {
    let k: Vector2<f64> = dt * linear_eq_system(a, x);

    let dx_dot: Vector2<f64> = x + dt * linear_eq_system(a, x + 0.5 * k);
    dx_dot
}

fn task_4() {
    let dt = 0.1;
    // make fucntion to make initial condidiotns for grid from -10, -10 to 10, 10
    let initial_conditions = (-10..=10)
        .step_by(2)
        .flat_map(|x| (-10..=10).step_by(2).map(move |y| (x as f64, y as f64)))
        .collect::<Vec<(f64, f64)>>();
    let mut plot = Plot::new();
    for condition in initial_conditions.clone() {
        let mut t = 0.0;
        let mut x = condition.0;
        let mut y = condition.1;
        let mut x_axis = vec![x];
        let mut y_axis = vec![y];
        while t < 10.0 {
            let (next_x, next_y) = midpoint_method(x, y, system_task_4, dt);
            t += dt;
            x = next_x;
            y = next_y;
            x_axis.push(x);
            y_axis.push(y);
        }
        add_gradient_traces(&mut plot, x_axis, y_axis);
    }
    let layout = Layout::new()
        .x_axis(Axis::new().title(Title::from("x")).range(vec![-25.0, 25.0]))
        .y_axis(Axis::new().title(Title::from("y")).range(vec![-25.0, 25.0]));
    plot.set_layout(layout);
    plot.show_image(plotly::ImageFormat::JPEG, 1000, 800);
}

fn system_task_4() -> (fn(f64, f64) -> f64, fn(f64, f64) -> f64) {
    let dx: fn(f64, f64) -> f64 = |x, y| x * (3.0 - x - 2.0 * y);
    let dy: fn(f64, f64) -> f64 = |x, y| y * (2.0 - x - y);
    (dx, dy)
}

fn main() {
    task_1();
    task_2();
    task_3();
    task_4();
}
