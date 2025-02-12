pub fn width(text: &str) -> usize {
    text.lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
}

pub fn height(text: &str) -> usize {
    text.lines().count()
}
