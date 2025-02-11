use std::fmt::{Display, Result, Write};

use crossterm::{
    style::{ContentStyle, StyledContent},
    Command,
};

use crate::ui::text::PrintLines;

pub mod text;

pub struct Border {
    inner: String,
    style: ContentStyle,
    h: char,
    v: char,
    tl: char,
    tr: char,
    bl: char,
    br: char,
}

impl Border {
    pub fn normal(inner: String, style: ContentStyle) -> Self {
        Self {
            inner,
            style,
            h: '─',
            v: '│',
            tl: '┌',
            tr: '┐',
            bl: '└',
            br: '┘',
        }
    }

    pub fn rounded(inner: String, style: ContentStyle) -> Self {
        Self {
            inner,
            style,
            h: '─',
            v: '│',
            tl: '╭',
            tr: '╮',
            bl: '╰',
            br: '╯',
        }
    }

    fn style<D: Display>(&self, content: D) -> StyledContent<D> {
        StyledContent::new(self.style, content)
    }
}

impl Command for Border {
    fn write_ansi(&self, f: &mut impl Write) -> Result {
        let split = self.inner.split('\n');
        let max_len = split
            .clone()
            .map(|l| l.len())
            .fold(0, |acc, x| if x > acc { x } else { acc });

        let v = self.style(self.v);
        let h = self.style(self.h.to_string().repeat(max_len));
        let tl = self.style(self.tl);
        let tr = self.style(self.tr);
        let bl = self.style(self.bl);
        let br = self.style(self.br);

        let pad = |line: &str| {
            let diff = max_len - line.len();
            let per_side = diff as f32 / 2.0;
            let left = per_side.ceil() as usize;
            let right = per_side.floor() as usize;

            format!("{}{}{}", " ".repeat(left), line, " ".repeat(right))
        };

        let mut output = split
            .map(|l| format!("{}{}{}", v, pad(l), v))
            .collect::<Vec<String>>()
            .join("\n");

        output = format!(
            "{}\n{}\n{}",
            format!("{}{}{}", tl, h, tr),
            output,
            format!("{}{}{}", bl, h, br)
        );

        PrintLines(output).write_ansi(f)
    }
}
