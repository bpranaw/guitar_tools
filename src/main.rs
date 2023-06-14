use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use eframe::egui;
use realfft::RealFftPlanner;
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};
use std::{
    cmp::Ordering,
    error::Error,
    f32::consts::PI,
    sync::{Arc, Mutex},
};

const SPACING: f32 = 10.0;
const VOLUME_SCALER: f32 = 0.1;
const HARMONIC_GUARD: usize = 20;

// MAIN -------------------------------------------------------------------------------------------------------------------------
fn main() -> Result<(), eframe::Error> {
    // Taken from eframe documentation: https://docs.rs/eframe/latest/eframe/
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Guitar Tools",
        native_options,
        Box::new(|_cc| Box::<GuitarToolsApp>::default()),
    )
}

// UI ---------------------------------------------------------------------------------------------------------------------------
// Adapted from eframe documentation: https://docs.rs/eframe/latest/eframe/
struct GuitarToolsApp {
    app_mode: AppModeOptions,
    volume: i32,
    tuning_result: String,
}

impl Default for GuitarToolsApp {
    fn default() -> Self {
        Self {
            app_mode: AppModeOptions::Home,
            volume: 10,
            tuning_result: "Result: N/A".to_string(),
        }
    }
}

impl eframe::App for GuitarToolsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        draw_menu(&mut self.app_mode, ctx);

        match self.app_mode {
            AppModeOptions::Home => draw_home(ctx),
            AppModeOptions::TuneByEar => draw_tune_by_ear(ctx, &mut self.volume),
            AppModeOptions::TuneByRecording => draw_tune_by_recording(ctx, &mut self.tuning_result),
        }
    }
}

/*
   Purpose: This draws the top-most "menu" that allows users to select which portion of the app they would like to use
   Notes: Default is the "Home" portion
*/
fn draw_menu(app_mode: &mut AppModeOptions, ctx: &egui::Context) {
    egui::TopBottomPanel::top("Heading Panel").show(ctx, |ui| {
        ui.heading("Navigation:");
        ui.end_row();
        ui.horizontal(|ui| {
            ui.radio_value(app_mode, AppModeOptions::Home, "Home");
            ui.radio_value(app_mode, AppModeOptions::TuneByEar, "Tune by ear");
            ui.radio_value(
                app_mode,
                AppModeOptions::TuneByRecording,
                "Tune by recording",
            );
        });
    });
}

/*
   Purpose: Draws the portion related to the "homepage" of the project
   Notes: This is just a little bit of information about how to use the application/what it's for
*/
fn draw_home(ctx: &egui::Context) {
    egui::TopBottomPanel::top("Middle Panel").show(ctx, |ui| {
        ui.heading("How to use the app:");
        ui.add_space(SPACING);
        ui.label("You can use the navigation bar at the top to select which mode you want the app to be in.");
        ui.label("If you would like to close out of the app, you can just the exit button that is native to your system's apps.");
        ui.add_space(SPACING);
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Whats this app for?");
        ui.add_space(SPACING);
        ui.label("This application is meant to help you with guitar related tasks. Currently, it only has the capability to help with your guitar tuning.");
        ui.label("You may choose to \"Tune by ear\" or \"Tune by recording\". ");
        ui.label("\"Tune by ear\" lets you play specific pitches so you can tune your guitar strings accordingly by the sound.");
        ui.label("\"Tune by recording\" lets you record your guitar strings making noise. The application will then tell you how close your string is to the proper pitch.");
        ui.label(
            "Note: Try to use the \"Tune by ear\" feature to get your guitar string as close as possible to the proper pitch. Extreme variance will cause inconsistent behavior with the \"Tune by recording\" algorithm",
        );
        ui.add_space(SPACING);
    });
}

/*
   Purpose: This function displays the "Tune by ear" page of the application
   Notes: The tunings listed here are some of the common tunings found on https://muted.io/guitar-tuning-chart/ and https://theacousticguitarist.com/alternate-tunings-for-acoustic-guitar/
*/
fn draw_tune_by_ear(ctx: &egui::Context, volume: &mut i32) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Make sure to turn your SYSTEM volume down! This can be quite loud");
        ui.label("Press one of the notes to have it be played out loud.");
        ui.label(
            "Notes are listed as: [Note] or [Note]_S or [Note]_F for sharp and flat respectively.",
        );
        ui.label(
            "If two notes are the same in a tuning, undercase denotes that it is the higher note.",
        );
        ui.add_space(SPACING);
        ui.label("Volume:");
        ui.add(egui::Slider::new(volume, 0..=100));
        ui.add_space(SPACING);
        //Standard Tuning -------------------------------------------------------------------------
        ui.label("Standard Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("E").clicked() {
                play_note(Note::E2, *volume);
            }
            if ui.button("A").clicked() {
                play_note(Note::A2, *volume);
            }
            if ui.button("D").clicked() {
                play_note(Note::D3, *volume);
            }
            if ui.button("G").clicked() {
                play_note(Note::G3, *volume);
            }
            if ui.button("B").clicked() {
                play_note(Note::B3, *volume);
            }
            if ui.button("e").clicked() {
                play_note(Note::E4, *volume);
            }
        });
        ui.add_space(SPACING);

        //Half Step Down --------------------------------------------------------------------------
        ui.label("Half Step Down Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("E_F").clicked() {
                play_note(Note::E2F, *volume);
            }
            if ui.button("A_F").clicked() {
                play_note(Note::A2F, *volume);
            }
            if ui.button("D_F").clicked() {
                play_note(Note::D3F, *volume);
            }
            if ui.button("G_F").clicked() {
                play_note(Note::G3F, *volume);
            }
            if ui.button("B_F").clicked() {
                play_note(Note::B3F, *volume);
            }
            if ui.button("e_F").clicked() {
                play_note(Note::E4F, *volume);
            }
        });
        ui.add_space(SPACING);

        //Full Step Down --------------------------------------------------------------------------
        ui.label("Full Step Down Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("D").clicked() {
                play_note(Note::D2, *volume);
            }
            if ui.button("G").clicked() {
                play_note(Note::G2, *volume);
            }
            if ui.button("C").clicked() {
                play_note(Note::C3, *volume);
            }
            if ui.button("F").clicked() {
                play_note(Note::F3, *volume);
            }
            if ui.button("A").clicked() {
                play_note(Note::A3, *volume);
            }
            if ui.button("d").clicked() {
                play_note(Note::D4, *volume);
            }
        });
        ui.add_space(SPACING);

        //Drop D Tuning ---------------------------------------------------------------------------
        ui.label("Drop D Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("D").clicked() {
                play_note(Note::D2, *volume);
            }
            if ui.button("A").clicked() {
                play_note(Note::A2, *volume);
            }
            if ui.button("d").clicked() {
                play_note(Note::D3, *volume);
            }
            if ui.button("G").clicked() {
                play_note(Note::G3, *volume);
            }
            if ui.button("B").clicked() {
                play_note(Note::B3, *volume);
            }
            if ui.button("E").clicked() {
                play_note(Note::E4, *volume);
            }
        });
        ui.add_space(SPACING);

        //Open E Tuning ---------------------------------------------------------------------------
        ui.label("Open E Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("E").clicked() {
                play_note(Note::E2, *volume);
            }
            if ui.button("B").clicked() {
                play_note(Note::B2, *volume);
            }
            if ui.button("e").clicked() {
                play_note(Note::E3, *volume);
            }
            if ui.button("G_S").clicked() {
                play_note(Note::G3S, *volume);
            }
            if ui.button("b").clicked() {
                play_note(Note::B3, *volume);
            }
            if ui.button("e4").clicked() {
                play_note(Note::E4, *volume);
            }
        });
        ui.add_space(SPACING);
    });
}

/*
   Purpose: This draws the ui for the "Tune by recording" page
   Notes: It's recommended that any usere that is using this has their strings at least somewhat near the pitch, i.e. after tuning by ear.
*/
fn draw_tune_by_recording(ctx: &egui::Context, tuning_result: &mut String) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Instructions:");
        ui.label(
            "Play a string on your guitar and press the button which you would like to tune it to.",
        );
        ui.label(
            "Your audio is recorded, and processed. After, the program will tell you how close you are to the proper pitch.",
        );
        ui.label(
            "Try to use the \"Tune by ear\" feature to get your guitar string as close as possible. Extreme variance will cause inconsistent behavior with the algorithm",
        );
        ui.label(
            "Use this as more of a confirmation that you have tuned properly.",
        );
        ui.add_space(SPACING);
        ui.label(tuning_result.as_str());
        ui.add_space(SPACING);
        //Standard Tuning -------------------------------------------------------------------------
        ui.label("Standard Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("E").clicked() {
                *tuning_result = tune_by_recording(Note::E2);
            }
            if ui.button("A").clicked() {
                *tuning_result = tune_by_recording(Note::A2);
            }
            if ui.button("D").clicked() {
                *tuning_result = tune_by_recording(Note::D3);
            }
            if ui.button("G").clicked() {
                *tuning_result = tune_by_recording(Note::G3);
            }
            if ui.button("B").clicked() {
                *tuning_result = tune_by_recording(Note::B3);
            }
            if ui.button("e").clicked() {
                *tuning_result = tune_by_recording(Note::E4);
            }
        });
        ui.add_space(SPACING);

        //Half Step Down --------------------------------------------------------------------------
        ui.label("Half Step Down Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("E_F").clicked() {
                *tuning_result = tune_by_recording(Note::E2F);
            }
            if ui.button("A_F").clicked() {
                *tuning_result = tune_by_recording(Note::A2F);
            }
            if ui.button("D_F").clicked() {
                *tuning_result = tune_by_recording(Note::D3F);
            }
            if ui.button("G_F").clicked() {
                *tuning_result = tune_by_recording(Note::G3F);
            }
            if ui.button("B_F").clicked() {
                *tuning_result = tune_by_recording(Note::B3F);
            }
            if ui.button("e_F").clicked() {
                *tuning_result = tune_by_recording(Note::E4F);
            }
        });
        ui.add_space(SPACING);

        //Full Step Down --------------------------------------------------------------------------
        ui.label("Full Step Down Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("D").clicked() {
                *tuning_result = tune_by_recording(Note::D2);
            }
            if ui.button("G").clicked() {
                *tuning_result = tune_by_recording(Note::G2);
            }
            if ui.button("C").clicked() {
                *tuning_result = tune_by_recording(Note::C3);
            }
            if ui.button("F").clicked() {
                *tuning_result = tune_by_recording(Note::F3);
            }
            if ui.button("A").clicked() {
                *tuning_result = tune_by_recording(Note::A3);
            }
            if ui.button("d").clicked() {
                *tuning_result = tune_by_recording(Note::D4);
            }
        });
        ui.add_space(SPACING);

        //Drop D Tuning ---------------------------------------------------------------------------
        ui.label("Drop D Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("D").clicked() {
                *tuning_result = tune_by_recording(Note::D2);
            }
            if ui.button("A").clicked() {
                *tuning_result = tune_by_recording(Note::A2);
            }
            if ui.button("d").clicked() {
                *tuning_result = tune_by_recording(Note::D3);
            }
            if ui.button("G").clicked() {
                *tuning_result = tune_by_recording(Note::G3);
            }
            if ui.button("B").clicked() {
                *tuning_result = tune_by_recording(Note::B3);
            }
            if ui.button("E").clicked() {
                *tuning_result = tune_by_recording(Note::E4);
            }
        });
        ui.add_space(SPACING);

        //Open E Tuning ---------------------------------------------------------------------------
        ui.label("Open E Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("E").clicked() {
                *tuning_result = tune_by_recording(Note::E2);
            }
            if ui.button("B").clicked() {
                *tuning_result = tune_by_recording(Note::B2);
            }
            if ui.button("e").clicked() {
                *tuning_result = tune_by_recording(Note::E3);
            }
            if ui.button("G_S").clicked() {
                *tuning_result = tune_by_recording(Note::G3S);
            }
            if ui.button("b").clicked() {
                *tuning_result = tune_by_recording(Note::B3);
            }
            if ui.button("e4").clicked() {
                *tuning_result = tune_by_recording(Note::E4);
            }
        });
        ui.add_space(SPACING);
    });
}

#[derive(PartialEq)]
enum AppModeOptions {
    Home,
    TuneByEar,
    TuneByRecording,
}

// SOUND ------------------------------------------------------------------------------------------------------------------------
/*
   Purpose: Holds the values in Hz for guitar notes.
   Notes: Based on information from https://fretsuccess.com/what-are-the-guitar-string-frequencies/ and https://en.wikipedia.org/wiki/Scientific_pitch_notation
*/
#[derive(Copy, Clone)]
enum Note {
    //Standard Tunings
    E2 = 82,
    A2 = 110,
    D3 = 147,
    G3 = 196,
    B3 = 247,
    E4 = 330,

    E2F = 78,
    A2F = 104,
    D3F = 138,
    G3F = 185,
    B3F = 233,
    E4F = 311,

    D2 = 73,
    G2 = 98,
    C3 = 131,
    F3 = 175,
    A3 = 220,
    D4 = 294,

    B2 = 123,
    E3 = 165,
    G3S = 208,
}

/*
   Purpose: Plays a note of the given frequency for one second
   Notes: Based on documentation: https://docs.rs/rodio/latest/rodio/
*/
fn play_note(frequency: Note, volume: i32) {
    let sample_rate: u32 = 48000;
    let duration: u32 = 1;
    //Casts enum to f32
    let frequency = frequency as i16 as f32;

    let mut source: Vec<f32> = vec![];

    //Builds Note audio
    for t in (0..(sample_rate * duration)).map(|x| x as f32 / sample_rate as f32) {
        let sample = (t * frequency * 2.0 * PI).sin();

        let wave = sample * volume as f32 * VOLUME_SCALER;
        source.push(wave);
    }

    //For playing audio
    // Based on https://docs.rs/rodio/latest/rodio/ and https://docs.rs/rodio/latest/src/rodio/buffer.rs.html
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    //Creating the "Source" that rodio needs to play sounds
    let source: SamplesBuffer<f32> = SamplesBuffer::new(1, sample_rate, source);

    let sink = Sink::try_new(&stream_handle).unwrap();

    // Play the sound directly on the device
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}

// Audio Recording --------------------------------------------------------------------------------------------------------------
// This portion is based on the "cpal:recording" section of https://www.youtube.com/watch?v=ZweInbMBsa4
struct AudioData {
    samples: Vec<f32>,
}

/*
   Purpose: This is intended to record a vibrating guitar string and return the recorded audio so it can be processed.
   Notes: This is based on https://docs.rs/cpal/0.15.2/cpal/, https://github.com/RustAudio/cpal/blob/master/examples/record_wav.rs#L129 and https://www.youtube.com/watch?v=ZweInbMBsa4
*/
fn obtain_audio() -> Result<AudioData, Box<dyn Error>> {
    // We need to get the default audio devices/configs
    // Currently it just panics if that doesn't work. That chnage will come later but my goal is getting the prototype to work right now.

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no input device available");

    let mut supported_configs_range = device
        .supported_input_configs()
        .expect("error while querying configs");

    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    // Making the structure that will store the recording
    let clip = AudioData {
        samples: Vec::new(),
    };

    let clip = Arc::new(Mutex::new(Some(clip)));
    let clip_2 = clip.clone();

    type ClipHandler = Arc<Mutex<Option<AudioData>>>;
    let channels = supported_config.channels();
    /*
       Notes: These were taken from https://github.com/RustAudio/cpal/blob/master/examples/record_wav.rs#L129 and https://www.youtube.com/watch?v=ZweInbMBsa4
    */
    fn write_input_data<T>(input: &[T], channels: u16, writer: &ClipHandler)
    where
        T: cpal::Sample,
    {
        if let Ok(mut guard) = writer.try_lock() {
            if let Some(clip) = guard.as_mut() {
                for frame in input.chunks(channels.into()) {
                    clip.samples.push(frame[0].to_f32());
                }
            }
        }
    }

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };

    //Does the writing based on the device data type
    let stream = match supported_config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &supported_config.into(),
            move |data, _: &_| write_input_data::<f32>(data, channels, &clip_2),
            err_fn,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &supported_config.into(),
            move |data, _: &_| write_input_data::<i16>(data, channels, &clip_2),
            err_fn,
        )?,
        cpal::SampleFormat::U16 => device.build_input_stream(
            &supported_config.into(),
            move |data, _: &_| write_input_data::<u16>(data, channels, &clip_2),
            err_fn,
        )?,
    };

    //Starts the recording, and keeps it going for one second
    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    drop(stream);
    let clip = clip.lock().unwrap().take().unwrap();

    Ok(clip)
}

// Sound Analysis ---------------------------------------------------------------------------------------------------------------

/*
   Purpose: This takes in a vector representing a signal waveform, performs a fourier transform and outputs the spectrogram (real normalized not complex)
   Notes: This section is based on the documents in https://docs.rs/realfft/3.3.0/realfft/ and took inspiration from the source code at https://docs.rs/audioviz/latest/src/audioviz/fft.rs.html#55-62
*/
fn generate_fourier_transform(audio: AudioData) -> Vec<f64> {
    // We need to converet the samples into f64
    let mut source: Vec<f64> = Vec::new();
    for i in &audio.samples {
        source.push(*i as f64);
    }

    // make a planner
    let mut real_planner = RealFftPlanner::<f64>::new();

    // create a FFT
    let r2c = real_planner.plan_fft_forward(source.len());

    // make a vector for storing the spectrum
    let mut spectrum = r2c.make_output_vec();

    // forward transform the signal
    r2c.process(&mut source, &mut spectrum).unwrap();

    let mut data: Vec<f64> = Vec::new();

    //Normalizes complex to real
    //Theoretically the index at our desired frequency should have the highest magnitude.
    for i in &spectrum {
        data.push(i.norm());
    }
    data
}

// Putting Note Call to Frequency output all together ---------------------------------------------------------------------------
/*
   Purpose: Finds the greatest absolute value in the given vector and outputs the index for it
   Notes: In theory, the outputted index should be the most prominent frequency in the spectrogram
          During testing, it seems that the second and third harmonics keep getting picked up. For example, a perfectly tuned E2 string (82 Hz) would pick up 164 Hz, 246 Hz etc.
          My current solution to this is to limit where the data is gathered, but that assumes that the string is already somewhere in the ballpark of the pitch...
          That's why there's a target note in here.
*/
fn find_greatest(data: Vec<f64>, target_note: Note) -> usize {
    let mut index = 0;
    let mut greatest: f64 = 0.0;

    // This is technically bad because in theory someone could pass a target note of 4 -> 4 * 2 < 10, but none of the notes are currently like that.
    // Minimum is currently 82, so subtracting the harmonic_guard is not a problem
    let target_range = target_note as usize * 2 - HARMONIC_GUARD;

    for (i, x) in data.iter().enumerate() {
        if i >= target_range {
            break;
        }
        if x.abs() > greatest.abs() {
            greatest = data[i];
            index = i;
        }
    }
    index
}

/*
   Purpose: Records audio, runs it through a fourier transformation and determines the most prominent frequency. Then outputs a string telling the user whether they need to tune up or down
   Notes: This will be sent to the gui to update the internal result string.
*/
fn tune_by_recording(note: Note) -> String {
    let mut result = String::new();

    let audio = obtain_audio().unwrap();
    let spectrogram = generate_fourier_transform(audio);
    let index = find_greatest(spectrogram, note);

    let note = note as u32;
    let index = index as u32;

    result += "Result: (Target Pitch: ";
    result += note.to_string().as_str();
    result += " Hz Recorded Pitch: ";
    result += index.to_string().as_str();
    result += " Hz): ";

    match note.cmp(&index) {
        Ordering::Less => result += "\"You shoud loosen your string!\"",
        Ordering::Greater => result += "\"You should tighen your string!\"",
        Ordering::Equal => result += "\"Perfect!\"",
    }

    result
}
