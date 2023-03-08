use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// this is mostly ai-generated code to use as reference, so it's really bad and mostly wrong
// maybe just call the c++ code directly? idk how to port it to rust
// i also don't know how to call the c++ code from rust, though

trait GomokuInterface {
    fn brain_about(&self) -> String;
    fn brain_eval(&self, x: i32, y: i32);
    fn brain_init(&self);
    fn brain_turn(&self);
    fn brain_my(&self, x: i32, y: i32);
    fn brain_opponents(&self, x: i32, y: i32);
    fn brain_block(&self, x: i32, y: i32);
    fn brain_end(&self);
    fn brain_restart(&self);
    fn brain_takeback(&self, x: i32, y: i32) -> bool;
}

struct Gomoku {
    width: i32,              // board width
    height: i32,             // board height
    info_timeout_turn: i32,  // time for one turn in milliseconds
    info_timeout_match: i32, // time for the whole match in milliseconds
    info_time_left: i32,     // time left in milliseconds
    info_max_memory: i32,    // max memory in bytes, 0 for unlimited
    info_game_type: i32, // 0 = human opponent, 1 = AI opponent, 2 = tournament, 3 = network tournament
    info_exact5: bool,   // true = only exactly 5 in a row wins, false = 5 or more in a row wins
    info_renju: bool,    // true = renju rules, false = gomoku rules
    info_continuous: bool, // true = continuous game, false = single game
    data_folder: String, // folder for storing persistent data
    cmd: String,

    turn_semaphore: Arc<Semaphore>,
    command_semaphore: Arc<Semaphore>,
    in_turn: Arc<Mutex<bool>>,

    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl Gomoku {
    fn new(width: i32, height: i32) -> Self {
        let (sender, receiver) = mpsc::channel();

        Gomoku {
            width,
            height,
            info_timeout_turn: 30000,
            info_timeout_match: 1000000000,
            info_time_left: 1000000000,
            info_max_memory: 0,
            info_game_type: 1,
            info_exact5: false,
            info_renju: false,
            info_continuous: false,
            data_folder: String::new(),
            cmd: String::new(),

            turn_semaphore: Arc::new(Semaphore::new(0)),
            command_semaphore: Arc::new(Semaphore::new(0)),
            in_turn: Arc::new(Mutex::new(false)),

            sender,
            receiver,
        }
    }

    fn get_line(&mut self) {
        self.cmd = self.receiver.recv().unwrap();
        if self.cmd.is_empty() {
            std::process::exit(0);
        }
    }

    fn get_cmd_param(command: &str, input: &str) -> String {
        if input.starts_with(command) {
            if let Some(index) = input.find(' ') {
                return input[index + 1..].trim().to_string();
            } else {
                return command.to_string();
            }
        } else {
            return "".to_string();
        }
    }

    fn is_integer(str: &str) -> bool {
        if let Ok(num) = str.parse::<i32>() {
            return true;
        }
        return false;
    }

    /**
     * Parse params to Ints. (Parameters can be coordinates x,y and optionally who.)
     */
    fn params_to_ints() -> Vec<i32> {}
}
