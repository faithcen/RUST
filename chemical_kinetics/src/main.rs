extern crate ndarray;
use ndarray::{Array1, array};
use std::fs::File;
use std::io::prelude::*;

fn arrhenius(a: f64, ea: f64, r: f64, t: f64) -> f64 {
    a * (-ea / (r * t)).exp()
}

fn clamp_concentrations(concentrations: &mut Array1<f64>) {
    for concentration in concentrations.iter_mut() {
        if *concentration < 0.0 {
            *concentration = 0.0;
        }
    }
}

fn model(t: f64, concentrations: &Array1<f64>, params: &Array1<f64>) -> Array1<f64> {
    let x = concentrations[0];
    let y = concentrations[1];
    
    let a1 = params[0];
    let ea1 = params[1];
    let a2 = params[2];
    let ea2 = params[3];
    let r = params[4];
    let t0 = params[5];
    let heating_rate = params[6];

    let temp = t0 + heating_rate * t;
    let k1 = arrhenius(a1, ea1, r, temp);
    let k2 = arrhenius(a2, ea2, r, temp);
    
    array![-k1 * x, k1 * x - k2 * y, k2 * y]
}

fn rk4_step(t: f64, y: &Array1<f64>, dt: f64, params: &Array1<f64>) -> Array1<f64> {
    let k1 = model(t, y, params) * dt;
    let k2 = model(t + 0.5 * dt, &(y + &k1 * 0.5), params) * dt;
    let k3 = model(t + 0.5 * dt, &(y + &k2 * 0.5), params) * dt;
    let k4 = model(t + dt, &(y + &k3), params) * dt;

    y.clone() + (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0
}

fn main() {
    let mut concentrations = array![1.0, 0.0, 0.0];
    let dt = 0.1;
    let t_final = 1000.0;

    let a1 = 1e7;
    let ea1 = 90e3;
    let a2 = 1e6;
    let ea2 = 90e3;
    let r = 8.314;
    let t0 = 300.0;
    let heating_rate = 1.0;

    let params = array![a1, ea1, a2, ea2, r, t0, heating_rate];

    let mut file = File::create("output.txt").expect("Unable to create file");
    file.write_all(b"Time, X, Y, Z\n").unwrap();

    let mut t = 0.0;
    while t <= t_final {
        let line = format!("{}, {}, {}, {}\n", t, concentrations[0], concentrations[1], concentrations[2]);
        file.write_all(line.as_bytes()).unwrap();
    
        concentrations = rk4_step(t, &concentrations, dt, &params);
    
        // Clamp concentrations to non-negative values
        clamp_concentrations(&mut concentrations);
    
        // Normalize the concentrations
        let total_concentration = concentrations.sum();
        concentrations /= total_concentration;
    
        t += dt;
    }
}
