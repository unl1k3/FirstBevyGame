//use bevy::color::palettes::css::LIMEGREEN;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

#[derive(Resource, Debug)]
struct WordList(Vec<String>);

#[derive(Resource, Debug)]
struct WindowSize {
    width: f32,
    height: f32,
}
#[derive(Resource, Clone)]
struct MainFont(TextFont);

#[derive(Resource)]
struct IndexTarget(i32);
impl Default for IndexTarget {
    fn default() -> Self {
        IndexTarget(0)
    }
}

#[derive(Component)]
struct RandomChar {
    target: i32,
    tot: i32,
}
#[derive(Component)]
struct AnimateTranslation;
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<WindowSize>()
        .init_resource::<IndexTarget>()
        .add_systems(
            Startup,
            (load_words_system, setup_window_size, setup_font, setup).chain(),
        )
        .add_systems(Update, (animate_translation, key_pressed))
        .run();
}
// Carica le parole in una risorsa all'avvio
fn load_words_system(mut commands: Commands) {
    let path = "words/it_50k.txt";
    let file = std::fs::read_to_string(format!("assets/{}", path))
        .expect("Non riesco a leggere il file delle parole");

    let words: Vec<String> = file
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    commands.insert_resource(WordList(words));
}
fn spawn_word(
    mut commands: Commands,
    main_font: Res<MainFont>,
    _width: f32,
    height: f32,
    word: &str,
) {
    let spacing = 30.0;
    let tot = word.len() as i32;

    for (i, c) in word.chars().enumerate() {
        spawn_letter(&mut commands, c, i, spacing, height, &main_font.0, tot);
    }
}

fn setup_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(MainFont(TextFont {
        font,
        font_size: 50.0,
        ..default()
    }));
}
fn setup(
    mut commands: Commands,
    main_font: Res<MainFont>,
    window_size: Res<WindowSize>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!(
        "Dimensioni finestra:{}x{}",
        window_size.width, window_size.height
    );
    let texture = asset_server.load("image/gabe-idle-run.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };

    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_scale(Vec3::splat(6.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    // Create a minimal UI explaining how to interact with the example
    commands.spawn((
        Text::new("Test 1\nTest 2"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
    let word = "ciao";
    spawn_word(
        commands,
        main_font,
        window_size.width,
        window_size.height,
        word,
    );
}

fn animate_translation(
    _time: Res<Time>,
    window_size: Res<WindowSize>,
    mut query: Query<&mut Transform, With<RandomChar>>,
) {
    // let mut i = 0;
    for mut transform in &mut query {
        //transform.translation.x += (window_size.width / 2.0) * ops::sin(time.elapsed_secs());
        transform.translation.y -= 1.0; //time.elapsed_secs(); //(window_size.height / 4.0) * ops::cos(time.elapsed_secs());
        if transform.translation.y < (-window_size.height / 2.0) {
            transform.translation.y = window_size.height / 2.0;
        }
    }
}

fn spawn_letter(
    commands: &mut Commands,
    c: char,
    i: usize,
    spacing: f32,
    height: f32,
    text_font: &TextFont,
    tot: i32,
) {
    let mut rng = rand::rng();

    let color = Srgba::new(rng.random(), rng.random(), rng.random(), 1.0);

    let x = -(tot as f32 - 1.0) * spacing / 2.0 + i as f32 * spacing;

    commands.spawn((
        Text2d::new(c.to_string()),
        text_font.clone(),
        TextColor(color.into()),
        Transform::from_xyz(x, height / 2.0, 0.0),
        GlobalTransform::default(),
        AnimateTranslation,
        RandomChar {
            target: i as i32,
            tot,
        },
    ));
}

fn setup_window_size(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut window_size: ResMut<WindowSize>,
) {
    if let Ok(window) = window_query.single() {
        window_size.width = window.resolution.width();
        window_size.height = window.resolution.height();
        println!("Saved window size globally: {:?}", *window_size);
    }
}

fn key_pressed(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(Entity, &mut Text2d, &mut RandomChar, &mut TextColor)>,
    mut index: ResMut<IndexTarget>,
    window_size: ResMut<WindowSize>,
    main_font: Res<MainFont>,
    word_list: Res<WordList>,
) {
    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        match &event.logical_key {
            Key::Character(c) => {
                let pressed_char = c.as_str();
                let target_index = index.0;
                //let green = *TextColor(LIMEGREEN.into());

                // Cerca solo l'entità giusta
                for (ent, text, random_word, mut _textcolor) in &mut query {
                    if text.0 == pressed_char && random_word.target == target_index {
                        println!(
                            "Preso!!:{} {} {}",
                            text.0, random_word.target, random_word.tot
                        );

                        index.0 += 1;
                        commands.entity(ent).despawn();

                        if index.0 == random_word.tot {
                            // text.0 == "Hello";
                            println!("Tutte le lettere corrette! Rimuovo.");
                            index.0 = 0;
                            // let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
                            //let word = "ciauz";

                            let mut rng = rand::rng();

                            let random_word = word_list.0.choose(&mut rng);
                            let word = random_word.unwrap();
                            /*match random_word {
                                Some(word) => println!("Parola casuale: {}", word),
                                None => println!("Il vettore è vuoto!"),
                            }*/

                            let spacing = 30.0;
                            let tot = word.len() as i32;

                            for (i, c) in word.chars().enumerate() {
                                spawn_letter(
                                    &mut commands,
                                    c,
                                    i,
                                    spacing,
                                    window_size.height,
                                    &main_font.0,
                                    tot,
                                );
                            }
                        }

                        break; // trovato! possiamo uscire dal loop
                    }
                }
            }
            Key::Space => {
                println!("Spazio premuto");
            }
            _ => {}
        }
    }
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}
