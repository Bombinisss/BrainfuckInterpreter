use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};
use egui_file_dialog::FileDialog;
use std::thread;
use std::time::Duration;

/// We derive Deserialize/Serialize, so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BrainfuckInterpreterInterface {
    path: String,
    #[serde(skip)]
    file_dialog: FileDialog,
    #[serde(skip)]
    letter_index: usize,
    #[serde(skip)]
    frame_count: usize,
    #[serde(skip)]
    last_update_time: std::time::Instant,
    #[serde(skip)]
    input_text: String,
    #[serde(skip)]
    input_brainfuck: String,
    #[serde(skip)]
    output: String,
}

impl Default for BrainfuckInterpreterInterface {
    fn default() -> Self {
        Self {
            // Example stuff:
            path: "".to_owned(),
            file_dialog: FileDialog::new()
                .min_size([595.0, 375.0])
                .max_size([595.0, 375.0])
                .resizable(false)
                .movable(false),
            letter_index: 0,
            frame_count: 0,
            last_update_time: std::time::Instant::now(),
            input_text: "".to_string(),
            input_brainfuck: "".to_string(),
            output: "".to_string(),
        }
    }
}

impl BrainfuckInterpreterInterface {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for BrainfuckInterpreterInterface {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.heading("Brainfuck code");
            
            let available_size = Vec2::new(ui.available_width(),0.0); // Remaining space in the panel
            ui.add_sized(
                available_size, // Use the available size
                egui::TextEdit::multiline(&mut self.input_brainfuck)
                    .hint_text("Type brainfuck here..."),
            );
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Output");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.output)
                            .hint_text("This is output-only")
                            .interactive(false),
                    );
                    ui.add_space(10.0);
                    ui.heading("Input");
                    ui.text_edit_multiline(&mut self.input_text);
                });
                
                //boxes here
                
            });
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
    ui.hyperlink_to(
                "Source code",
                "https://github.com/Bombinisss/BrainfuckInterpreter/");
    ui.separator();
}
