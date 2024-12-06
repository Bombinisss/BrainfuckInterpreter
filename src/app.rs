use egui::{Color32, Vec2};
use egui_file_dialog::FileDialog;
use std::sync::{Arc, Mutex};
use std::{fs, thread};

/// We derive Deserialize/Serialize, so we can persist app state on shutdown.
pub struct BrainfuckInterpreterInterface {
    path: String,
    file_dialog: FileDialog,
    pub(crate) letter_index: Arc<Mutex<usize>>,
    pub(crate) box_index: Arc<Mutex<usize>>,
    pub(crate) delay: Arc<Mutex<u64>>,
    power: u32,
    counter: usize,
    pub(crate) input_text: Arc<Mutex<String>>,
    pub(crate) input_brainfuck: Arc<Mutex<String>>,
    pub(crate) output: Arc<Mutex<String>>,
    pub(crate) data: Arc<Mutex<Vec<u8>>>,
    pub(crate) timer_running: Arc<Mutex<bool>>,
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
                .movable(true),
            letter_index: Arc::new(Mutex::new(0)),
            box_index: Arc::new(Mutex::new(0)),
            delay: Arc::new(Mutex::new(5u64)),
            power: 0,
            counter: 0,
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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        Default::default()
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
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2]) // Prevent auto-shrinking of the scroll area
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading("Brainfuck code");

                        let not_running = !*self.timer_running.lock().unwrap();

                        ui.add_enabled_ui(not_running, |ui| {
                            if ui.button("Run").clicked() {
                                self.start_interpreter();
                            };
                        });

                        ui.add_enabled_ui(!not_running, |ui| {
                            if ui.button("Stop").clicked() && *self.timer_running.lock().unwrap() {
                                self.stop_interpreter();
                            };
                        });

                        ui.add_enabled_ui(not_running, |ui| {
                            if ui.button("Select File").clicked() {
                                self.file_dialog.select_file();
                                self.counter += 200;
                            }
                            if ui.button("Clear").clicked() {
                                self.input_brainfuck = Arc::new(Mutex::new("".to_string()));
                            }
                        });

                        if let Some(path) = self.file_dialog.update(ctx).selected() {
                            self.path = path
                                .to_str()
                                .unwrap_or_else(|| "Error: Invalid path")
                                .to_string();
                            self.path = self.path[4..].to_string();
                            match fs::read_to_string(self.path.clone()) {
                                Ok(content) => {
                                    // Filter symbols
                                    let filtered: String = content
                                        .chars()
                                        .filter(|c| {
                                            ['[', ']', '-', '>', '+', '<', '.', ','].contains(c)
                                        })
                                        .collect();
                                    if self.counter > 0 {
                                        self.input_brainfuck = Arc::new(Mutex::new(filtered));
                                        self.counter -= 1;
                                    }
                                }
                                Err(_e) => {}
                            }
                        }
                    });

                    let available_size = Vec2::new(ui.available_width(), 0.0);

                    if !*self.timer_running.lock().unwrap() {
                        ui.add_sized(
                            available_size,
                            egui::TextEdit::multiline(&mut *self.input_brainfuck.lock().unwrap())
                                .hint_text("Type brainfuck here...")
                                .interactive(!*self.timer_running.lock().unwrap())
                                .font(egui::FontId::new(14.0, egui::FontFamily::Monospace)),
                        );
                    } else {
                        let letter_index = *self.letter_index.lock().unwrap(); //TODO: fix lag on big delay
                        let input_brainfuck = self.input_brainfuck.lock().unwrap();
                        let text = &*input_brainfuck;

                        let start_idx = text
                            .char_indices()
                            .nth(letter_index)
                            .map(|(idx, _)| idx)
                            .unwrap_or(text.len());
                        let end_idx = text
                            .char_indices()
                            .nth(letter_index + 1)
                            .map(|(idx, _)| idx)
                            .unwrap_or(text.len());

                        // Split the text
                        let before = &text[..start_idx];
                        let highlighted = &text[start_idx..end_idx];
                        let after = &text[end_idx..];

                        // Create RichText for each segment
                        let before_text = egui::RichText::new(before);
                        let highlighted_text =
                            egui::RichText::new(highlighted).background_color(Color32::RED);
                        let after_text = egui::RichText::new(after);

                        ui.horizontal_wrapped(|ui| {
                            ui.label(before_text);
                            ui.label(highlighted_text);
                            ui.label(after_text);
                        });
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
                            ui.add_space(10.0);
                            ui.add_enabled_ui(!*self.timer_running.lock().unwrap(), |ui| {
                                ui.style_mut().spacing.slider_width = 200.0;
                                ui.add(
                                    egui::Slider::new(&mut *self.delay.lock().unwrap(), 0..=1000)
                                        .text("Delay"),
                                );
                                ui.horizontal(|ui| {
                                    ui.label(format!(
                                        "Memory size: {}",
                                        self.data.lock().unwrap().len()
                                    ));
                                    if ui.button("+").clicked() {
                                        let data_len: i64 = self.data.lock().unwrap().len() as i64;
                                        let new_size: i64 =
                                            data_len + 2usize.pow(self.power) as i64;
                                        self.data.lock().unwrap().resize(new_size as usize, 0);
                                    }
                                    if ui.button("-").clicked() {
                                        let data_len: i64 = self.data.lock().unwrap().len() as i64;
                                        let mut new_size: i64 =
                                            data_len - 2usize.pow(self.power) as i64;
                                        new_size = new_size.max(2);
                                        self.data.lock().unwrap().resize(new_size as usize, 0);
                                    }
                                    ui.style_mut().spacing.slider_width = 51.0;
                                    ui.add(
                                        egui::Slider::new(&mut self.power, 0..=16).text("Power"),
                                    );
                                })
                            });

                            ui.horizontal(|ui| {
                                let not_running = !*self.timer_running.lock().unwrap();

                                ui.add_enabled_ui(not_running, |ui| {
                                    if ui.button("Run").clicked() {
                                        self.start_interpreter();
                                    };
                                });

                                ui.add_enabled_ui(!not_running, |ui| {
                                    if ui.button("Stop").clicked()
                                        && *self.timer_running.lock().unwrap()
                                    {
                                        self.stop_interpreter();
                                    };
                                });

                                ui.add_enabled_ui(not_running, |ui| {
                                    if ui.button("Select File").clicked() {
                                        self.file_dialog.select_file();
                                        self.counter += 200;
                                    }
                                    if ui.button("Clear").clicked() {
                                        self.input_brainfuck = Arc::new(Mutex::new("".to_string()));
                                    }
                                });

                                if let Some(path) = self.file_dialog.update(ctx).selected() {
                                    self.path = path
                                        .to_str()
                                        .unwrap_or_else(|| "Error: Invalid path")
                                        .to_string();
                                    self.path = self.path[4..].to_string();
                                    match fs::read_to_string(self.path.clone()) {
                                        Ok(content) => {
                                            // Filter symbols
                                            let filtered: String = content
                                                .chars()
                                                .filter(|c| {
                                                    ['[', ']', '-', '>', '+', '<', '.', ',']
                                                        .contains(c)
                                                })
                                                .collect();
                                            if self.counter > 0 {
                                                self.input_brainfuck =
                                                    Arc::new(Mutex::new(filtered));
                                                self.counter -= 1;
                                            }
                                        }
                                        Err(_e) => {}
                                    }
                                }
                            });
                        });

                        // Right side panel: Dynamic boxes
                        ui.vertical(|ui| {
                            // Adjust spacing between boxes
                            ui.spacing_mut().item_spacing = egui::vec2(1.0, 1.0);

                            ui.horizontal_wrapped(|ui| {
                                let clip_rect = ui.clip_rect(); // Get the visible area
                                let data = self.data.lock().unwrap(); // Lock the data for access

                                for (i, value) in data.iter().enumerate() {
                                    // Allocate space for the current box and get its rectangle
                                    let (_, rect) = ui.allocate_space([box_size, box_size].into());

                                    // Check if the box is within the visible area
                                    if rect.intersects(clip_rect) {
                                        let rect_color = if i == *self.box_index.lock().unwrap() {
                                            highlight_color
                                        } else {
                                            if ctx.style().visuals.dark_mode {
                                                Color32::DARK_GRAY
                                            } else {
                                                Color32::GRAY
                                            }
                                        };

                                        // Draw the box
                                        ui.painter().rect_filled(rect, 1.2, rect_color);

                                        // Draw the value in the center of the box
                                        ui.painter().text(
                                            rect.center(),
                                            egui::Align2::CENTER_CENTER,
                                            format!("{}", value),
                                            egui::TextStyle::Body.resolve(ui.style()),
                                            if ctx.style().visuals.dark_mode {
                                                Color32::GRAY
                                            } else {
                                                Color32::WHITE
                                            },
                                        );
                                    }
                                }
                            });
                        });
                    });
                });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
        // Request a repaint to keep the animation going
        ctx.request_repaint();
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
}
