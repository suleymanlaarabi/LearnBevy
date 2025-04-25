# Workshop Bevy : Créer un plugin d'animation

**Prérequis**
- [Setup Bevy (Clicker sur moi)](https://bevyengine.org/learn/quick-start/getting-started/setup/)
- [Lire et essayer bevy avec le QuickStart (Clicker sur moi)](https://bevyengine.org/learn/quick-start/getting-started/apps/)

## Objectif

Comprendre comment structurer un code Bevy avec :
- Un plugin (`Plugin` trait)
- Des composants
- Des systèmes
- Des ressources (si besoin)

---

## Etapes

- Completer le main.rs dans src (les commentaires details les attendu)
- Ajouter le plugin propsé ci dessous (POUR APRES)

## Plugins proposés (Dabord completer le main.rs avant de lire ca)

### 1. Plugin d'animation de spritesheet

**But** : une entité affiche une animation à partir d’une spritesheet.
**Lien** :
  - [exemple animation (Clicker sur moi)](https://bevyengine.org/examples/2d-rendering/sprite-sheet/)
  - [simple sprite (Clicker sur moi)](https://bevyengine.org/examples/2d-rendering/sprite/)

**Exemple de composants**
```rust
// PS : la proc_macro "derive" permet d’implémenter automatiquement un trait pour un type.
// un trait ? (comme les interfaces en Java ou C#) : https://blog.guillaume-gomez.fr/Rust/2/2
// Pour les curieux sur les proc_macros : https://petanode.com/posts/rust-proc-macro/ (ne perdez pas votre temps dessus)
#[derive(Component)]
// Les composants requis servent à indiquer les composants
// à ajouter automatiquement si l'utilisateur ne les a pas explicitement ajoutés.
#[require(
    Sprite
)]
pub struct SpriteSheetAnimation {
    start: usize,
    end: usize,
    timer: Timer,
    current_frame: usize,
}
```

**Ajouter une entiter avec votre composant**
```rust
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // la camera est utiliser par un system qui pour savoir comment rendre le monde dans la window
    commands.spawn(Camera2d);
    // defini comment notre spritesheet et decouper
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 2, None, None);
    // ajoute le layout dans l'asset storage et retourne l'id pour le retrouver
    let layout_handle = texture_layouts.add(layout);
    commands.spawn((
        // Transform represente la position, scale, rotation etc... de l'entiter
        Transform::from_scale(Vec3::splat(3.0)),
        Sprite::from_atlas_image(
            // retourne un l'id de limage charger le (l'asset serve)
            asset_server.load("player.png"),
            TextureAtlas {
                layout: layout_handle,
                index: 1,
            },
        ),
        // ici ajouter votre composant pour animer la sprite sheet
    ));
}
```

**Ajouter un system pour mettre a jour le sprite animer**
```rust
pub fn update_sprite_animation(
    time: Res<Time>, // ressource qui contient le temps ecouler depuis la derniere frame
    mut query: Query<(&mut SpriteSheetAnimation, &mut Sprite)>, // modifier la pour quelle contienne le Sprite de lentiter
) {
    let elapsed_time = time.delta();
    for (mut sprite_sheet_animation, mut sprite) in &mut query {
        sprite_sheet_animation.timer.tick(elapsed_time); // faire avancer le Timer
        if sprite_sheet_animation.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                // ici avec metter a jour limage de l'atlas du sprite
            }
        }
    }
}
```

**Enregistrer vos systemes**
```rust
App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_player)
    // Ajouter ici un planning pour que a chaque frame votre systems soit executer
    .add_systems(un_scheduler_qui_sexecute_a_chaque_update, update_sprite_animation)
    .run();
```
