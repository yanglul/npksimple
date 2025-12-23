
use bevy::prelude::*;
use bevy::camera::*;
use bevy::sprite::*;
use std::fs::File;
mod npk;
use crate::npk::img::Access;
use std::io::BufReader;
use crate::npk::NPK::Npk;


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
    let file = File::open("ImagePacks2/sprite_charater.NPK").unwrap();
    let mut reader = BufReader::new(file);
    let npk = Npk::default();
    npk.read(&mut reader);
    let s = i16::from_le_bytes()

     App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
