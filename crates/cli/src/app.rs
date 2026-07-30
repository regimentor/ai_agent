use std::io::stdout;

use anyhow::Result;
use crossterm::{
    cursor::{SetCursorStyle, Show},
    event, execute,
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Position},
    widgets::{Block, Paragraph},
};

enum InputMode {
    Normal,
    Insert,
}

pub struct App {
    user_input: Vec<String>,
    caret_position: (usize, usize),
    mode: InputMode,
}

impl App {
    pub fn new() -> Self {
        let user_input = vec![
            String::from("Hello, world!"),
            String::from("This is a test."),
        ];
        App {
            user_input,
            caret_position: (0, 0),
            mode: InputMode::Normal,
        }
    }

    fn clamp_caret(&self, x: usize, y: usize) -> (usize, usize) {
        if self.user_input.is_empty() {
            return (0, 0);
        }
        let last_line_index = self.user_input.len() - 1;
        let _y = y.clamp(0, last_line_index);
        let current_line_length = self.user_input[_y].chars().count();
        let _x = x.clamp(0, current_line_length);
        (_x, _y)
    }

    fn move_caret_left(&mut self) {
        // let cursor_moved_left = self.carret_position.saturating_sub(1);
        let (mut x, y) = self.caret_position;
        x = x.saturating_sub(1);
        self.caret_position = self.clamp_caret(x, y)
    }

    fn move_caret_right(&mut self) {
        let (mut x, y) = self.caret_position;
        x += 1;
        self.caret_position = self.clamp_caret(x, y);
    }

    fn move_caret_up(&mut self) {
        let (x, mut y) = self.caret_position;
        y = y.saturating_sub(1);
        self.caret_position = self.clamp_caret(x, y);
    }

    fn move_caret_down(&mut self) {
        let (x, mut y) = self.caret_position;
        y += 1;
        self.caret_position = self.clamp_caret(x, y);
    }

    fn paste_char(&mut self, new_char: char) {
        let paste_index = self.get_caret_byte_index();
        self.user_input[self.caret_position.1].insert(paste_index, new_char);
        self.move_caret_right();
    }

    fn delete_char(&mut self) {
        let (mut x, y) = self.caret_position;
        if x == 0 {
            return;
        }
        let end = self.get_caret_byte_index();
        x -= 1;
        self.caret_position = self.clamp_caret(x, y);
        let start = self.get_caret_byte_index();
        self.user_input[self.caret_position.1].drain(start..end);
    }

    fn get_caret_byte_index(&mut self) -> usize {
        self.user_input[self.caret_position.1]
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.caret_position.0)
            .unwrap_or(self.user_input[self.caret_position.1].len())
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Length(4), Constraint::Length(1)]);
        let [input_area, mode_info] = frame.area().layout(&layout);

        let input_text = self.user_input.join("\n");

        frame.render_widget(
            Paragraph::new(input_text).block(Block::bordered().title("Input message")),
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
            input_area.x + self.caret_position.0 as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + self.caret_position.1 as u16 + 1,
        ))
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
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
                        event::KeyCode::Char('h') => self.move_caret_left(),
                        event::KeyCode::Char('l') => self.move_caret_right(),
                        event::KeyCode::Char('k') => self.move_caret_up(),
                        event::KeyCode::Char('j') => self.move_caret_down(),
                        _ => {}
                    },
                    InputMode::Insert => match key.code {
                        event::KeyCode::Esc => {
                            self.mode = InputMode::Normal;
                            execute!(stdout(), SetCursorStyle::BlinkingBlock, Show)?;
                        }
                        event::KeyCode::Char(ch) => self.paste_char(ch),
                        event::KeyCode::Backspace => self.delete_char(),
                        event::KeyCode::Left => self.move_caret_left(),
                        event::KeyCode::Right => self.move_caret_right(),
                        event::KeyCode::Up => self.move_caret_up(),
                        event::KeyCode::Down => self.move_caret_down(),
                        _ => {}
                    },
                }
            }
        }
    }
}
