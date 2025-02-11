use std::fmt::{Display, Result, Write};

use crossterm::{
    cursor::{MoveDown, MoveLeft},
    style::Print,
    Command,
};

pub struct PrintLines<T: Display>(pub T);
impl<T: Display> Command for PrintLines<T> {
    fn write_ansi(&self, f: &mut impl Write) -> Result {
        for line in self.0.to_string().split('\n') {
            Print(line).write_ansi(f)?;
            MoveDown(1).write_ansi(f)?;
            MoveLeft(line.len() as u16).write_ansi(f)?;
        }

        Ok(())
    }
}
