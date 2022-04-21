use bevy::prelude::*;

mod randomness;
mod gacha_bank;

// use gacha_bank::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Coin(i32);

#[derive(Component)]
struct Pins {
    name: String,
    path: String, // or root/src/Pins/*name*.png
}

#[derive(Component)]
struct Collection(String);

#[derive(Component)]
struct Rarety(i32);

fn drop_coin_system(
    mut query: Query<&mut Coin, With<Player>>
){
    for mut coin in query.iter_mut() {
        coin.0 -= 1;
    }
}

fn setup_scene(
    mut commands: Commands
){
    commands.spawn().insert(Player).insert(Coin(5));
    commands.spawn().insert(Player).insert(Coin(0));
    
}

fn print_coin_status_system(
    mut query: Query<&Coin, With<Player>>
){
    for coin in query.iter(){
        println!("coin value: {}", coin.0);
    }
}

fn main() {

    App::new()
        .add_startup_system(setup_scene.system())
        .add_system(drop_coin_system.system())
        .add_system(print_coin_status_system.system())
        .run();
}
