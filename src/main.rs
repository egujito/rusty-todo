extern crate ncurses;
use ncurses::*;

const KEY_K_LC: i32 = 107;
const KEY_J_LC: i32 = 106;
const KEY_ENTER: i32 = 10;


enum TaskState {

  Done,
  Todo

}

struct Ui {

  task_win: WINDOW,
  input_win: WINDOW,

}

struct Task { 

  content: String,
  state: TaskState

}

impl Task {

  fn new(content: String, state: TaskState) -> Self {
    Self {
      content,
      state
    }
  }

  fn toggle_state(&mut self) {
    
    match self.state {
      TaskState::Done => {
        self.state = TaskState::Todo
      }
      TaskState::Todo => {
        self.state = TaskState::Done
      }
    }
  }

}

impl Ui {

  fn new(task_win: WINDOW, input_win: WINDOW) -> Self {

    Self {
      task_win,
      input_win   
    } 

  }

  fn create_task_win (max_y: i32, max_x: i32) -> WINDOW {

    let win = newwin(max_y - 10, max_x - 10, 5, 5);
    box_(win, 0, 0);
    wrefresh(win);
    win
  
  }

  fn create_input_win (max_x: i32) -> WINDOW {

    let win = newwin(3, max_x - 10, 1, 5);
    box_(win, 0, 0); // to comment on future.
    wrefresh(win);
    win

  }

}


fn ui_loop (ui: Ui) {

  let mut task_list: Vec<Task> = Vec::new();

  let mut curr_item: i32 = 0;

  let tasky = Task::new("Lol".to_string(), TaskState::Todo);
  let tas = Task::new("hhhhhh".to_string(), TaskState::Done);

  task_list.push(tasky);
  task_list.push(tas);

  loop {

      for i in 0..(task_list.len() as i32) {
        

        if i == curr_item {

            wattron(ui.task_win, A_REVERSE());

        }

        match task_list[i as usize].state {

          TaskState::Done => {

            mvwprintw(ui.task_win, i+1, 1, &format!("[ X ] {}", task_list[i as usize].content));

          }

          TaskState::Todo => {

            mvwprintw(ui.task_win, i+1, 1, &format!("[   ] {}", task_list[i as usize].content));

          }
        }
        wattroff(ui.task_win, A_REVERSE());

      }  

      let choice = wgetch(ui.task_win);

      match choice {

          KEY_K_LC => {
            if curr_item != 0 {
              curr_item -= 1;
            }
          }

          KEY_J_LC => {
            if curr_item != (task_list.len() as i32) - 1 {
              curr_item += 1;
            }
          }

          KEY_ENTER => { 
            task_list[curr_item as usize].toggle_state();
          }
          _ => {}
      }
  }

}
pub fn launch_ui() {

  noecho();
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

  let mut max_x = 0;
  let mut max_y = 0;

  getmaxyx(stdscr(), &mut max_y, &mut max_x);

  let ui = Ui::new(Ui::create_task_win(max_y, max_x), Ui::create_input_win(max_x));

  box_(ui.input_win, 0, 0);
  wrefresh(ui.input_win);
  
  ui_loop(ui);

}   

fn main()
{

  initscr();

  keypad(stdscr(), true);
  cbreak();

  launch_ui();

  endwin();

}