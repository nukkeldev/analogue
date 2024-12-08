use std::collections::HashMap;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{prelude::*, DefaultTerminal};

use editor::node::Node;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App::default();
    // app.graph.nodes.insert(
    //     0,
    //     Node {
    //         ty: NodeType::Builtin(BuiltinType::ENTRY),
    //     },
    // );
    // app.graph.positions.insert(0, (0, 0));
    let result = app.run(&mut terminal);

    ratatui::restore();
    result
}

#[derive(Debug, Default)]
struct App {
    graph: Graph,

    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|f| self.draw(f))?;
            self.handle_inputs()?;
        }
        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        f.render_widget(self, f.area());
    }

    fn handle_inputs(&mut self) -> Result<()> {
        match event::read()? {
            event::Event::Key(e) if e.code == KeyCode::Char('q') => {
                self.exit = true;
            }
            _ => {}
        };

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<usize, Node>,
    positions: HashMap<usize, (i32, i32)>,
}

impl Widget for &Graph {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}
