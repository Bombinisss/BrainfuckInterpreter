use egui::{Color32, Vec2};
use egui_file_dialog::FileDialog;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// We derive Deserialize/Serialize, so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BrainfuckInterpreterInterface {
    pub(crate) path: String,
    #[serde(skip)]
    file_dialog: FileDialog,
    #[serde(skip)]
    pub(crate) letter_index: Arc<Mutex<usize>>,
    #[serde(skip)]
    pub(crate) box_index: Arc<Mutex<usize>>,
    #[serde(skip)]
    last_update_time: Instant,
    #[serde(skip)]
    pub(crate) input_text: Arc<Mutex<String>>,
    #[serde(skip)]
    pub(crate) input_brainfuck: Arc<Mutex<String>>,
    #[serde(skip)]
    pub(crate) output: Arc<Mutex<String>>,
    #[serde(skip)]
    pub(crate) data: Arc<Mutex<Vec<u8>>>,
    #[serde(skip)]
    pub(crate) timer_running: Arc<Mutex<bool>>,
    #[serde(skip)]
    pub(crate) timer_thread_handle: Option<thread::JoinHandle<()>>,
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
            letter_index: Arc::new(Mutex::new(0)),
            box_index: Arc::new(Mutex::new(0)),
            last_update_time: std::time::Instant::now(),
            input_text: Arc::new(Mutex::new("".to_string())),
            input_brainfuck: Arc::new(Mutex::new("".to_string())),
            output: Arc::new(Mutex::new("".to_string())),
            data: Arc::new(Mutex::new(vec![0; 256])),
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
        let timer_running = Arc::clone(&self.timer_running);
        let data_arc = Arc::clone(&self.data);
        let box_index_arc = Arc::clone(&self.box_index);
        let input_brainfuck = Arc::clone(&self.input_brainfuck);
        let input_text = Arc::clone(&self.input_text);
        let output_brainfuck = Arc::clone(&self.output);

        if let Some(handle) = self.timer_thread_handle.take() {
            handle.join().unwrap();
        }

        if *timer_running.lock().unwrap() || input_brainfuck.lock().unwrap().is_empty() {
            return; // Timer is already running or input is empty
        }

        data_arc.lock().unwrap().fill(0);
        output_brainfuck.lock().unwrap().clear();

        *timer_running.lock().unwrap() = true;

        // Spawn a thread for the timer
        self.timer_thread_handle = Some(thread::spawn(move || {
            let mut data_pointer: usize = 0;
            let mut instruction_pointer: usize = 0;

            while *timer_running.lock().unwrap() {
                if instruction_pointer >= input_brainfuck.lock().unwrap().len() {
                    *timer_running.lock().unwrap() = false;
                    break;
                }
                let instruction = input_brainfuck
                    .lock()
                    .unwrap()
                    .chars()
                    .nth(instruction_pointer)
                    .unwrap();

                match instruction {
                    '>' => {
                        data_pointer += 1;
                        instruction_pointer += 1;
                        thread::sleep(Duration::from_millis(5));
                    }
                    '<' => {
                        data_pointer -= 1;
                        instruction_pointer += 1;
                        thread::sleep(Duration::from_millis(5));
                    }
                    '+' => {
                        data_arc.lock().unwrap()[data_pointer] += 1;
                        instruction_pointer += 1;
                        thread::sleep(Duration::from_millis(5));
                    }
                    '-' => {
                        data_arc.lock().unwrap()[data_pointer] -= 1;
                        instruction_pointer += 1;
                        thread::sleep(Duration::from_millis(5));
                    }
                    '.' => {
                        *output_brainfuck.lock().unwrap() +=
                            &*String::from(data_arc.lock().unwrap()[data_pointer] as char);
                        instruction_pointer += 1;
                        thread::sleep(Duration::from_millis(5));
                    }
                    ',' => {
                        data_arc.lock().unwrap()[data_pointer] =
                            input_text.lock().unwrap().chars().nth(0).unwrap() as u8; // TODO: Throw error on empty input or wait
                        let mut locked_text = input_text.lock().unwrap();
                        if !locked_text.is_empty() {
                            locked_text.drain(..1);
                        }
                        instruction_pointer += 1;
                        thread::sleep(Duration::from_millis(5));
                    }
                    '[' => {
                        if data_arc.lock().unwrap()[data_pointer] == 0 {
                            let mut bracket_nesting = 1;
                            let code_length = input_brainfuck.lock().unwrap().len();
                            let mut pos = instruction_pointer + 1;
                            while pos < code_length && bracket_nesting > 0 {
                                let instruction =
                                    input_brainfuck.lock().unwrap().chars().nth(pos).unwrap();
                                if instruction == '[' {
                                    bracket_nesting += 1;
                                } else if instruction == ']' {
                                    bracket_nesting -= 1;
                                }
                                pos += 1;
                            }
                            instruction_pointer = pos;
                            thread::sleep(Duration::from_millis(5));
                        } else {
                            instruction_pointer += 1;
                            thread::sleep(Duration::from_millis(5));
                        }
                    }
                    ']' => {
                        if data_arc.lock().unwrap()[data_pointer] != 0 {
                            let mut bracket_nesting = 1;
                            if instruction_pointer == 0 {
                                break;
                            }
                            let mut pos = instruction_pointer - 1;
                            while pos > 0 && bracket_nesting > 0 {
                                let instruction =
                                    input_brainfuck.lock().unwrap().chars().nth(pos).unwrap();
                                if instruction == ']' {
                                    bracket_nesting += 1;
                                } else if instruction == '[' {
                                    bracket_nesting -= 1;
                                }
                                if pos == 0 {
                                    break;
                                }
                                pos -= 1;
                            }
                            instruction_pointer = pos + 1;
                            thread::sleep(Duration::from_millis(5));
                        } else {
                            instruction_pointer += 1;
                            thread::sleep(Duration::from_millis(5));
                        }
                    }
                    _ => {
                        instruction_pointer += 1;
                    }
                }

                *box_index_arc.lock().unwrap() = data_pointer;
            }
            return;
        }));
    }
    pub fn stop_interpreter(&mut self) {
        *self.timer_running.lock().unwrap() = false;
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

        egui::CentralPanel::default().show(ctx, |ui| { // TODO: Add scroll
            ui.horizontal(|ui| {
                ui.heading("Brainfuck code");

                if ui.button("Run").clicked() && !*self.timer_running.lock().unwrap() {
                    self.start_interpreter();
                };
                if ui.button("Stop").clicked() && *self.timer_running.lock().unwrap() {
                    self.stop_interpreter();
                };
            });

            let available_size = Vec2::new(ui.available_width(), 0.0);
            if !*self.timer_running.lock().unwrap() {
                ui.add_sized(
                    available_size,
                    egui::TextEdit::multiline(&mut *self.input_brainfuck.lock().unwrap())
                        .hint_text("Type brainfuck here..."),
                );
            } else {
                ui.add_sized(
                    available_size,
                    egui::TextEdit::multiline(&mut *self.input_brainfuck.lock().unwrap())
                        .hint_text("Type brainfuck here...")
                        .interactive(false),
                );
            }

            ui.add_space(10.0);
            let box_size = 30.0;
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
                            let (_, rect) = ui.allocate_space([box_size, box_size].into());

                            let rect_color = if i == *self.box_index.lock().unwrap() as usize {
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
