use vte::{Parser, Perform};

struct StripPerformer {
    output: Vec<u8>,
}

impl Perform for StripPerformer {
    fn print(&mut self, c: char) {
        self.output.extend(c.to_string().as_bytes());
    }

    fn execute(&mut self, byte: u8) {
        if byte == b'\n' {
            self.output.push(b'\n');
        }
    }
}

// @TODO remove this and find a diff solution
pub fn strip(input: &str) -> String {
    let mut performer = StripPerformer { output: Vec::new() };
    let mut parser = Parser::new();

    let processed = parser.advance_until_terminated(&mut performer, input.as_bytes());
    assert_eq!(
        processed,
        input.len(),
        "input length did not equal processed length"
    );
    String::from_utf8(performer.output).expect("performer output was not utf-8")
}

#[cfg(test)]
mod tests {
    use crossterm::style::Stylize;

    use super::*;

    #[test]
    fn test_strip() {
        assert_eq!(
            "abcdefg",
            strip(format!("{}{}", "abcd".red(), "efg".on_red()).as_str())
        );
    }
}
