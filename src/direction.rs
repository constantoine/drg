use std::convert::{From, Into};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    TopRight,
    Right,
    BottomRight,
    BottomLeft,
    Left,
    TopLeft,
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        let mut val = value;
        if val == -1 {
            val = 5;
        }
        match (val % 6).abs() {
            0 => Self::TopRight,
            1 => Self::Right,
            2 => Self::BottomRight,
            3 => Self::BottomLeft,
            4 => Self::Left,
            5 => Self::TopLeft,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for Direction {
    fn into(self) -> i32 {
        match self {
            Self::TopRight => 0,
            Self::Right => 1,
            Self::BottomRight => 2,
            Self::BottomLeft => 3,
            Self::Left => 4,
            Self::TopLeft => 5,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TopRight => f.write_str("↗"),
            Self::Right => f.write_str("→"),
            Self::BottomRight => f.write_str("↘"),
            Self::BottomLeft => f.write_str("↙"),
            Self::Left => f.write_str("←"),
            Self::TopLeft => f.write_str("↖"),
        }
    }
}
