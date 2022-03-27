use std::{env, process};

use ncurses::*;
use todo_rs::*;

// TODO(#2): add new items to TODO
// TODO(#3): delete items
// TODO(#4): edit the items
// TODO(#5): keep track of date when the item was DONE
// TODO(#6): undo system
// TODO(#7): configuration file for setting keybindings
// TODO(#8): reorder the items
// TODO(#9): two panels instead of tabs

fn main() {
    let mut args = env::args();
    args.next().unwrap();

    let file_path = {
        match args.next() {
            Some(file_path) => file_path,
            None => {
                eprintln!("Usage: todo-rs <file-path>");
                eprintln!("ERROR: file path is not provided");
                process::exit(1);
            }
        }
    };

    let mut todos = Vec::<String>::new();
    let mut todo_curr: usize = 0;
    let mut dones = Vec::<String>::new();
    let mut done_curr: usize = 0;

    load_state(&mut todos, &mut dones, &file_path);

    initscr();
    noecho();

    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut tab = Status::Todo;

    let mut ui = Ui::default();

    while !quit {
        erase();

        ui.begin(Vec2::new(0, 0), LayoutKind::Horz);
        {
            ui.begin_layout(LayoutKind::Vert);
            {
                ui.label("TODO:", REGULAR_PAIR);
                for (index, todo) in todos.iter().enumerate() {
                    ui.label(
                        &format!("- [ ] {}", todo),
                        if index == todo_curr && tab == Status::Todo {
                            HIGHLIGHT_PAIR
                        } else {
                            REGULAR_PAIR
                        },
                    );
                }
            }
            ui.end_layout();

            ui.begin_layout(LayoutKind::Vert);
            {
                ui.label("DONE:", REGULAR_PAIR);
                for (index, done) in dones.iter().enumerate() {
                    ui.label(
                        &format!("- [x] {}", done),
                        if index == done_curr && tab == Status::Done {
                            HIGHLIGHT_PAIR
                        } else {
                            REGULAR_PAIR
                        },
                    );
                }
            }
            ui.end_layout();
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'z' => match tab {
                Status::Todo => list_up(&mut todo_curr),
                Status::Done => list_up(&mut done_curr),
            },
            's' => match tab {
                Status::Todo => list_down(&todos, &mut todo_curr),
                Status::Done => list_down(&dones, &mut done_curr),
            },
            '\n' => match tab {
                Status::Todo => {
                    list_transfer(&mut dones, &mut todos, &mut todo_curr);
                }
                Status::Done => {
                    list_transfer(&mut todos, &mut dones, &mut done_curr);
                }
            },
            '\t' => {
                tab = tab.toggle();
            }
            _ => {}
        }
    }

    save_state(&todos, &dones, &file_path);

    endwin();
}
