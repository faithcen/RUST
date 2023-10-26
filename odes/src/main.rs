// https://srenevey.github.io/ode-solvers/index.html

use ode_solvers::dopri5::*;
use ode_solvers::*;
use std::path::Path;
use std::fs::File;
use std::io::{Write, Error};

type State = Vector6<f64>;
type Time = f64;

struct KeplerOrbit {
    mu: f64,
  }

impl ode_solvers::System<State> for KeplerOrbit {
    // Equations of motion of the system
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let r = (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]).sqrt();

        dy[0] = y[3];
        dy[1] = y[4];
        dy[2] = y[5];
        dy[3] = - self.mu * y[0] / r.powi(3);
        dy[4] = - self.mu * y[1] / r.powi(3);
        dy[5] = - self.mu * y[2] / r.powi(3);
    }
}  

fn main() {
    let system = KeplerOrbit {mu: 398600.435436};
    let pi: f64 = 3.14; 

    let a: f64 = 20000.0;
    let period = 2.0 * pi * (a.powi(3) / system.mu).sqrt();
    let y0 = State::new(-5007.248417988539, -1444.918140151374, 3628.534606178356, 0.717716656891, -10.224093784269, 0.748229399696);

    let mut stepper = Dopri5::new(system, 0.0, 5.0 * period, 60.0, y0, 1.0e-10, 1.0e-10);
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("{:?}", stats);
            let path = Path::new("output.txt");
            if let Err(e) = save(stepper.x_out(), stepper.y_out(), path) {
                println!("Failed to save results: {}", e);
            } else {
                println!("Results saved in: {:?}", path);
            }
            
        },
        Err(_) => println!("An error occured."),
    }
}



fn save(x: &[f64], y: &[Vector6<f64>], path: &Path) -> Result<(), Error> {
    let mut file = File::create(path)?;
    writeln!(file, "{},{},{},{},{},{},{}", "time", "y1", "y2", "y3", "y4", "y5", "y6")?;

    for (time, state) in x.iter().zip(y.iter()) {
        writeln!(file, "{},{},{},{},{},{},{}", time, state[0], state[1], state[2], state[3], state[4], state[5])?;
    }

    Ok(())
}




