pub mod dsp; // Declare the dsp module

use dsp::filters::FIRLowPass;
use nih_plug::{buffer, context, prelude::*};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct NihPlugin {
    params: Arc<NihPluginParams>,
    lpf: dsp::filters::FIRLowPass,
}

#[derive(Params)]
struct NihPluginParams {
    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "tone"]
    pub tone: FloatParam,

    #[id = "output"]
    pub output: FloatParam,
}

impl Default for NihPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(NihPluginParams::default()),
            lpf: FIRLowPass::new(48000.0, 1000.0, 20),
        }
    }
}

impl Default for NihPluginParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            tone: FloatParam::new(
                "Tone",
                1000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20.0e3,
                    factor: FloatRange::skew_factor(0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz_with_note_name(4, false))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),

            output: FloatParam::new(
                "Output",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for NihPlugin {
    const NAME: &'static str = "Clap DSP Playground";
    const VENDOR: &'static str = "Kevin Nel";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "hello@kevontheweb.net";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
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
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        let sample_rate = buffer_config.sample_rate;
        let initial_cutoff = self.params.tone.default_plain_value();
        self.lpf = FIRLowPass::new(sample_rate, initial_cutoff, 20);
        true // Return true on success
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let gain = self.params.gain.smoothed.next();
        let output_gain = self.params.output.smoothed.next();
        let tone = self.params.tone.smoothed.next();
        self.lpf.set_cutoff(tone); // Set the cutoff frequency once per buffer

        for channel_samples in buffer.iter_samples() {
            for sample in channel_samples {
                // let pre_filtered = self.lpf.process(*sample);
                let pre_filtered = *sample;
                let clipped = dsp::drives::wave_shapers::green_clipper(pre_filtered * gain);
                *sample = clipped * output_gain;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for NihPlugin {
    const CLAP_ID: &'static str = "com.legalcontent.nih-plugin";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("An example plugin, passes analog in to analog out");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

// impl Vst3Plugin for NihPlugin {
//     const VST3_CLASS_ID: [u8; 16] = *b"dosp50tubjdkek30";

//     // And also don't forget to change these categories
//     const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
//         &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
// }

nih_export_clap!(NihPlugin);
// nih_export_vst3!(NihPlugin);
