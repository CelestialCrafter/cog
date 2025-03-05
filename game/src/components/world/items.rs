use std::fmt;

use hecs::Entity;
use ratatui::{
    style::{Color, Style},
    text::Text,
};

#[derive(Clone, Copy)]
pub enum ZoomLevel {
    Close = 2,
    Far = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    #[default]
    Empty,

    RawIron,
    RawCopper,
    RawGold,
    RawSilver,
    RawTin,

    Pod(Entity),
    Tunnel(Entity),
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
            Self::Pod(id) => write!(f, "{:?}", id),
            Self::Tunnel(id) => write!(f, "{:?}", id),
        }
    }
}

impl Item {
    pub fn entity(&self) -> Option<&Entity> {
        match self {
            Item::Empty => None,
            Item::RawIron => None,
            Item::RawCopper => None,
            Item::RawGold => None,
            Item::RawSilver => None,
            Item::RawTin => None,
            Item::Pod(e) => Some(e),
            Item::Tunnel(e) => Some(e),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Empty => Color::Reset,
            Self::RawIron => Color::DarkGray,
            Self::RawCopper => Color::Yellow,
            Self::RawGold => Color::LightYellow,
            Self::RawSilver => Color::Gray,
            Self::RawTin => Color::LightBlue,
            Self::Pod(_) => Color::DarkGray,
            Self::Tunnel(_) => Color::White,
        }
    }

    pub fn render(&self, zoom: ZoomLevel) -> Text {
        let color = self.color();
        let bg = Style::default().bg(color);

        match zoom {
            ZoomLevel::Close => match self {
                Self::Empty => Text::raw("····\n····"),
                Self::RawIron => Text::styled("╒══╕\n╘══╛", color),
                Self::RawCopper => Text::styled("┌──┐\n└──┘", color),
                Self::RawGold => Text::styled("╭──╮\n╰──╯", color),
                Self::RawSilver => Text::styled("┏━━┓\n┗━━┛", color),
                Self::RawTin => Text::styled("┍━━┑\n┕━━┙", color),
                Self::Pod(_) => Text::styled("╔══╗\n╚══╝", color),
                Self::Tunnel(_) => Text::styled("⇅⇄⇅⇄\n⇄⇅⇄⇅", color),
            },
            ZoomLevel::Far => match self {
                Self::Empty => Text::raw("  "),
                Self::RawIron => Text::styled("▪▪", bg),
                Self::RawCopper => Text::styled("▫▫", bg),
                Self::RawGold => Text::styled("◆◆", bg),
                Self::RawSilver => Text::styled("◇◇", bg),
                Self::RawTin => Text::styled("◈◈", bg),
                Self::Pod(_) => Text::styled("  ", color),
                Self::Tunnel(_) => Text::styled("⇅⇄", color),
            },
        }
    }
}
