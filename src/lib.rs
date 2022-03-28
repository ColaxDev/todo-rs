use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader, Write},
    ops::{Add, Mul},
    process,
};

use ncurses::*;

pub const REGULAR_PAIR: i16 = 0;
pub const HIGHLIGHT_PAIR: i16 = 1;

#[derive(Default, Clone, Copy)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub enum LayoutKind {
    Vert,
    Horz,
}

struct Layout {
    kind: LayoutKind,
    pos: Vec2,
    size: Vec2,
}

impl Layout {
    fn new(kind: LayoutKind, pos: Vec2) -> Self {
        Self {
            kind,
            pos,
            size: Vec2::new(0, 0),
        }
    }

    fn availible_pos(&self) -> Vec2 {
        use LayoutKind::*;
        match self.kind {
            Horz => self.pos + self.size * Vec2::new(1, 0),
            Vert => self.pos + self.size * Vec2::new(0, 1),
        }
    }

    fn add_widget(&mut self, size: Vec2) {
        use LayoutKind::*;
        match self.kind {
            Horz => {
                self.size.x += size.x;
                self.size.y = cmp::max(self.size.y, size.y)
            }
            Vert => {
                self.size.x = cmp::max(self.size.x, size.x);
                self.size.y += size.y;
            }
        }
    }
}

#[derive(Default)]
pub struct Ui {
    layouts: Vec<Layout>,
}

impl Ui {
    pub fn begin(&mut self, pos: Vec2, kind: LayoutKind) {
        assert!(self.layouts.is_empty());
        self.layouts.push(Layout {
            kind,
            pos,
            size: Vec2::new(0, 0),
        });
    }

    pub fn begin_layout(&mut self, kind: LayoutKind) {
        let layout = self
            .layouts
            .last()
            .expect("Can't create a layout outside of Ui::begin() and Ui::end()");
        let pos = layout.availible_pos();
        self.layouts.push(Layout::new(kind, pos));
    }

    pub fn end_layout(&mut self) {
        let layout = self
            .layouts
            .pop()
            .expect("Unbalanced Ui::begin_layout() and Ui::end_layout calls");
        self.layouts
            .last_mut()
            .expect("Unbalanced Ui::begin_layout() and Ui::end_layout calls")
            .add_widget(layout.size);
    }

    pub fn label_fixed_width(&mut self, text: &str, width: i32, pair: i16) {
        let layout = self
            .layouts
            .last_mut()
            .expect("Trying to render label outside of any layout");
        let pos = layout.availible_pos();

        mv(pos.y, pos.x);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair) as u32);

        layout.add_widget(Vec2::new(width, 1));
    }

    pub fn label(&mut self, text: &str, pair: i16) {
        self.label_fixed_width(text, text.len() as i32, pair);
    }

    pub fn end(&mut self) {
        self.layouts
            .pop()
            .expect("Unbalanced Ui::begin() and Ui::end() calls");
    }
}

#[derive(Debug, PartialEq)]
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
    let todo_item = line
        .strip_prefix("TODO: ")
        .map(|title| (Status::Todo, title));
    let done_item = line
        .strip_prefix("DONE: ")
        .map(|title| (Status::Done, title));

    todo_item.or(done_item)
}

pub fn list_drag_up(list: &mut [String], list_curr: &mut usize) {
    if *list_curr > 0 {
        list.swap(*list_curr, *list_curr - 1);
        *list_curr -= 1;
    }
}

pub fn list_drag_down(list: &mut [String], list_curr: &mut usize) {
    if (*list_curr + 1) < list.len() {
        list.swap(*list_curr, *list_curr + 1);
        *list_curr += 1;
    }
}

pub fn list_up(list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1;
    }
}

pub fn list_down(list: &[String], list_curr: &mut usize) {
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
        if *list_src_curr >= list_src.len() && !list_src.is_empty() {
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

pub fn save_state(todos: &[String], dones: &[String], file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    for todo in todos.iter() {
        writeln!(file, "TODO: {}", todo).unwrap();
    }
    for done in dones.iter() {
        writeln!(file, "DONE: {}", done).unwrap();
    }
}
