# Workshop Bevy : Créer un plugin d'animation

## Objectif

Comprendre comment structurer un code Bevy avec :
- Un plugin (`Plugin` trait)
- Des composants
- Des systèmes
- Des ressources (si besoin)

---

## Plugins proposés

### 1. Plugin d'animation de spritesheet

**But** : une entité affiche une animation à partir d’une spritesheet.
**Lien** :
  - [exemple animation](https://bevyengine.org/examples/2d-rendering/sprite-sheet/)
  - [simple sprite](https://bevyengine.org/examples/2d-rendering/sprite/)

**Exemple de composants**
```rust

// PS: la proc_macro "dervive" permet d'implementer un trait pour un type
// c'est quoi un trait ? (comme les interface en c++): https://blog.guillaume-gomez.fr/Rust/2/2
// pour les curieux sur les proc_macro: "https://petanode.com/posts/rust-proc-macro/": (perder pas votre temps dessus)
#[derive(Component)]
// les composant requis serve a indiquer les composant
// a ajouter si l'utilisateur ne les a pas explicitement ajouter
#[require(
    Sprite
)]
pub struct SpriteSheetAnimation {
    start: usize,
    end: usize,
    timer: Timer,
    current_frame: usize
}

```

**Ajouter une entiter avec votre composant**
```rust
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::new(37, 50), 3, 7, None, None); // defini comment notre spritesheet et decouper
    let layout_handle = texture_layouts.add(layout); // ajoute le layout dans l'asset storage et retourne l'id pour le retrouver
    commands.spawn((
        Sprite::from_atlas_image(
            asset_server.load("player.png"), // retourne un l'id de limage charger le (l'asset server)
            TextureAtlas {
                layout: layout_handle,
                index: 0,
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
