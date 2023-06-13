use eframe::egui;
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};
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
        ui.add_space(10.0);
        ui.label("You can use the navigation bar at the top to select which mode you want the app to be in.");
        ui.label("If you would like to close out of the app, you can just the exit button that is native to your system's apps.");
        ui.add_space(10.0);
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Whats this app for?");
        ui.add_space(10.0);
        ui.label("This application is meant to help you with guitar related tasks. Currently, it only has the capability to help with your guitar tuning.");
        ui.label("You may choose to \"Tune by ear\" or \"Tune by recording\". ");
        ui.label("\"Tune by ear\" lets you choose your tuning and play specific pitches so you can tune your guitar strings accordingly by the sound.");
        ui.label("\"Tune by recording\" lets you select your tuning and record your guitar strings making noise. The application will then tell you how close your string is to the proper pitch.");
        ui.add_space(10.0);
    });
}

/*
   Purpose:
   Notes:
*/
fn draw_tune_by_ear(ctx: &egui::Context, volume: &mut i32) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("Make sure to turn your SYSTEM volume down! This can be quite loud");
        ui.add(egui::Slider::new(volume, 0..=100));
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
    });
}

/*
   Purpose:
   Notes:
*/
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

        let wave = sample * volume as f32;
        println!("{}", sample);
        source.push(wave);
    }

    println!("{}", source[100]);

    //For playing audio
    // Based on https://docs.rs/rodio/latest/rodio/ and https://docs.rs/rodio/latest/src/rodio/buffer.rs.html
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    //Creating the "Source" that rodio needs to play sounds
    let source: SamplesBuffer<f32> = SamplesBuffer::new(1, sample_rate, source);

    let sink = Sink::try_new(&stream_handle).unwrap();

    // Play the sound directly on the device
    println!("{}", sink.volume());
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
