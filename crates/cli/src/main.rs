use std::io::stdout;

use anyhow::Result;
use crossterm::{
    cursor::{SetCursorStyle, Show},
    event, execute,
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Position},
    widgets::{Block, Borders, Paragraph},
};

enum InputMode {
    Normal,
    Insert,
}

struct App {
    user_input: String,
    carret_position: usize,
    mode: InputMode,
}

impl App {
    const fn new() -> Self {
        App {
            user_input: String::new(),
            carret_position: 0,
            mode: InputMode::Normal,
        }
    }

    fn clamp_carret(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.user_input.chars().count())
    }

    fn move_carret_left(&mut self) {
        let cursor_moved_left = self.carret_position.saturating_sub(1);
        self.carret_position = self.clamp_carret(cursor_moved_left);
    }

    fn move_carret_right(&mut self) {
        let cursor_moved_right = self.carret_position.saturating_add(1);
        self.carret_position = self.clamp_carret(cursor_moved_right);
    }

    fn past_char(&mut self, new_char: char) {
        let past_index = self.get_carret_byte_index();
        self.user_input.insert(past_index, new_char);
        self.move_carret_right();
    }

    fn delete_char(&mut self) {
        if self.carret_position == 0 {
            return;
        }
        let end = self.get_carret_byte_index();
        self.carret_position -= 1;
        let start = self.get_carret_byte_index();
        self.user_input.drain(start..end);
    }

    fn get_carret_byte_index(&mut self) -> usize {
        self.user_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.carret_position)
            .unwrap_or(self.user_input.len())
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Length(3), Constraint::Length(1)]);
        let [input_area, mode_info] = frame.area().layout(&layout);

        frame.render_widget(
            Paragraph::new(self.user_input.as_str())
                .block(Block::bordered().title("Input message")),
            input_area,
        );

        let mode_text = match self.mode {
            InputMode::Insert => "INSERT",
            InputMode::Normal => "NORMAL",
        };

        frame.render_widget(Paragraph::new(mode_text), mode_info);

        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            input_area.x + self.carret_position as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        ))
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match self.mode {
                    InputMode::Normal => match key.code {
                        event::KeyCode::Char('q') => return Ok(()),
                        event::KeyCode::Char('i') => {
                            self.mode = InputMode::Insert;
                            execute!(stdout(), SetCursorStyle::BlinkingBar, Show)?;
                        }
                        event::KeyCode::Char('h') => self.move_carret_left(),
                        event::KeyCode::Char('l') => self.move_carret_right(),
                        _ => {}
                    },
                    InputMode::Insert => match key.code {
                        event::KeyCode::Esc => {
                            self.mode = InputMode::Normal;
                            execute!(stdout(), SetCursorStyle::BlinkingBlock, Show)?;
                        }
                        event::KeyCode::Char(ch) => self.past_char(ch),
                        event::KeyCode::Backspace => self.delete_char(),
                        _ => {}
                    },
                }
            }
        }
    }
}

fn main() {
    match ratatui::run(|terminal| App::new().run(terminal)) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e:#}");
        }
    }
}
