use bevy::prelude::Vec2;

pub trait EnumDisplay {
    fn to_display_string(&self) -> String;
}

pub trait ArenaTraits {
    fn to_display_string(&self) -> String;
    fn offset_matrix(&self) -> Vec2;
    fn grid_index(&self) -> u8;
    
    fn debug_tile(&self) -> &str;
}
