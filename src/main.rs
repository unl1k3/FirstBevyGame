use bevy::color::palettes::css::LIMEGREEN;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<WindowSize>()
        .init_resource::<IndexTarget>()
        .add_systems(Startup, (setup_window_size, setup_font, setup).chain())
        .add_systems(Update, (animate_translation, key_pressed))
        .run();
}
fn spawn_word(
    mut commands: Commands,
    main_font: Res<MainFont>,
    width: f32,
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
fn setup(mut commands: Commands, main_font: Res<MainFont>, window_size: Res<WindowSize>) {
    /*let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    */
    // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    println!(
        "Dimensioni finestra:{}x{}",
        window_size.width, window_size.height
    );
    commands.spawn(Camera2d);

    // Demonstrate changing translation
    //   let mut rng = rand::rng();

    let word = "ciao";
    spawn_word(
        commands,
        main_font,
        window_size.width,
        window_size.height,
        word,
    );

    /*let spacing = 30.0; // distanza tra lettere
    let start_x = -(word.len() as f32 - 1.0) * spacing / 2.0; // centrare orizzontalmente

    for (i, c) in word.chars().enumerate() {
        let random_number1: f32 = rng.random();
        let random_number2: f32 = rng.random();
        let random_number3: f32 = rng.random();
        let x = start_x + i as f32 * spacing;

        commands.spawn((
            Text2d::new(c.to_string()),
            text_font.clone(),
            TextColor(Srgba::new(random_number1, random_number2, random_number3, 1.0).into()),
            Transform::from_xyz(x, window_size.height / 2.0, 0.0), // posizione orizzontale
            GlobalTransform::default(),
            AnimateTranslation,
            RandomChar { target: i as i32 },
        ));

        //if i == 0 {
        //  commands.entity(ent).insert(RightChar);
        //}
        //  println!("val i:{}", i);
    }
    */
    /*
    commands.spawn((
        Text2d::new("translation"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        TextColor(LIGHT_BLUE.into()),
        AnimateTranslation,
        MyText,
    ));*/
    //  .with_child((TextSpan("::bis".to_string()), TextColor(LIGHT_GREEN.into())));
}

fn animate_translation(
    _time: Res<Time>,
    window_size: Res<WindowSize>,
    mut query: Query<&mut Transform, With<RandomChar>>,
) {
    // let mut i = 0;
    for mut transform in &mut query {
        //transform.translation.x += (window_size.width / 2.0) * ops::sin(time.elapsed_secs());
        transform.translation.y -= 2.0; //time.elapsed_secs(); //(window_size.height / 4.0) * ops::cos(time.elapsed_secs());
        if transform.translation.y < (-window_size.height / 2.0) {
            transform.translation.y = window_size.height / 2.0;
        }
        /*
        i = i + 1;
        println!(
            "lettera:{} x={},y={} Tempo:{}",
            i,
            transform.translation.x,
            transform.translation.y,
            time.elapsed_secs()
        );
        */
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
    asset_server: Res<AssetServer>,
    window_size: ResMut<WindowSize>,
    main_font: Res<MainFont>,
) {
    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        match &event.logical_key {
            Key::Character(c) => {
                let pressed_char = c.as_str();
                let target_index = index.0;
                let green = *TextColor(LIMEGREEN.into());

                // Cerca solo l'entità giusta
                for (ent, text, random_word, mut textcolor) in &mut query {
                    if text.0 == pressed_char && random_word.target == target_index {
                        println!(
                            "Preso!!:{} {} {}",
                            text.0, random_word.target, random_word.tot
                        );
                        textcolor.0 = green;
                        index.0 += 1;
                        commands.entity(ent).despawn();
                        /*if index.0 == 4 {
                        for e in query.iter() {
                          commands.entity(text).despawn();
                        }
                        */

                        if index.0 == random_word.tot {
                            // text.0 == "Hello";
                            println!("Tutte le lettere corrette! Rimuovo.");
                            index.0 = 0;
                            // let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
                            let word = "ciauz";

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

/*fn keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Text2d, With<RandomWord>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        println!("Prmuto r");
        for mut text in &mut query {
            println!("testo:{}", text.0);
        }
    }
}
*/
impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }
}
