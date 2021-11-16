extern crate ncurses;
use ncurses::*;

use std::fs;
use std::io::{self, BufRead};

// const KEY_K_LC: i32 = 107;
// const KEY_J_LC: i32 = 106;
// const KEY_I_LC: i32 = 105;
const KEY_ENTER: i32 = 10;
const KEY_ESCAPE: i32 = 27;
// const KEY_E_LC: i32 = 101;
// const KEY_D_LC: i32 = 100;

const FILE: &str = "TASKS";

enum InputState {
    Edit,
    New,
}
enum TaskState {
    Done,
    Todo,
}

struct Ui {
    task_win: WINDOW,
    input_win: WINDOW,
}

struct Vec2d {
    x: i32,
    y: i32,
}

struct Task {
    content: String,
    state: TaskState,
}

impl Task {
    fn new(content: String, state: TaskState) -> Self {
        Self { content, state }
    }

    fn toggle_state(&mut self) {
        match self.state {
            TaskState::Done => self.state = TaskState::Todo,
            TaskState::Todo => self.state = TaskState::Done,
        }
    }
}

//TODO: revamp the ui
fn parse_item(line: &mut str, index: i32) -> Task {
    let mut new_status: TaskState = TaskState::Todo;
    match &line[..8] {
        "Todo -> " => {
            new_status = TaskState::Todo;
        }
        "Done -> " => {
            new_status = TaskState::Done;
        }
        error => {
            eprintln!(
                "Error while parsing file. Unknown expression: {} on file: {}::{}",
                error,
                FILE,
                index + 1
            );
        }
    }

    Task {
        content: line.replace(&line[..8], ""),
        state: new_status,
    }
}
fn load_file() -> Vec<Task> {
    let file = fs::File::open(FILE).unwrap();
    let mut vec: Vec<Task> = Vec::new();

    for (index, line) in io::BufReader::new(file).lines().enumerate() {
        let task = parse_item(&mut line.unwrap(), index as i32);
        vec.push(task);
    }

    vec
}
// fn save_file() -> Vec<Task> {}
impl Ui {
    fn new(task_win: WINDOW, input_win: WINDOW) -> Self {
        Self {
            task_win,
            input_win,
        }
    }

    fn stdscreen() -> Vec2d {
        let mut x = 0;
        let mut y = 0;

        getmaxyx(stdscr(), &mut y, &mut x);

        Vec2d { x, y }
    }
    fn create_task_win(max_y: i32, max_x: i32) -> WINDOW {
        let win = newwin(max_y - 10, max_x - 10, 5, 5);
        box_(win, 0, 0);
        wrefresh(win);
        win
    }

    fn create_input_win(max_x: i32, max_y: i32, prompt: &str) -> WINDOW {
        let win = newwin(1, max_x - 10, max_y - 5, 5);
        mvwprintw(win, 0, 0, prompt);
        wrefresh(win);
        win
    }

    fn read_buffer(&mut self, buffer: &mut String, mode: InputState) -> Result<Task, &str> {
        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        echo();

        let mut curs_pos: i32 = 10;
        wmove(self.input_win, 0, curs_pos);
        let mut limit = 10;

        loop {
            match mode {
                InputState::New => {
                    Ui::create_input_win(Ui::stdscreen().x, Ui::stdscreen().y, "Add Task:");
                }

                InputState::Edit => {
                    Ui::create_input_win(Ui::stdscreen().x, Ui::stdscreen().y, "Edit Task:");
                    curs_pos = 11 + buffer.len() as i32;
                    limit = 11;
                }
            }

            mvwprintw(self.input_win, 0, limit, &buffer);
            wmove(self.input_win, 0, curs_pos);

            let key: i32 = wgetch(self.input_win);

            match key {
                32..=126 => {
                    // ALPHABET LETTERS

                    buffer.push(key as u8 as char);
                    curs_pos += 1;
                }
                KEY_BACKSPACE => {
                    if curs_pos > limit {
                        buffer.pop();
                        curs_pos -= 1;
                    }
                }
                KEY_ESCAPE => {
                    //DISCARD BUFFER
                    break;
                }

                KEY_ENTER => return Ok(Task::new(buffer.to_string(), TaskState::Todo)),
                _ => {}
            }
        }

        Err("Discarded current buffer")
    }

    fn add_task(&mut self, curr_item: usize, task_list: &mut Vec<Task>, mode: InputState) {
        match mode {
            InputState::New => match self.read_buffer(&mut String::new(), mode) {
                Ok(new_task) => {
                    task_list.push(new_task);
                }
                Err(_) => {}
            },
            InputState::Edit => match self.read_buffer(&mut task_list[curr_item].content, mode) {
                Ok(new_task) => {
                    wclear(self.task_win);
                    self.task_win = Ui::create_task_win(Ui::stdscreen().y, Ui::stdscreen().x);
                    task_list[curr_item].content = new_task.content;
                }
                Err(_) => {}
            },
        }

        Ui::create_input_win(Ui::stdscreen().x, Ui::stdscreen().y, "");
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        noecho();
    }

    fn delete_task(&mut self, index: usize, task_list: &mut Vec<Task>) {
        task_list.remove(index);
        wclear(self.task_win);
        self.task_win = Ui::create_task_win(Ui::stdscreen().y, Ui::stdscreen().x);
    }
}

fn ui_loop(ui: &mut Ui) {
    keypad(ui.input_win, true);
    let mut task_list: Vec<Task> = load_file();

    let mut curr_item: i32 = 0;

    loop {
        for i in 0..task_list.len() as i32 {
            if i == curr_item {
                wattron(ui.task_win, A_REVERSE());
            }

            match task_list[i as usize].state {
                TaskState::Done => {
                    mvwprintw(
                        ui.task_win,
                        i + 1,
                        1,
                        &format!("-[X] {}", task_list[i as usize].content),
                    );
                }

                TaskState::Todo => {
                    mvwprintw(
                        ui.task_win,
                        i + 1,
                        1,
                        &format!("-[ ] {}", task_list[i as usize].content),
                    );
                }
            }
            wattroff(ui.task_win, A_REVERSE());
        }

        let choice = wgetch(ui.task_win);

        match choice as u8 as char {
            'k' => {
                if curr_item != 0 {
                    curr_item -= 1;
                }
            }

            'j' => {
                if curr_item != task_list.len() as i32 - 1 {
                    curr_item += 1;
                }
            }

            '\n' => {
                task_list[curr_item as usize].toggle_state();
            }

            'i' => {
                ui.add_task(curr_item as usize, &mut task_list, InputState::New);
            }

            'e' => {
                ui.add_task(curr_item as usize, &mut task_list, InputState::Edit);
            }

            'd' => {
                if task_list.len() as i32 > 0 {
                    ui.delete_task(curr_item as usize, &mut task_list);
                    if curr_item == 0 {
                        curr_item = 1;
                    }
                    curr_item -= 1;
                }
            }
            'q' => {
                end(0);
            }
            _ => {}
        }
    }
}

fn end(code: i32) {
    endwin();
    std::process::exit(code);
}
pub fn launch_ui() {
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let mut ui = Ui::new(
        Ui::create_task_win(Ui::stdscreen().y, Ui::stdscreen().x),
        Ui::create_input_win(Ui::stdscreen().x, Ui::stdscreen().y, ""),
    );

    ui_loop(&mut ui);
}

fn main() {
    initscr();

    cbreak();

    launch_ui();

    endwin();
}
