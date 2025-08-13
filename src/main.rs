use log::{debug, error, info, warn};

fn main() -> eframe::Result {
    unsafe {
        std::env::set_var("RUST_LOG", "learn_rust=debug");
    };
    pretty_env_logger::init();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "learn rust",
        native_options,
        Box::new(|_| Ok(Box::new(MyApp))),
    )
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
}
