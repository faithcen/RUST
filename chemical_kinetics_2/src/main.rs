// https://srenevey.github.io/ode-solvers/index.html

// use ode_solvers::dopri5::*;
use ode_solvers::rk4::*;
// use ode_solvers::dop853::*;
use ode_solvers::*;
use std::path::Path;
use std::fs::File;
use std::io::{Write, Error};
use ndarray::{Array1, array};

type State = Vector3<f64>;
type Time = f64;

struct ChemicalKinetics {
    params: Array1<f64>
  }

impl ode_solvers::System<State> for ChemicalKinetics {

    fn system(&self, t: Time, y: &State, dy: &mut State) {
        let a1 = self.params[0];
        let ea1 = self.params[1];
        let a2 = self.params[2];
        let ea2 = self.params[3];
        let r = self.params[4];
        let t0 = self.params[5];
        let heating_rate = self.params[6];
        let temp = t0 + heating_rate * t;
        let k1 = arrhenius(a1, ea1, r, temp);
        let k2 = arrhenius(a2, ea2, r, temp);
        
        dy[0] = -k1*y[0];
        dy[1] = k1*y[0]-k2*y[1];
        dy[2] = k2*y[1];

        // Adjust differentials to avoid negative values in the next step
        adjust_differentials(y, dy);

    }
}  

fn main() {
    let a1 = 1e7;
    let ea1 = 90e3;
    let a2 = 1e6;
    let ea2 = 90e3;
    let r = 8.314;
    let t0 = 300.0;
    let heating_rate = 1.0;

    let params = array![a1, ea1, a2, ea2, r, t0, heating_rate];

    let system = ChemicalKinetics {params};
  
   
    let y0 = State::new(1., 0., 0.);

    // let mut stepper = Dopri5::new(system, 0.0, 1000.0, 0.1, y0, 1.0e-10, 1.0e-10);
    let mut stepper = Rk4::new(system, 0.0, y0, 1000., 0.1);
    // let mut stepper = Dop853::new(system, 0.0, 1000.0, 0.1, y0, 1.0e-10, 1.0e-10);
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



fn save(x: &[f64], y: &[Vector3<f64>], path: &Path) -> Result<(), Error> {
    let mut file = File::create(path)?;
    writeln!(file, "{},{},{},{}", "time", "y1", "y2", "y3")?;

    for (time, state) in x.iter().zip(y.iter()) {
        writeln!(file, "{},{},{},{}", time, state[0], state[1], state[2])?;
    }

    Ok(())
}

fn arrhenius(a: f64, ea: f64, r: f64, t: f64) -> f64 {
    a * (-ea / (r * t)).exp()
}

fn adjust_differentials(y: &State, dy: &mut State) {
    for (val, diff) in y.iter().zip(dy.iter_mut()) {
        if *val + *diff < 0.0 {
            *diff = -*val;  // Adjust differential to make next y value zero
        }
    }
}



