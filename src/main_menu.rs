//TODO: rename to menu.rs
//TODO audio setting
use bevy::{app::AppExit, prelude::*};

use crate::AppState;

pub struct MainMenuPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct MenuData(Option<Entity>);
#[derive(Component, Debug)]
struct Background;
#[derive(Component, Debug)]
struct Title;
#[derive(Component, Debug)]
enum MenuButton {
    Play,
    Quit,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_press_system)
            .init_resource::<MenuData>()
            .add_state(MenuData::default())
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu).with_system(button_press_system),
            )
            .add_system_set(SystemSet::on_enter(AppState::Intro).with_system(spawn_intro))
            .add_system_set(
                SystemSet::on_update(AppState::Intro).with_system(poll_intro_timer_over),
            ) // or spawn main menu directly?
            .add_system_set(SystemSet::on_exit(AppState::Intro).with_system(spawn_main_menu))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(despawn_main_menu))
            .add_system(main_menu_controls)
            .add_startup_system(audio_system);
    }
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            //why is this called twice?
            // thats why we can't expect here
            dbg!(button);
            match button {
                MenuButton::Play => {
                    state.set(AppState::Build);
                }
                MenuButton::Quit => {
                    exit.send(AppExit);
                }
            }
        }
    }
}

fn root() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: UiColor(Color::hex("0C1E21").unwrap()),
        ..Default::default()
    }
}

fn border() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(800.0), Val::Auto),
            border: UiRect::all(Val::Px(0.0)),
            ..Default::default()
        },
        color: UiColor(Color::hex("27373B").unwrap()),
        ..Default::default()
    }
}

fn menu_background() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        color: UiColor(Color::hex("0C1E21").unwrap()),
        ..Default::default()
    }
}

fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(20.0)),
            ..Default::default()
        },
        color: UiColor(Color::hex("45A7BA").unwrap()),
        ..Default::default()
    }
}

fn intro_text(asset_server: &Res<AssetServer>) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::from_section(
            "Splash Mobs",
            TextStyle {
                font: asset_server.load("fonts/Oswald-SemiBold.ttf"),
                font_size: 70.0,
                color: Color::hex("0C1E21").unwrap(),
            },
        ),
        ..Default::default()
    };
}

fn button_text(asset_server: &Res<AssetServer>, label: &str) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::from_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/Oswald-SemiBold.ttf"),
                font_size: 70.0,
                color: Color::hex("0C1E21").unwrap(),
            },
        ),
        ..Default::default()
    };
}

fn spawn_intro(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut menu_data: ResMut<MenuData>,
) {
    let ui_root = commands
        .spawn_bundle(root())
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn_bundle(border()).with_children(|parent| {
                // left vertical fill (content)
                parent
                    .spawn_bundle(menu_background())
                    .insert(Background)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(button())
                            .with_children(|parent| {
                                parent.spawn_bundle(intro_text(&asset_server));
                            })
                            .insert(Title);
                    });
            });
        })
        .id();

    *menu_data = MenuData(Some(ui_root));
}

fn poll_intro_timer_over(mut state: ResMut<State<AppState>>, time: Res<Time>) {
    if time.seconds_since_startup() > 3.0 {
        state
            .set(AppState::MainMenu)
            .expect("Couldn't switch state to MainMenu");
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    menu_background_query: Query<Entity, With<Background>>,
    asset_server: Res<AssetServer>,
) {
    let entity = menu_background_query.single();
    commands.entity(entity).despawn_descendants();
    commands.entity(entity).with_children(|parent| {
        parent
            .spawn_bundle(button())
            .with_children(|parent| {
                parent.spawn_bundle(button_text(&asset_server, "Play"));
            })
            .insert(MenuButton::Play);
        parent
            .spawn_bundle(button())
            .with_children(|parent| {
                parent.spawn_bundle(button_text(&asset_server, "Quit"));
            })
            .insert(MenuButton::Quit);
    });
}

fn despawn_main_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.0.unwrap()).despawn_recursive();
}

fn main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::MainMenu {
        if keys.just_pressed(KeyCode::Return) {
            app_state.set(AppState::Build).unwrap();
            keys.reset(KeyCode::Return);
        }
    } else {
        // todo remember build timer so it is not resetted when going to menu
        if keys.just_pressed(KeyCode::Escape) {
            // lololol
            app_state.set(AppState::Intro).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}

fn audio_system(
    audio: Res<Audio>,
    app_state: Res<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    match *app_state.current() {
        AppState::Intro => {
            audio.play_with_settings(
                asset_server.load("music/menu-start.mp3"),
                PlaybackSettings {
                    repeat: false,
                    volume: 0.75,
                    speed: 1.0,
                },
            );

            audio.play_with_settings(
                asset_server.load("music/menu-loop.ogg"),
                PlaybackSettings {
                    repeat: true,
                    volume: 0.75,
                    speed: 1.0,
                },
            );
        }

        AppState::MainMenu => {
            audio.play_with_settings(
                asset_server.load("music/menu-loop.ogg"),
                PlaybackSettings {
                    repeat: true,
                    volume: 0.75,
                    speed: 1.0,
                },
            );
        }
        // AppState::Build => {
        //     audio.play_with_settings(
        //        ...
        //         PlaybackSettings {
        //             repeat: true,
        //             volume: 0.75,
        //             speed: 1.0,
        //         },
        //     );
        // }
        // AppState::Attack => {
        //     audio.play_with_settings(
        //         ...
        //         PlaybackSettings {
        //             repeat: true,
        //             volume: 0.75,
        //             speed: 1.0,
        //         },
        //     );
        // }
        _ => {}
    }
}
