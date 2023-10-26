use std::time::Instant;
struct RK4<F>
where
    F: Fn(f64, f64) -> f64,
{
    func: F,
}

impl<F> RK4<F>
where
    F: Fn(f64, f64) -> f64,
{
    fn new(func: F) -> Self {
        RK4 { func }
    }

    fn integrate(&self, y0: f64, t0: f64, t: f64, h: f64) -> f64 {
        let mut y = y0;
        let mut time = t0;

        while time < t {
            let k1 = h * (self.func)(time, y);
            let k2 = h * (self.func)(time + 0.5 * h, y + 0.5 * k1);
            let k3 = h * (self.func)(time + 0.5 * h, y + 0.5 * k2);
            let k4 = h * (self.func)(time + h, y + k3);

            y += (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
            time += h;
        }

        y
    }
}

// Normal fonksiyon tanımıyla dy/dt fonksiyonunu tanımlayalım.
#[allow(unused_variables)]
fn dy_dt(t: f64, y: f64) -> f64 {
    t.powi(2)
}
#[allow(unused_variables)]
fn main() {
    let rk4_solver = RK4::new(dy_dt);
    //let rk4_solver = RK4::new(|t, y| t.powi(2));

    let y0 = 1.0;
    let t0 = 0.0;
    let t_end = 2.0;
    let step = 0.0001;

    let start = Instant::now(); 
    let result_y = rk4_solver.integrate(y0, t0, t_end, step);
    let duration = start.elapsed();
    println!("y({}) = {}", t_end, result_y);
    println!("Computaion time is = {:?}", duration);
}
