
use bevy::prelude::*;
use std::fs::File;
mod npkin;
use crate::npkin::img::Access;
use std::io::BufReader;
use crate::npkin::NPK::Npk;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d::default());
    commands.spawn(Sprite::from_image(asset_server.load("logo.png")));
}
fn main() {
    //加载文件
    let file = File::open("ImagePacks2/sprite_character.NPK").unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut npk = Npk::default();
    npk.read(reader);
 

     App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
