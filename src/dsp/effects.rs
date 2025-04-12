use std::f32::consts::PI;
use super::filters::AllPassFilter;

#[derive(Debug, Clone)]
pub struct UniVibe {
    sample_rate: f32,
    lfo_phases: Vec<f32>,
    allpass_filters: Vec<Vec<AllPassFilter>>,
    base_delay_samples: usize,
    delay_modulation_range: f32,
}

impl UniVibe {
    pub fn new(sample_rate: f32, num_stages: usize) -> Self {
        let base_delay_ms = 5.0;
        let base_delay_samples = (sample_rate * base_delay_ms / 1000.0) as usize;
        let delay_modulation_ms = 2.0;
        let delay_modulation_range = (sample_rate * delay_modulation_ms / 1000.0) as f32;

        let mut filters = Vec::new();
        for _ in 0..num_stages {
            filters.push(vec![AllPassFilter::new(
                base_delay_samples + (delay_modulation_range * 2.0) as usize,
            )]);
        }

        UniVibe {
            sample_rate,
            lfo_phases: vec![0.0; num_stages],
            allpass_filters: filters,
            base_delay_samples,
            delay_modulation_range,
        }
    }

    pub fn reset(&mut self) {
        for stage_filters in &mut self.allpass_filters {
            for filter in stage_filters {
                filter.reset();
            }
        }
        self.lfo_phases.fill(0.0);
    }

    pub fn process(
        &mut self,
        input: f32,
        rate: f32,
        depth: f32,
        feedback: f32,
        num_stages: usize,
    ) -> f32 {
        let mut processed = input;

        for stage in 0..num_stages {
            let lfo_frequency = rate;
            let modulation_depth = depth;
            let lfo_value = (2.0 * PI * self.lfo_phases[stage]).sin();
            self.lfo_phases[stage] += lfo_frequency / self.sample_rate;
            if self.lfo_phases[stage] > 1.0 {
                self.lfo_phases[stage] -= 1.0;
            }

            let delay_offset = lfo_value * modulation_depth * self.delay_modulation_range;
            let current_delay = (self.base_delay_samples as f32 + delay_offset)
                .clamp(1.0, self.allpass_filters[stage][0].delay_buffer.len() as f32 - 1.0);
            let floor_delay = current_delay.floor() as usize;
            let frac = current_delay - floor_delay as f32;

            let filter = &mut self.allpass_filters[stage][0];
            filter.set_feedback(feedback);

            let delayed1_index = (filter.delay_index + filter.delay_buffer.len() - floor_delay)
                % filter.delay_buffer.len();
            let delayed2_index = (filter.delay_index + filter.delay_buffer.len() - floor_delay - 1 + filter.delay_buffer.len())
                % filter.delay_buffer.len();
            let delayed1 = filter.delay_buffer[delayed1_index];
            let delayed2 = filter.delay_buffer[delayed2_index];
            let interpolated_delay = delayed1 + frac * (delayed2 - delayed1);

            let output = filter.feedback * processed + interpolated_delay;
            filter.delay_buffer[filter.delay_index] = processed - filter.feedback * output;
            filter.delay_index = (filter.delay_index + 1) % filter.delay_buffer.len();
            processed = output;
        }

        processed
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.base_delay_samples = (sample_rate * 5.0 / 1000.0) as usize;
        self.delay_modulation_range = (sample_rate * 2.0 / 1000.0) as f32;
        for stage_filters in &mut self.allpass_filters {
            for filter in stage_filters {
                filter.delay_buffer.resize(self.base_delay_samples + (self.delay_modulation_range * 2.0) as usize, 0.0);
                filter.reset();
            }
        }
        self.lfo_phases.fill(0.0);
    }
}