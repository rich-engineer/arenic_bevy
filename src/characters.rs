use bevy::prelude::Component;

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct GuildMaster;

#[derive(Component)]
pub struct SelectedShadow;

#[derive(Component)]
pub struct Hunter;

#[derive(Component)]
#[allow(dead_code)]
pub struct Character {
    pub name: String,
}

#[derive(Component)]
pub struct ActiveHero;

#[derive(Component)]
pub struct ArenaBossText;
