use bevy::{app::AppExit, prelude::*};

use crate::AppState;

pub struct MainMenuPlugin;

#[derive(Component, Debug)]
struct MainMenuData {
    ui_root: Entity,
}

#[derive(Component, Debug)]
enum MenuButton {
    Play,
    Quit,
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(AppState::Build)
                    .expect("Couldn't switch state to Build"),
                MenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::MainMenu).with_system(button_press_system),
        )
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup))
        .add_system(main_menu_controls);
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_root = commands
        .spawn_bundle(root())
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn_bundle(border()).with_children(|parent| {
                // left vertical fill (content)
                parent
                    .spawn_bundle(menu_background())
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(button())
                            .with_children(|parent| {
                                parent.spawn_bundle(button_text(&asset_server, "New Game"));
                            })
                            .insert(MenuButton::Play);
                        parent
                            .spawn_bundle(button())
                            .with_children(|parent| {
                                parent.spawn_bundle(button_text(&asset_server, "Quit"));
                            })
                            .insert(MenuButton::Quit);
                    });
            });
        })
        .id();

    commands.insert_resource(MainMenuData { ui_root });
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
}

fn main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::MainMenu {
        if keys.just_pressed(KeyCode::Return) {
            app_state.set(AppState::Build).unwrap();
            keys.reset(KeyCode::Return);
        }
    } else {
        if keys.just_pressed(KeyCode::Escape) {
            //set some state to add continue button
            app_state.set(AppState::MainMenu).unwrap();
            // still needed?
            keys.reset(KeyCode::Escape);
        }
    }
}
