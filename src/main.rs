use eframe::egui;

mod keypad;
use keypad::Keypad;
use log::{debug, error, info, warn};

fn main() -> eframe::Result {
    unsafe {
        std::env::set_var("RUST_LOG", "learn_rust=debug");
    };
    pretty_env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Custom Keypad App",
        options,
        Box::new(|cc| {
            // Use the dark theme
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            cc.egui_ctx.set_debug_on_hover(true);
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    keypad: Keypad,
}

impl MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            keypad: Keypad::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Custom Keypad")
            .default_pos([100.0, 100.0])
            .title_bar(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Your name: ");
                    ui.text_edit_singleline(&mut self.name);
                });
                ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                if ui
                    .add_enabled(false, egui::Button::new("Can't click this"))
                    .clicked()
                {
                    error!("button click");
                }
                if ui.button("Increment").clicked() {
                    self.age += 1;
                }
                ui.label(format!("Hello '{}', age {}", self.name, self.age));
            });
        self.keypad.show(ctx);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        self.keypad.bump_events(ctx, raw_input);
    }
}
