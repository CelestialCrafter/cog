use std::fmt;

use ratatui::{style::Style, text::Text};

use crate::colors;

#[derive(Clone, Copy)]
pub enum ZoomLevel {
    Close = 2,
    Far = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Item {
    #[default]
    Empty,

    RawIron,
    RawCopper,
    RawGold,
    RawSilver,
    RawTin,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty"),

            Self::RawIron => write!(f, "Raw Iron"),
            Self::RawCopper => write!(f, "Raw Copper"),
            Self::RawGold => write!(f, "Raw Gold"),
            Self::RawSilver => write!(f, "Raw Silver"),
            Self::RawTin => write!(f, "Raw Tin"),
        }
    }
}

impl Item {
    pub fn render(&self, zoom: ZoomLevel) -> Text {
        match zoom {
            ZoomLevel::Close => match self {
                Self::Empty => Text::styled("····\n····", Style::default()),
                Self::RawIron => Text::styled("╒══╕\n╘══╛", Style::default().fg(colors::IRON)),
                Self::RawCopper => Text::styled("┌──┐\n└──┘", Style::default().fg(colors::COPPER)),
                Self::RawGold => Text::styled("╭──╮\n╰──╯", Style::default().fg(colors::GOLD)),
                Self::RawSilver => Text::styled("┏━━┓\n┗━━┛", Style::default().fg(colors::SILVER)),
                Self::RawTin => Text::styled("┍━━┑\n┕━━┙", Style::default().fg(colors::TIN)),
            },
            ZoomLevel::Far => match self {
                Self::Empty => Text::raw("  "),
                Self::RawIron => Text::styled("▪▪", Style::default().bg(colors::IRON)),
                Self::RawCopper => Text::styled("▫▫", Style::default().bg(colors::COPPER)),
                Self::RawGold => Text::styled("◆◆", Style::default().bg(colors::GOLD)),
                Self::RawSilver => Text::styled("◇◇", Style::default().bg(colors::SILVER)),
                Self::RawTin => Text::styled("◈◈", Style::default().bg(colors::TIN)),
            },
        }
    }
}
