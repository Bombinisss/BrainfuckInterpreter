#[cfg(test)]
mod tests {
    use crate::BrainfuckInterpreterInterface;
    
    #[test]
    fn test_interpreter_run() {
        let mut interface = BrainfuckInterpreterInterface::default();
        *interface.input_brainfuck.lock().unwrap() = "+.".to_string();
        interface.start_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
        assert_eq!(*interface.output.lock().unwrap(), "\u{01}".to_string());
    }

    #[test]
    fn test_interpreter_with_input() {
        let mut interface = BrainfuckInterpreterInterface::default();
        *interface.input_brainfuck.lock().unwrap() = ",.".to_string();
        *interface.input_text.lock().unwrap() = "A".to_string();
        interface.start_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
        assert_eq!(*interface.output.lock().unwrap(), "A".to_string());
    }

    #[test]
    fn test_nested_loops() {
        let mut interface = BrainfuckInterpreterInterface::default();
        *interface.input_brainfuck.lock().unwrap() = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();
        interface.start_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
        assert_eq!(*interface.output.lock().unwrap(), "Hello World!\n".to_string());
    }

    #[test]
    fn test_interpreter_stop() {
        let mut interface = BrainfuckInterpreterInterface::default();
        *interface.input_brainfuck.lock().unwrap() = "+.".to_string();
        interface.start_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
        assert_eq!(*interface.timer_running.lock().unwrap(), false);
    }
}
