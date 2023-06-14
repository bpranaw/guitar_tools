use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use eframe::egui;
use realfft::RealFftPlanner;
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};
use std::{
    error::Error,
    f32::consts::PI,
    fs,
    sync::{Arc, Mutex},
};

const SPACING: f32 = 10.0;

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
}

impl Default for GuitarToolsApp {
    fn default() -> Self {
        Self {
            app_mode: AppModeOptions::Home,
            volume: 1,
        }
    }
}

impl eframe::App for GuitarToolsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        draw_menu(&mut self.app_mode, ctx);

        match self.app_mode {
            AppModeOptions::Home => draw_home(ctx),
            AppModeOptions::TuneByEar => draw_tune_by_ear(ctx, &mut self.volume),
            AppModeOptions::TuneByRecording => draw_tune_by_recording(ctx),
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
        ui.label("\"Tune by ear\" lets you choose your tuning and play specific pitches so you can tune your guitar strings accordingly by the sound.");
        ui.label("\"Tune by recording\" lets you select your tuning and record your guitar strings making noise. The application will then tell you how close your string is to the proper pitch.");
        ui.add_space(SPACING);
    });
}

/*
   Purpose: This function displays the "Tune by ear" page of the application
   Notes: The tunings listen here are some of the common tunings found on https://muted.io/guitar-tuning-chart/ and https://theacousticguitarist.com/alternate-tunings-for-acoustic-guitar/
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
   Purpose:
   Notes:
*/
fn draw_tune_by_recording(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Instructions:");
        ui.label(
            "Play a string on your guitar and press the button which you would like to tune it to.",
        );
        ui.label(
            "Your audio is recorded, and processed. After, the program will tell you how close you are to the proper pitch.",
        );
        ui.add_space(SPACING);
        //Standard Tuning -------------------------------------------------------------------------
        ui.label("Standard Tuning:");
        //This allows the buttons to be horizontally placed left to right
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("E").clicked() { 
                let audio = obtain_audio().unwrap();
                // To test the audo
                let sample_rate: u32 = audio.sample_rate;
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let source: SamplesBuffer<f32> = SamplesBuffer::new(1, sample_rate, audio.samples);
                let sink = Sink::try_new(&stream_handle).unwrap();
                sink.append(source);
                sink.sleep_until_end();
            }
            if ui.button("A").clicked() {
                generate_fourier_transform();
            }
            if ui.button("D").clicked() {

            }
            if ui.button("G").clicked() {

            }
            if ui.button("B").clicked() {

            }
            if ui.button("e").clicked() {

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

        let wave = sample * volume as f32;
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
    sample_rate: u32,
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
        sample_rate: supported_config.sample_rate().0,
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
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    drop(stream);
    let clip = clip.lock().unwrap().take().unwrap();

    eprintln!("Recorded {} samples", clip.samples.len());
    Ok(clip)
}

// Sound Analysis ---------------------------------------------------------------------------------------------------------------

/*
   Purpose: This takes in a vector representing a signal waveform, performs a fourier transform and outputs the spectrogram
   Notes: This section is based on the documents in https://docs.rs/realfft/3.3.0/realfft/
*/
fn generate_fourier_transform() {
    // JUST FOR TESTING ------------
    let sample_rate: u32 = 48000;
    let duration: u32 = 1;
    //Casts enum to f32
    let frequency = Note::E2 as i16 as f32;

    let mut source: Vec<f64> = vec![];

    //Builds Note audio
    for t in (0..(sample_rate * duration)).map(|x| x as f32 / sample_rate as f32) {
        let sample = (t * frequency * 2.0 * PI).sin();
        source.push(sample as f64);
    }
    // JUST FOR TESTING ------------

    // make a planner
    let mut real_planner = RealFftPlanner::<f64>::new();

    // create a FFT
    let r2c = real_planner.plan_fft_forward(source.len());

    // make a vector for storing the spectrum
    let mut spectrum = r2c.make_output_vec();

    // forward transform the signal
    r2c.process(&mut source, &mut spectrum).unwrap();

    // create an inverse FFT
    let c2r = real_planner.plan_fft_inverse(source.len());

    // create a vector for storing the output
    let mut outdata = c2r.make_output_vec();

    // inverse transform the spectrum back to a real-valued signal
    c2r.process(&mut spectrum, &mut outdata).unwrap();

    let mut data: String = String::new();
    for t in 0..outdata.len() {
        outdata[t] = outdata[t] * (1.0 / outdata.len() as f64);
        data += t.to_string().as_str();
        data += " : ";
        data += outdata[t].to_string().as_str();
        data += "\n";
        if t == 82 {
            println!("{} : {}", t, outdata[t]);
        }
    }

    fs::write("test.txt", data);
}
