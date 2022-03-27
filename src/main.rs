use ncurses::*;
use std::cmp::min;
use todo_rs::*;

fn main() {
    initscr();
    noecho();

    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut todos: Vec<String> = vec![
        "Write the todo app".to_string(),
        "Buy a bread".to_string(),
        "Make a cup of tea".to_string(),
    ];
    let mut todo_curr: usize = 0;
    let mut dones: Vec<String> = vec![
        "Start coding".to_string(),
        "Have a breakfast".to_string(),
        "Make a cup of tea".to_string(),
    ];
    let mut done_curr: usize = 0;

    let mut ui = Ui::default();

    while !quit {
        erase();

        ui.begin(0, 0);
        {
            ui.label("TODO:", REGULAR_PAIR);
            ui.begin_list(todo_curr);
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(&format!("- [ ] {}", todo), index);
            }
            ui.end_list();

            ui.label("------------------------------", REGULAR_PAIR);

            ui.label("DONE:", REGULAR_PAIR);
            ui.begin_list(0);
            for (index, done) in dones.iter().enumerate() {
                ui.list_element(&format!("- [x] {}", done), index + 1);
            }
            ui.end_list();
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'z' => {
                if todo_curr > 0 {
                    todo_curr -= 1;
                }
            }
            's' => {
                if (todo_curr + 1) < todos.len() {
                    todo_curr += 1;
                }
            }
            ' ' => {
                if todo_curr < todos.len() {
                    dones.push(todos.remove(todo_curr));
                }
            }

            _ => {}
        }
    }

    endwin();
}
