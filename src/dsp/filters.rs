use std::f32::consts::PI;

pub struct IIRLowPass {
    a1: f32,
    b0: f32,
    y1: f32,
    sample_rate: f32,
}

impl IIRLowPass {
    pub fn new(sample_rate: f32, cutoff_freq: f32) -> Self {
        let omega_c = 2.0 * PI * cutoff_freq;
        let ts = 1.0 / sample_rate;
        let alpha = (omega_c * ts / 2.0).tan();
        let b0 = 1.0 / (1.0 + alpha); // Corrected b0
        let a1 = (1.0 - alpha) / (1.0 + alpha); // Corrected a1

        Self {
            a1,
            b0,
            y1: 0.0,
            sample_rate,
        }
    }

    pub fn set_cutoff(&mut self, cutoff_freq: f32) -> () {
        let omega_c = 2.0 * PI * cutoff_freq;
        let ts = 1.0 / self.sample_rate;
        let alpha = (omega_c * ts / 2.0).tan();
        self.b0 = 1.0 / (1.0 + alpha); // Corrected b0
        self.a1 = (1.0 - alpha) / (1.0 + alpha); // Corrected a1
        self.y1 = 0.0; // Reset the filter state when cutoff changes to avoid artifacts
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let y = self.b0 * input + self.a1 * self.y1; // Standard first-order IIR LP
        self.y1 = y;
        y
    }
}

pub fn remove_dc_offset(input: &[f32]) -> Vec<f32> {
    if input.is_empty() {
        return Vec::new();
    }

    let sum: f32 = input.iter().sum();
    let mean = sum / input.len() as f32;

    let output: Vec<f32> = input.iter().map(|&sample| sample - mean).collect();
    output
}

use std::vec::Vec;

pub struct FIRLowPass {
    coefficients: Vec<f32>,
    history: Vec<f32>,
    sample_rate: f32,
    order: usize,
}

impl FIRLowPass {
    pub fn new(cutoff_freq: f32, sample_rate: f32, order: usize) -> Self {
        let coefficients = Self::design_lowpass(cutoff_freq, sample_rate, order);
        let history = vec![0.0; order];
        FIRLowPass {
            coefficients,
            history,
            sample_rate,
            order,
        }
    }

    pub fn set_cutoff(&mut self, cutoff_freq: f32) {
        self.coefficients = Self::design_lowpass(cutoff_freq, self.sample_rate, self.order);
        self.history.resize(self.order, 0.0);
    }

    pub fn process(&mut self, input: f32) -> f32 {
        self.history.insert(0, input);
        self.history.pop(); // Keep history length consistent with order

        let mut output = 0.0;
        for i in 0..self.coefficients.len() {
            output += self.coefficients[i] * self.history[i];
        }
        output
    }

    fn design_lowpass(cutoff_freq: f32, sample_rate: f32, order: usize) -> Vec<f32> {
        if order % 2 != 0 || order < 2 {
            eprintln!(
                "Warning: FIR filter order should be an even number >= 2 for this simple design."
            );
        }

        let num_taps = order + 1;
        let mut coefficients = vec![0.0; num_taps];
        let normalized_cutoff = cutoff_freq / (sample_rate / 2.0); // Nyquist frequency

        if normalized_cutoff >= 0.0 && normalized_cutoff <= 1.0 {
            for i in 0..num_taps {
                if i == order / 2 {
                    coefficients[i] = normalized_cutoff;
                } else {
                    let m = i as f32 - order as f32 / 2.0;
                    coefficients[i] = normalized_cutoff * (m * PI).sin() / (m * PI);
                }

                // Apply a window function (Hamming window for better stopband attenuation)
                let window_value = 0.54 - 0.46 * (2.0 * PI * i as f32 / order as f32).cos();
                coefficients[i] *= window_value;
            }
        } else {
            eprintln!("Warning: Invalid normalized cutoff frequency.");
        }

        coefficients
    }
}
