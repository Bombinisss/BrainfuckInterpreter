#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use crate::BrainfuckInterpreterInterface;
    
    #[test]
    fn test_interpreter_run() {
        let mut interface = BrainfuckInterpreterInterface::default();
        interface.delay = Arc::new(Mutex::new(0u64));
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
        interface.delay = Arc::new(Mutex::new(0u64));
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
        interface.delay = Arc::new(Mutex::new(0u64));
        *interface.input_brainfuck.lock().unwrap() = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".to_string();
        interface.delay = Arc::new(Mutex::new(0u64));
        interface.start_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
        assert_eq!(*interface.output.lock().unwrap(), "Hello World!\n".to_string());
    }

    #[test]
    fn test_interpreter_stop() {
        let mut interface = BrainfuckInterpreterInterface::default();
        interface.delay = Arc::new(Mutex::new(0u64));
        *interface.input_brainfuck.lock().unwrap() = "+.".to_string();
        interface.start_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
        assert_eq!(*interface.timer_running.lock().unwrap(), false);
    }
    
    #[test]
    fn test_interpreter_try_break1() {
        let mut interface = BrainfuckInterpreterInterface::default();
        interface.delay = Arc::new(Mutex::new(0u64));
        *interface.input_brainfuck.lock().unwrap() = "<<<<".to_string();
        interface.start_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
    }
    
    #[test]
    fn test_interpreter_try_break2() {
        let mut interface = BrainfuckInterpreterInterface::default();
        interface.delay = Arc::new(Mutex::new(0u64));
        *interface.input_brainfuck.lock().unwrap() = "+++[,]".to_string();
        interface.start_interpreter();
        thread::sleep(Duration::from_millis(100));
        interface.stop_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
    }
    
    #[test]
    fn test_interpreter_try_break3() {
        let mut interface = BrainfuckInterpreterInterface::default();
        interface.delay = Arc::new(Mutex::new(0u64));
        *interface.input_brainfuck.lock().unwrap() = "[[]".to_string();
        interface.start_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
    }
    
    #[test]
    fn test_interpreter_try_break4() {
        let mut interface = BrainfuckInterpreterInterface::default();
        interface.delay = Arc::new(Mutex::new(0u64));
        *interface.input_brainfuck.lock().unwrap() = "+[+.]".to_string();
        interface.start_interpreter();
        thread::sleep(Duration::from_millis(100));
        interface.stop_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
    }
    
    #[test]
    fn test_interpreter_try_break5() {
        let mut interface = BrainfuckInterpreterInterface::default();
        interface.delay = Arc::new(Mutex::new(0u64));
        *interface.input_brainfuck.lock().unwrap() = "++[+.+.++]".to_string();
        interface.start_interpreter();
        thread::sleep(Duration::from_millis(100));
        interface.stop_interpreter();
        if let Some(handle) = interface.timer_thread_handle.take() {
            handle.join().unwrap();
        }
    }
}
