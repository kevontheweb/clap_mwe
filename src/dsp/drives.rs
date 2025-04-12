pub mod wave_shapers {
    use std::f64;
    use std::vec::Vec;

    // https://www.desmos.com/calculator/he9xxaqggh
    pub fn green_clipper(x: f32) -> f32 {
        if x >= 0.0 {
            x.ln_1p() as f32 // ln(x + 1)
        } else {
            (2.65 * x).exp() - 1.0
        }
    }

    // todo:
    // https://www.desmos.com/calculator/xptpxqnz6x

    pub fn red_clipper(x: f64) -> f64 {
        let num = x / 2.0;
        let c = 0.75;
        let den = (1.0 + c * (x / 2.0 - 1.0).powf(2.0)).sqrt();
        let coef = 1.0 / 2.0;
        num / den + coef - 1.5
    }

    pub fn sigmoid(x: f64) -> f64 {
        // return 4.5 * x * (1.0 + 0.75 * (x / 4.5).powf(2.0)).sqrt();
        // return 2.0 * x * (1.0 + -4.0 * (x / 2.0).powf(2.0)).sqrt();
        // return 2.0 * (2.0 + -0.9 * x * (0.2 * (x / -0.9).powf(2.0)).sqrt()).exp().log10() - 1.8; // https://www.desmos.com/calculator/1ifson3ks6
        0.43 * (x - 0.3) * (1.0 + 0.75 * ((x - 0.3) / 0.3).powf(2.0)).sqrt()
        // let c = 1.9;
        // let p = 6.0;
        // let m = 0.9;
        // let o = 0.5;
        // return o * (o + c * x * (m * (x / c).powf(p)).sqrt()).exp().log10();
    }

    // hard clipper
    pub fn hard_clipper(x: f64, t: f64) -> f64 {
        x.min(t).max(-t)
    }

    /*
    // sine sigmoid piecewise
    pub fn h(x: &[f64]) -> Vec<f64> {
        let mut result = Vec::with_capacity(x.len());
        for &val in x {
            if val > 0.0 {
                result.push((val / (1.0 + (val - 1.0).powf(2.0)).sqrt()) - 0.5);
            } else {
                result.push((val.sin() / 8.0) - 0.5);
            }
        }
        result
    }

    // sigmoid
    pub fn j(x: f64, m: f64) -> f64 {
        1.0 / (1.0 + (-m * x).exp()) - 0.5
    }

    // tanh
    pub fn k(x: f64, a: f64, b: f64) -> f64 {
        let tanh_term = ((b * (x + 1.5) - 0.5).exp() - (b * (x + 1.5) - 0.5).neg().exp()) / ((b * (x + 1.5) - 0.5).exp() + (b * (x + 1.5) - 0.5).neg().exp()) - a.tanh();
        (1.0 / (a.tanh() + 1.0)) * (tanh_term + a.tanh()) - 0.5
    }

    // m function
    pub fn m(x: f64, c: f64) -> f64 {
        (x - 0.5) / (1.0 + c * (2.0 * x - 1.0).powf(2.0)).sqrt()
    }

    // n function
    pub fn n(x: &[f64], d: f64) -> Result<Vec<f64>, &'static str> {
        if d < -1.0 || d > 2.5 {
            return Err("d should be between -1 and 2.5");
        }
        let mut result = Vec::with_capacity(x.len());
        for &val in x {
            if val >= 0.0 {
                result.push((d * (val + 1.0).ln()) - 0.25);
            } else {
                result.push((d * val).exp() - 0.25 - 1.0);
            }
        }
        Ok(result)
    }

    // chebychev
    pub fn p(x: f64, n: i32) -> Result<f64, &'static str> {
        match n {
            1 => Ok(x),
            2 => Ok(2.0 * x.powf(2.0) - 1.0),
            3 => Ok(4.0 * x.powf(3.0) - 3.0 * x),
            4 => Ok(8.0 * x.powf(4.0) - 8.0 * x.powf(2.0) + 1.0),
            5 => Ok(16.0 * x.powf(5.0) - 20.0 * x.powf(3.0) + 5.0 * x),
            6 => Ok(32.0 * x.powf(6.0) - 48.0 * x.powf(4.0) + 18.0 * x.powf(2.0) - 1.0),
            _ => Err("n should be between 1 and 6"),
        }
    }
    */
}

pub mod tube {
    use std::f64;

    /*
    pub fn Igk(V_gk: f64) -> f64 {
        let G_g = 3.263e-4;
        let C_g = 11.00;
        let z = 1.156;
        G_g * (((C_g * V_gk).exp() + 1.0).log10() * (1.0 / C_g)).powf(z)
    }

    pub fn Ipk(V_gk: f64, V_pk: f64) -> f64 {
        let G = 1.37e-3;
        let C = 4.56;
        let y = 1.349;
        let mu = 86.9;
        G * ((((C / mu) * (V_gk * V_pk)).exp() + 1.0).log10() * (1.0 / C)).powf(y)
    }
    */

    pub fn koren() -> f64 {
        todo!("implement");
    }
}
