use eframe::{epi, egui};

mod camera_driver;

struct AppState {
    content: String
}

impl epi::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(self.content.as_str())
        });
    }

    fn name(&self) -> &str {
        "Image Importer"
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("App booting...");

    let app_state = AppState {
        content: "Some init content".to_string()
    };
    let mut window_options = eframe::NativeOptions::default();
    window_options.initial_window_size = Some(egui::Vec2::new(800., 600.));
    window_options.decorated = true;
    eframe::run_native(Box::new(app_state), window_options);
}
