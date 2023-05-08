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
            0 => Direction::TopRight,
            1 => Direction::Right,
            2 => Direction::BottomRight,
            3 => Direction::BottomLeft,
            4 => Direction::Left,
            5 => Direction::TopLeft,
            _ => unreachable!(),
        }
    }
}

impl Into<i32> for Direction {
    fn into(self) -> i32 {
        match self {
            Direction::TopRight => 0,
            Direction::Right => 1,
            Direction::BottomRight => 2,
            Direction::BottomLeft => 3,
            Direction::Left => 4,
            Direction::TopLeft => 5,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::TopRight => f.write_str("↗"),
            Direction::Right => f.write_str("→"),
            Direction::BottomRight => f.write_str("↘"),
            Direction::BottomLeft => f.write_str("↙"),
            Direction::Left => f.write_str("←"),
            Direction::TopLeft => f.write_str("↖"),
        }
    }
}
