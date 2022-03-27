use ncurses::*;
use todo_rs::*;

// TODO(#1): persist the state of the application
// TODO(#2): add new items to TODO
// TODO(#3): delete items
// TODO: edit the items
// TODO: keep track of date when the item was DONE
// TODO: undo system

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
    let mut tab = Tab::Todo;

    let mut ui = Ui::default();

    while !quit {
        erase();

        ui.begin(0, 0);
        {
            match tab {
                Tab::Todo => {
                    ui.label("[TODO] DONE ", REGULAR_PAIR);
                    ui.label("------------", REGULAR_PAIR);
                    ui.begin_list(todo_curr);
                    for (index, todo) in todos.iter().enumerate() {
                        ui.list_element(&format!("- [ ] {}", todo), index);
                    }
                    ui.end_list();
                }
                Tab::Done => {
                    ui.label(" TODO [DONE]", REGULAR_PAIR);
                    ui.label("------------", REGULAR_PAIR);
                    ui.begin_list(done_curr);
                    for (index, done) in dones.iter().enumerate() {
                        ui.list_element(&format!("- [x] {}", done), index);
                    }
                    ui.end_list();
                }
            }
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'z' => match tab {
                Tab::Todo => list_up(&mut todo_curr),
                Tab::Done => list_up(&mut done_curr),
            },
            's' => match tab {
                Tab::Todo => list_down(&todos, &mut todo_curr),
                Tab::Done => list_down(&dones, &mut done_curr),
            },
            ' ' => match tab {
                Tab::Todo => {
                    list_transfer(&mut dones, &mut todos, &mut todo_curr);
                }
                Tab::Done => {
                    list_transfer(&mut todos, &mut dones, &mut done_curr);
                }
            },
            '\t' => {
                tab = tab.toggle();
            }
            _ => {}
        }
    }

    endwin();
}
