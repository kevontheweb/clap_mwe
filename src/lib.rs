pub mod dsp; // Declare the dsp module

use dsp::effects::UniVibe;
use nih_plug::prelude::*;
use std::sync::Arc;

#[derive(Params)]
struct NihPluginParams {
    #[id = "rate"]
    pub rate: FloatParam,

    #[id = "depth"]
    pub depth: FloatParam,

    #[id = "stages"]
    pub num_stages: IntParam,

    #[id = "feedback"]
    pub feedback: FloatParam,

    #[id = "mix"]
    pub mix: FloatParam,
}

struct NihPlugin {
    params: Arc<NihPluginParams>,
    sample_rate: f32,
    univibe: UniVibe,
}

impl Default for NihPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(NihPluginParams::default()),
            sample_rate: 44100.0,
            univibe: UniVibe::new(44100.0, 2), // Default number of stages
        }
    }
}

impl Default for NihPluginParams {
    fn default() -> Self {
        Self {
            rate: FloatParam::new("Rate", 0.8, FloatRange::Linear { min: 0.1, max: 5.0 }),
            depth: FloatParam::new("Depth", 0.7, FloatRange::Linear { min: 0.0, max: 1.0 }),
            num_stages: IntParam::new("Stages", 2, IntRange::Linear { min: 1, max: 4 }),
            feedback: FloatParam::new("Feedback", 0.5, FloatRange::Linear { min: 0.0, max: 0.9 }),
            mix: FloatParam::new("Mix", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl Plugin for NihPlugin {
    const NAME: &'static str = "Simple UniVibe (No GUI)";
    const VENDOR: &'static str = "kevontheweb";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "hello@kevontheweb.net";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];
    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate as f32;
        self.univibe.set_sample_rate(self.sample_rate);
        true
    }

    fn reset(&mut self) {
        self.univibe.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_samples = buffer.samples();
        let num_channels = buffer.channels();
        let rate = self.params.rate.smoothed.next();
        let depth = self.params.depth.smoothed.next();
        let feedback = self.params.feedback.smoothed.next();
        let num_stages = self.params.num_stages.value() as usize;
        let mix = self.params.mix.smoothed.next();

        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                let input = *sample;
                let processed = self
                    .univibe
                    .process(input, rate, depth, feedback, num_stages);
                let output = input * (1.0 - mix) + processed * mix;
                *sample = output;
            }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for NihPlugin {
    const CLAP_ID: &'static str = "net.kevontheweb.univibecoding";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A simple multi-stage UniVibe effect (No GUI)");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

nih_export_clap!(NihPlugin);
