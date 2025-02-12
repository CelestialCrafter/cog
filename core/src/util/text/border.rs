use std::iter::repeat_n;

use crossterm::style::ContentStyle;

pub struct Border {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub tl: String,
    pub tr: String,
    pub bl: String,
    pub br: String,
    pub mr: usize,
    pub ml: usize,
    pub mt: usize,
    pub mb: usize,
}

impl Border {
    pub fn style(mut self, style: ContentStyle) -> Self {
        self.top = style.apply(self.top).to_string();
        self.bottom = style.apply(self.bottom).to_string();
        self.left = style.apply(self.left).to_string();
        self.right = style.apply(self.right).to_string();
        self.tl = style.apply(self.tl).to_string();
        self.tr = style.apply(self.tr).to_string();
        self.bl = style.apply(self.bl).to_string();
        self.br = style.apply(self.br).to_string();

        self
    }

    pub fn margin_uniform(self, amount: usize) -> Self {
        self.margin_orientation(amount, amount)
    }

    pub fn margin_orientation(self, horizontal: usize, vertical: usize) -> Self {
        self.margin_individual(vertical, vertical, horizontal, horizontal)
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

    pub fn uniform(text: String) -> Self {
        Self {
            top: text.clone(),
            bottom: text.clone(),
            left: text.clone(),
            right: text.clone(),
            tl: text.clone(),
            tr: text.clone(),
            bl: text.clone(),
            br: text.clone(),
            mr: 1,
            ml: 1,
            mt: 1,
            mb: 1,
        }
    }

    pub fn normal() -> Self {
        Self {
            top: '─'.to_string(),
            bottom: '─'.to_string(),
            left: '│'.to_string(),
            right: '│'.to_string(),
            tl: '┌'.to_string(),
            tr: '┐'.to_string(),
            bl: '└'.to_string(),
            br: '┘'.to_string(),
            mr: 1,
            ml: 1,
            mt: 1,
            mb: 1,
        }
    }

    pub fn rounded() -> Self {
        Self {
            top: '─'.to_string(),
            bottom: '─'.to_string(),
            left: '│'.to_string(),
            right: '│'.to_string(),
            tl: '╭'.to_string(),
            tr: '╮'.to_string(),
            bl: '╰'.to_string(),
            br: '╯'.to_string(),
            mr: 1,
            ml: 1,
            mt: 1,
            mb: 1,
        }
    }

    pub fn render(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let width = lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        let mut output = Vec::with_capacity(lines.len() + 2);

        // top
        output = repeat_n(
            format!(
                "{}{}{}",
                self.tl.repeat(self.ml),
                self.top.repeat(width),
                self.tr.repeat(self.mr)
            ),
            self.mt,
        )
        .chain(output)
        .collect();

        // left/right
        for line in lines {
            output.push(format!(
                "{}{:<width$}{}",
                self.left.repeat(self.ml),
                line,
                self.right.repeat(self.mr),
                width = width
            ));
        }

        // bottom
        output = output
            .into_iter()
            .chain(repeat_n(
                format!(
                    "{}{}{}",
                    self.bl.repeat(self.ml),
                    self.bottom.repeat(width),
                    self.br.repeat(self.mr)
                ),
                self.mb,
            ))
            .collect();

        output.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border() {
        let input = "abcd\nefg\nhijkl\nmnop";
        assert_eq!(
            "┌─────┐\n│abcd │\n│efg  │\n│hijkl│\n│mnop │\n└─────┘",
            Border::normal().render(input)
        );
        assert_eq!(
            "         \n  abcd   \n  efg    \n  hijkl  \n  mnop   \n         ",
            Border::uniform(" ".to_string())
                .margin_orientation(2, 1)
                .render(input)
        );
    }
}
