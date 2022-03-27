use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    process,
};

use ncurses::*;

pub const REGULAR_PAIR: i16 = 0;
pub const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
pub struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    pub fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested lists are not allowed!");
        self.list_curr = Some(id);
    }

    pub fn list_element(&mut self, label: &str, id: Id) -> bool {
        let id_curr = self
            .list_curr
            .expect("Not allowed to create list elements outside of lists");

        self.label(label, {
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });

        return false;
    }

    pub fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair) as u32);
        self.row += 1;
    }

    pub fn end_list(&mut self) {
        self.list_curr = None;
    }

    pub fn end(&mut self) {}
}

#[derive(Debug)]
pub enum Status {
    Todo,
    Done,
}

impl Status {
    pub fn toggle(&self) -> Self {
        match self {
            Status::Todo => Status::Done,
            Status::Done => Status::Todo,
        }
    }
}

pub fn parse_item(line: &str) -> Option<(Status, &str)> {
    let todo_prefix = "TODO: ";
    let done_prefix = "DONE: ";

    if line.starts_with(todo_prefix) {
        return Some((Status::Todo, &line[todo_prefix.len()..]));
    }

    if line.starts_with(done_prefix) {
        return Some((Status::Done, &line[done_prefix.len()..]));
    }

    return None;
}

pub fn list_up(list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1;
    }
}

pub fn list_down(list: &Vec<String>, list_curr: &mut usize) {
    if (*list_curr + 1) < list.len() {
        *list_curr += 1;
    }
}

pub fn list_transfer(
    list_dst: &mut Vec<String>,
    list_src: &mut Vec<String>,
    list_src_curr: &mut usize,
) {
    if *list_src_curr < list_src.len() {
        list_dst.push(list_src.remove(*list_src_curr));
        if *list_src_curr >= list_src.len() && list_src.len() > 0 {
            *list_src_curr = list_src.len() - 1;
        }
    }
}

pub fn load_state(todos: &mut Vec<String>, dones: &mut Vec<String>, file_path: &str) {
    let file = File::open(file_path).unwrap();
    for (index, line) in BufReader::new(file).lines().enumerate() {
        match parse_item(&line.unwrap()) {
            Some((Status::Todo, title)) => {
                todos.push(title.to_string());
            }
            Some((Status::Done, title)) => {
                dones.push(title.to_string());
            }
            None => {
                eprintln!("{}:{}: ERROR: ill-formed item line", file_path, index + 1);
                process::exit(1);
            }
        }
    }
}

pub fn save_state(todos: &Vec<String>, dones: &Vec<String>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    for todo in todos.iter() {
        writeln!(file, "TODO: {}", todo).unwrap();
    }
    for done in dones.iter() {
        writeln!(file, "DONE: {}", done).unwrap();
    }
}
