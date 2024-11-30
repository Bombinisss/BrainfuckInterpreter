use egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2};
use egui_file_dialog::FileDialog;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// We derive Deserialize/Serialize, so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BrainfuckInterpreterInterface {
    path: String,
    #[serde(skip)]
    file_dialog: FileDialog,
    #[serde(skip)]
    letter_index: Arc<Mutex<i64>>,
    #[serde(skip)]
    last_update_time: Instant,
    #[serde(skip)]
    input_text: Arc<Mutex<String>>,
    #[serde(skip)]
    input_brainfuck: Arc<Mutex<String>>,
    #[serde(skip)]
    output: Arc<Mutex<String>>,
    #[serde(skip)]
    data: Arc<Mutex<Vec<i32>>>,
    #[serde(skip)]
    timer_running: Arc<Mutex<bool>>,
    #[serde(skip)]
    timer_thread_handle: Option<thread::JoinHandle<()>>,
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
            letter_index: Arc::new(Mutex::new(-1)),
            last_update_time: std::time::Instant::now(),
            input_text: Arc::new(Mutex::new("".to_string())),
            input_brainfuck: Arc::new(Mutex::new("".to_string())),
            output: Arc::new(Mutex::new("".to_string())),
            data: Arc::new(Mutex::new(vec![0; 32])),
            timer_running: Arc::new(Mutex::new(false)),
            timer_thread_handle: None,
        }
    }
}

impl BrainfuckInterpreterInterface {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
    pub fn start_interpreter(&mut self) {
        // Start the timer on a separate thread if it is not already running
        let timer_running = Arc::clone(&self.timer_running);
        let data_arc = Arc::clone(&self.data);
        let letter_index_arc = Arc::clone(&self.letter_index);
        let input_brainfuck = Arc::clone(&self.input_brainfuck);
        let input_text = Arc::clone(&self.input_text);
        let output_brainfuck = Arc::clone(&self.output);
        
        if *timer_running.lock().unwrap() {
            return; // Timer is already running
        }

        *timer_running.lock().unwrap() = true;

        // Spawn a thread for the timer
        self.timer_thread_handle = Some(thread::spawn(move || {
            while *timer_running.lock().unwrap() {
                let mut letter_index = letter_index_arc.lock().unwrap().clone();
                let data_length = data_arc.lock().unwrap().len();
                letter_index = (letter_index + 1i64) % data_length as i64;
                *letter_index_arc.lock().unwrap()=letter_index;
                
                //TODO: Add brainfuck logic here
                
                thread::sleep(Duration::from_millis(100));
            }
        }));
    }
    pub fn stop_interpreter(&mut self) {
        let mut timer_running = self.timer_running.lock().unwrap();
        *timer_running = false;
        if let Some(handle) = self.timer_thread_handle.take() {
            handle.join().unwrap();
        }
    }
}

impl eframe::App for BrainfuckInterpreterInterface {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            ui.horizontal(|ui| {
                ui.heading("Brainfuck code");

                if ui.button("Run").clicked() && !*self.timer_running.lock().unwrap() {
                    self.start_interpreter();
                };
            });

            let available_size = Vec2::new(ui.available_width(), 0.0);
            ui.add_sized(
                available_size, // Use the available size
                egui::TextEdit::multiline(&mut *self.input_brainfuck.lock().unwrap())
                    .hint_text("Type brainfuck here..."),
            );

            ui.add_space(10.0);
            let box_size = 50.0;
            let highlight_color = Color32::RED;

            ui.horizontal(|ui| {
                // Left side panel: Text and Input
                ui.vertical(|ui| {
                    ui.heading("Output");
                    ui.add(
                        egui::TextEdit::multiline(&mut *self.output.lock().unwrap())
                            .hint_text("This is output-only")
                            .interactive(false),
                    );
                    ui.add_space(10.0);
                    ui.heading("Input");
                    ui.text_edit_multiline(&mut *self.input_text.lock().unwrap());
                });

                // Right side panel: Dynamic boxes
                ui.vertical(|ui| {
                    // Adjust spacing between boxes
                    ui.spacing_mut().item_spacing = egui::vec2(1.0, 1.0);

                    ui.horizontal_wrapped(|ui| {
                        for (i, value) in self.data.lock().unwrap().iter().enumerate() {
                            // Allocate space for each box
                            let (_, rect) = ui.allocate_space([box_size, box_size].into());

                            // Highlight logic for the selected box
                            let rect_color = if i == *self.letter_index.lock().unwrap() as usize {
                                highlight_color
                            } else {
                                Color32::GRAY
                            };

                            // Draw the box
                            ui.painter().rect_filled(rect, 0.0, rect_color);

                            // Draw the value in the center of the box
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                format!("{}", value),
                                egui::TextStyle::Body.resolve(ui.style()),
                                Color32::WHITE,
                            );
                        }
                    });
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });

            // Request a repaint to keep the animation going
            ctx.request_repaint();
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
        "https://github.com/Bombinisss/BrainfuckInterpreter/",
    );
    ui.separator();
}
