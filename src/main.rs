use eframe::egui;

// MAIN -------------------------------------------------------------------------------------------------------------------------
fn main() {
    // Taken from eframe documentation: https://docs.rs/eframe/latest/eframe/
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Guitar Tools",
        native_options,
        Box::new(|cc| Box::new(GuitarToolsApp::new(cc))),
    )
    .expect("Something went wrong with the app...");

    println!("Hello, world!");
}

// UI ---------------------------------------------------------------------------------------------------------------------------
// Adapted from eframe documentation: https://docs.rs/eframe/latest/eframe/
#[derive(Default)]
struct GuitarToolsApp {}

impl GuitarToolsApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for GuitarToolsApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            draw_panel(ctx);
        });
    }
}

/*
   Purpose:
   Notes:
*/
fn draw_panel(ctx: &egui::Context) {
    let mut app_mode = AppModeOptions::Home;
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Hello there!");
        ui.end_row();
        ui.horizontal(|ui| {
            ui.radio_value(&mut app_mode, AppModeOptions::Home, "Home");
            ui.radio_value(&mut app_mode, AppModeOptions::TuneByEar, "Tune by ear");
            ui.radio_value(
                &mut app_mode,
                AppModeOptions::TuneByRecording,
                "Tune by recording",
            );
        });
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
   Purpose:
   Notes:
*/
