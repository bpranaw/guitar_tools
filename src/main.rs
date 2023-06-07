use eframe::egui;
use rodio::{buffer::SamplesBuffer, OutputStream, Source};
use std::f32::consts::PI;

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
            volume: 50,
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
        ui.heading("Hello there!");
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

fn draw_home(ctx: &egui::Context) {}

fn draw_tune_by_ear(ctx: &egui::Context, volume: &mut i32) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add(egui::Slider::new(volume, 0..=100));
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
}

fn draw_tune_by_recording(ctx: &egui::Context) {}

#[derive(PartialEq)]
enum AppModeOptions {
    Home,
    TuneByEar,
    TuneByRecording,
}

// SOUND ------------------------------------------------------------------------------------------------------------------------
/*
   Purpose: Holds the values in Hz for guitar notes.
   Notes: Based on information from https://fretsuccess.com/what-are-the-guitar-string-frequencies/
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
        let wave = sample * f32::MAX * (volume / 100) as f32;
        source.push(wave);
    }

    println!("{}", source[100]);

    //For playing audio
    // Based on https://docs.rs/rodio/latest/rodio/ and https://docs.rs/rodio/latest/src/rodio/buffer.rs.html
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    //Creating the "Source" that rodio needs to play sounds
    let source: SamplesBuffer<f32> = SamplesBuffer::new(1, sample_rate, source);

    // Play the sound directly on the device
    stream_handle
        .play_raw(source.convert_samples())
        .expect("Something went wrong with playing the sound.");

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing
    std::thread::sleep(std::time::Duration::from_secs(duration.into()));
}
