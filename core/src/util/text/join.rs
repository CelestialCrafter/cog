use super::{
    border::Border,
    size::{height, width},
};

pub const TOP: f32 = 0.0;
pub const BOTTOM: f32 = 1.0;
pub const CENTER: f32 = 0.5;
pub const LEFT: f32 = 0.0;
pub const RIGHT: f32 = 1.0;

pub fn horizontal(pos: f32, strings: &Vec<String>) -> String {
    match &strings[..] {
        [] => return String::new(),
        [s] => return s.clone(),
        _ => (),
    };

    let max_height = strings.iter().map(|s| height(s)).max().unwrap_or(0);

    // pad
    let blocks: Vec<String> = strings
        .iter()
        .map(|s| {
            let block_height = s.lines().count();
            let extra = max_height.saturating_sub(block_height);
            let top = (extra as f32 * pos).round() as usize;
            let bottom = extra - top;

            let render = Border::uniform(' '.to_string())
                .margin_individual(top, bottom, 0, 0)
                .render(s);
            render
        })
        .collect();

    let blocks: Vec<Vec<&str>> = blocks.iter().map(|s| s.lines().collect()).collect();
    (0..max_height)
        .map(|i| blocks.iter().map(|lines| lines[i]).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn vertical(pos: f32, strings: &Vec<String>) -> String {
    match &strings[..] {
        [] => return String::new(),
        [s] => return s.clone(),
        _ => (),
    };

    let max_width = strings
        .iter()
        .flat_map(|s| s.lines().map(|line| width(line)))
        .max()
        .unwrap_or(0);

    let pad: Vec<String> = strings
        .iter()
        .map(|s| {
            let block_width = s.lines().map(|line| width(line)).max().unwrap_or(0);
            let diff = max_width.saturating_sub(block_width);
            let left = (diff as f32 * pos).round() as usize;
            let right = diff - left;

            Border::uniform(' '.to_string())
                .margin_individual(0, 0, left, right)
                .render(s)
        })
        .collect();

    pad.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join() {
        let input = vec!["a b\nc d\ne f".to_string(), "x\ny".to_string()];

        assert_eq!(horizontal(TOP, &input), "a bx\nc dy\ne f ");
        assert_eq!(vertical(RIGHT, &input), "a b\nc d\ne f\n  x\n  y");
    }
}
