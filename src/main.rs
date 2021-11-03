extern crate ncurses;
use ncurses::*;

enum TaskState {

  Done,
  Todo
}

struct Task { 

  content: String,
  state: TaskState

}

fn startup_test() -> Vec<Task>{

  let n = 10;
  let mut task_list: Vec<Task> = Vec::with_capacity(n);
  unsafe { 
    task_list.set_len(n); 
  }

  task_list

}

fn create_win (max_y: i32, max_x: i32) -> WINDOW {

  let win = newwin(max_y - 10, max_x - 10, 5, 5);
  box_(win, 0, 0);
  wrefresh(win);
  win

}

fn ui_loop (task_win: WINDOW) {

  let mut task_list = startup_test();

  for x in 0..10 {

      task_list[x].content = "".to_string();
      task_list[x].state = TaskState::Todo;
  }



  let mut curr_item: i32 = 0;

  loop {

      for i in 0..10{


        if task_list[i as usize].content != "" {

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
          

      }

      let choice: i32 = wgetch(task_win);

      match choice {
          _ => {curr_item = 0}

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

  launch_ui();

  endwin();

}