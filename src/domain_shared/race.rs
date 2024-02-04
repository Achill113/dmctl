use core::fmt;
use clap::ValueEnum;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, ValueEnum)]
pub enum Race {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    Goblin,
    Halfing,
    HalfElf,
    HalfOrc,
    Human,
    Orc,
    Tiefling,
    Troll
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
