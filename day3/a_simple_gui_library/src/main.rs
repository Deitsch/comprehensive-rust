// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

use std::ops::Add;

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
        self.label.len()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        _ = buffer.write_str(&self.label);
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + PADDING * 2
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let top_bot = top_bot_border(self.label.width());

        _ = buffer.write_str("\n");
        _ = buffer.write_str(&top_bot);
        _ = buffer.write_str("\n| ");
        self.label.draw_into(buffer);
        _ = buffer.write_str(" |\n");
        _ = buffer.write_str(&top_bot);
        _ = buffer.write_str("\n");
    }
}

static PADDING: usize = 1;

impl Widget for Window {
    fn width(&self) -> usize {
        let widget_max_width = self.widgets.iter().map(|w| w.width()).max().unwrap_or(0);
        let title_width = self.title.chars().count();
        [widget_max_width, title_width].iter().max().unwrap_or(&0).clone()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width() + PADDING * 2; // 1 each side
        let top_bot = top_bot_border(self.width());

        
        _ = buffer.write_str(&top_bot);
        _ = buffer.write_str("\n");
        let formatted_title = format!("|{: ^width$}|", &self.title);
        _ = buffer.write_str(&formatted_title);
        _ = buffer.write_str("\n");
        _ = buffer.write_str(&top_bot);
        _ = buffer.write_str("\n");

        for w in &self.widgets{
            let mut inter_buffer = String::new();
            w.draw_into(&mut inter_buffer);
            let split = inter_buffer.split("\n").map(|line| {
                format!("|{: <width$}|\n", &line)
            });
            for s in split {
                _ = buffer.write_str(&s);
            }
        }
        _ = buffer.write_str(&top_bot);
        _ = buffer.write_str("\n");
    }
}

fn top_bot_border(len: usize) -> String {
    "+-".to_owned() + &"-".repeat(len) + "-+"
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}

// fn main() {
//     let width = 10;
//     println!("left aligned:  |{: <width$}|", "foo");
//     println!("centered:      |{: ^width$}|", "foo");
//     println!("right aligned: |{: >width$}|", "foo");
// }