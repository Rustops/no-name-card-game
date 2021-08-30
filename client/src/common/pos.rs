#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

/// A discrete position in the world, with x and y being integral numbers.
/// Used among other things for positioning tiles, which are always snapped to the grid.
///
/// Not to be confused with Transform, which contains an entity's actual position.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(deny_unknown_fields)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn append_x(self, x: i32) -> Self {
        Pos::new(self.x + x, self.y)
    }

    pub fn append_y(self, y: i32) -> Self {
        Pos::new(self.x, self.y + y)
    }

    pub fn append_xy(self, x: i32, y: i32) -> Self {
        Pos::new(self.x + x, self.y + y)
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y)
    }
}
