use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::BrainfuckInterpreterInterface;

impl BrainfuckInterpreterInterface {
    pub fn start_interpreter(&mut self) {
        let timer_running = Arc::clone(&self.timer_running);
        let data_arc = Arc::clone(&self.data);
        let box_index_arc = Arc::clone(&self.box_index);
        let input_brainfuck = Arc::clone(&self.input_brainfuck);
        let input_text = Arc::clone(&self.input_text);
        let output_brainfuck = Arc::clone(&self.output);
        let delay_arc = Arc::clone(&self.delay);
        let letter_index_arc = Arc::clone(&self.letter_index);

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
            let mut sleep_switch: bool = false;
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
                    }
                    '<' => {
                        if data_pointer != 0 {
                            data_pointer -= 1;
                        }
                        instruction_pointer += 1;
                    }
                    '+' => {
                        if data_arc.lock().unwrap()[data_pointer]==255 {
                            data_arc.lock().unwrap()[data_pointer] = 0;
                        }
                        else { data_arc.lock().unwrap()[data_pointer] += 1; }
                        instruction_pointer += 1;
                    }
                    '-' => {
                        if data_arc.lock().unwrap()[data_pointer]==0 {
                            data_arc.lock().unwrap()[data_pointer] = 255;
                        }
                        else { data_arc.lock().unwrap()[data_pointer] -= 1; }
                        instruction_pointer += 1;
                    }
                    '.' => {
                        *output_brainfuck.lock().unwrap() +=
                            &*String::from(data_arc.lock().unwrap()[data_pointer] as char);
                        instruction_pointer += 1;
                    }
                    ',' => {
                        if !input_text.lock().unwrap().is_empty() {
                            data_arc.lock().unwrap()[data_pointer] =
                                input_text.lock().unwrap().chars().nth(0).unwrap() as u8;
                            let mut locked_text = input_text.lock().unwrap();
                            if !locked_text.is_empty() {
                                locked_text.drain(..1);
                            }
                            instruction_pointer += 1;
                        }
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
                        } else {
                            instruction_pointer += 1;
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
                        } else {
                            instruction_pointer += 1;
                        }
                    }
                    _ => {
                        instruction_pointer += 1;
                        sleep_switch = true;
                    }
                }

                if data_pointer >= data_arc.lock().unwrap().len() {
                    data_arc.lock().unwrap().resize(data_pointer + 1, 0);
                }
                *box_index_arc.lock().unwrap() = data_pointer;
                *letter_index_arc.lock().unwrap() = instruction_pointer;
                
                if !sleep_switch {
                    thread::sleep(Duration::from_millis(*delay_arc.lock().unwrap()));
                    sleep_switch = false;
                }
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