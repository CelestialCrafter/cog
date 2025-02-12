use crossterm::style::{Attributes, Color, ContentStyle};

use super::{ansi::strip, size::width};

pub struct Border<'a> {
    pub t: &'a str,
    pub b: &'a str,
    pub l: &'a str,
    pub r: &'a str,
    pub tl: &'a str,
    pub tr: &'a str,
    pub bl: &'a str,
    pub br: &'a str,

    pub mr: usize,
    pub ml: usize,
    pub mt: usize,
    pub mb: usize,

    pub sr: ContentStyle,
    pub sl: ContentStyle,
    pub st: ContentStyle,
    pub sb: ContentStyle,
}

impl<'a> Default for Border<'a> {
    fn default() -> Self {
        Self {
            t: " ",
            b: " ",
            l: " ",
            r: " ",
            tl: " ",
            tr: " ",
            bl: " ",
            br: " ",
            mr: 1,
            ml: 1,
            mt: 1,
            mb: 1,
            sr: ContentStyle::default(),
            sl: ContentStyle::default(),
            st: ContentStyle::default(),
            sb: ContentStyle::default(),
        }
        .style_uniform(ContentStyle {
            foreground_color: Some(Color::Reset),
            background_color: Some(Color::Reset),
            underline_color: Some(Color::Reset),
            attributes: Attributes::none(),
        })
    }
}

impl<'a> Border<'a> {
    pub fn style_uniform(self, style: ContentStyle) -> Self {
        self.style_individual(style, style, style, style)
    }

    pub fn style_individual(
        mut self,
        top: ContentStyle,
        bottom: ContentStyle,
        left: ContentStyle,
        right: ContentStyle,
    ) -> Self {
        self.st = top;
        self.sb = bottom;
        self.sl = left;
        self.sr = right;

        self
    }

    pub fn margin_uniform(self, amount: usize) -> Self {
        self.margin_individual(amount, amount, amount, amount)
    }

    pub fn margin_individual(
        mut self,
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
    ) -> Self {
        self.mt = top;
        self.mb = bottom;
        self.ml = left;
        self.mr = right;

        self
    }

    pub fn uniform(text: &'a str) -> Self {
        Self {
            t: text,
            b: text,
            l: text,
            r: text,
            tl: text,
            tr: text,
            bl: text,
            br: text,
            ..Default::default()
        }
    }

    pub fn normal() -> Self {
        Self {
            t: "─",
            b: "─",
            l: "│",
            r: "│",
            tl: "┌",
            tr: "┐",
            bl: "└",
            br: "┘",
            ..Default::default()
        }
    }

    pub fn rounded() -> Self {
        Self {
            t: "─",
            b: "─",
            l: "│",
            r: "│",
            tl: "╭",
            tr: "╮",
            bl: "╰",
            br: "╯",
            ..Default::default()
        }
    }

    pub fn render(&self, text: &str) -> String {
        let stripped = strip(text);
        let stripped_lines: Vec<&str> = stripped.lines().collect();
        let max_width = width(&stripped);

        let lines: Vec<&str> = text.lines().collect();

        let total_lines = lines.len() + self.mt + self.mb;
        let line_length = max_width + self.ml + self.mr;
        let mut output = String::with_capacity(total_lines * line_length);

        // top
        let top = (
            &self.tl.repeat(self.ml),
            &self.t.repeat(max_width),
            &self.tr.repeat(self.mr),
        );

        for _ in 0..self.mt {
            output.push_str(top.0);
            output.push_str(top.1);
            output.push_str(top.2);
            output.push('\n');
        }

        // left/right
        for (i, (line, stripped_line)) in lines.iter().zip(stripped_lines).enumerate() {
            output.push_str(&self.l.repeat(self.ml));
            output.push_str(line);
            output.push_str(&" ".repeat(max_width - stripped_line.len()));
            output.push_str(&self.r.repeat(self.mr));
            if i < lines.len() - 1 {
                output.push('\n');
            }
        }

        // bottom
        let bottom = (
            &self.bl.repeat(self.ml),
            &self.b.repeat(max_width),
            &self.br.repeat(self.mr),
        );

        for _ in 0..self.mb {
            output.push('\n');
            output.push_str(bottom.0);
            output.push_str(bottom.1);
            output.push_str(bottom.2);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crossterm::style::Stylize;

    use super::*;

    #[test]
    fn test_border() {
        let input = "abcd\nefg\nhijkl\nmnop";

        assert_eq!(
            "┌─────┐\n│abcd │\n│efg  │\n│hijkl│\n│mnop │\n└─────┘",
            Border::normal().render(input),
            "general border failed"
        );

        assert_eq!(
            "         \n  abcd   \n  efg    \n  hijkl  \n  mnop   \n         ",
            Border::uniform(" ")
                .margin_individual(1, 1, 2, 2)
                .render(input),
            "margin failed"
        );

        assert_eq!(
            "┌─────┐\n│\u{1b}[38;5;9mabcd │\n│efg  │\n│hijkl│\n│mnop\u{1b}[39m │\n└─────┘",
            Border::normal().render(input.red().to_string().as_str()),
            "colored content failed"
        );
    }
}
