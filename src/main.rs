extern crate ncurses;
use ncurses::*;

const KEY_K_LC: i32 = 107;
const KEY_J_LC: i32 = 106;

enum TaskState {

  Done,
  Todo

}

struct Task { 

  content: String,
  state: TaskState

}

impl TaskState {



}
fn create_win (max_y: i32, max_x: i32) -> WINDOW {

  let win = newwin(max_y - 10, max_x - 10, 5, 5);
  box_(win, 0, 0);
  wrefresh(win);
  win

}

fn ui_loop (task_win: WINDOW) {

  let mut task_list: Vec<Task> = Vec::new();

  let mut curr_item: i32 = 0;

  let tasky = Task { content: "LOL".to_string(), state: TaskState::Done}; // test
  let tas = Task { content: "AAAAAAAAAAA".to_string(), state: TaskState::Todo}; // test
  task_list.push(tasky);
  task_list.push(tas);

  loop {

      for i in 0..(task_list.len() as i32) {
        

        if i == curr_item {

            wattron(task_win, A_REVERSE());

        }

        match task_list[i as usize].state {

          TaskState::Done => {

            mvwprintw(task_win, i+1, 1, &format!("[ X ] {}", task_list[i as usize].content));

          }

          TaskState::Todo => {

            mvwprintw(task_win, i+1, 1, &format!("[   ] {}", task_list[i as usize].content));

          }
        }
        wattroff(task_win, A_REVERSE());

      }  

      let choice = wgetch(task_win);

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

  let task_win = create_win(max_y, max_x);
  
  ui_loop(task_win);

}   

fn main()
{

  initscr();

  keypad(stdscr(), true);
  cbreak();

  launch_ui();

  endwin();

}