#![allow(unused_imports, unused_variables, dead_code)]

use std::fmt::{Display, Formatter, Pointer, Write};

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.chars().count()
    }

    fn draw_into(&self, buffer: &mut dyn Write) {
        buffer.write_str(self.label.as_str()).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let w = self.width() + 2;
        buffer
            .write_str(format!("|{:<w$}|\n", "x").as_str())
            .unwrap();
        buffer
            .write_str(format!("|{:<32}|\n", format!("|{:^16}|", self.label)).as_str())
            .unwrap();
        buffer
            .write_str(format!("|{:<32}|\n", format!("{:^16}", "+--------------+")).as_str())
            .unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        self.title.chars().count()
    }

    fn draw_into(&self, buffer: &mut dyn Write) {
        let w = self.inner_width() + 2;
        buffer
            .write_str(format!("+{:-<w$}+\n", "-").as_str())
            .unwrap();
        buffer
            .write_str(format!("|{:^w$}|\n", self.title).as_str())
            .unwrap();
        buffer
            .write_str(format!("+{:=<w$}+\n", "=").as_str())
            .unwrap();
        buffer
            .write_str(format!("+{:-<w$}+\n", "-").as_str())
            .unwrap();
    }
}

fn main() {
    let mut window = Window::new(" Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("label")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}
