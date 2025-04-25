use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // ajouter le systeme move_player pour qu'il se lance a chaque frame
        // .add_systems ???
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        // Ajouter le composant Player pour que le systeme move_player iter dessus
        Transform::from_xyz(0., 0., 0.),
        Sprite::from_image(asset_server.load("player.png")),
    ));
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    // cette requete demande seulement le transform des entity qui ont le composant Player
    mut query: Query<&mut Transform, With<Player>>,
) {
    // Faudrait peut etre qu'il puisse se deplacer de gauche a droite ? et de haut en bas ?
    if keys.pressed(KeyCode::ArrowRight) {
        for mut transform in &mut query {
            transform.translation.x += 100. * time.delta_secs();
        }
    }
}

// Bonus:
// Le composant player pourait contenir ces controle
// example:
// struct Player {
//     left: KeyCode,
//     // etc...
// }
// et le systeme move_player verifierai les touche du joueur en question
// comme ca on pourrait faire spawn PLUSIEUR JOUEUR ! (chacun avec ces controle)
