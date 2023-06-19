use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Pos {
    col: u8,
    row: u8,
    layer: u8,
}

pub enum Hand {
    Left, Right
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Finger {
    LP, LR, LM, LI, LT,
    RT, RI, RM, RR, RP,
}

#[derive(Serialize, Deserialize)]
pub enum FingerKind {
    Pinky, Ring, Middle, Index, Thumb
}

impl Finger {
    pub fn hand(self) -> Hand {
	match self {
	    Finger::LP | Finger::LR | Finger::LM | Finger::LI | Finger::LT => Hand::Left,
	    Finger::RP | Finger::RM | Finger::RR | Finger::RI | Finger::RT => Hand::Right,
	}
    }
    pub fn kind(self) -> FingerKind {
	match self {
	    Finger::LT | Finger::RT => FingerKind::Thumb,
	    Finger::LI | Finger::RI => FingerKind::Index,
	    Finger::LM | Finger::RM => FingerKind::Middle,
	    Finger::LR | Finger::RR => FingerKind::Ring,
	    Finger::LP | Finger::RP => FingerKind::Pinky,
	}
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct KeyCoord {
    pos: Pos,
    x: f32,
    y: f32,
    finger: Finger,
}

#[derive(Serialize, Deserialize)]
pub struct Keyboard {
    name: String,
    map: HashMap<Pos, KeyCoord>,
    height: u8,
    width: u8,
    layers: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Layout {
    name: Option<String>,
    authors: Vec<String>,
    layers: Vec<Vec<Vec<char>>>
}
