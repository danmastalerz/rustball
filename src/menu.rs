use bevy::{prelude::*, ui::FocusPolicy};
use bevy::app::AppExit;
use crate::{GameState};
use crate::{PITCH1_SPRITE, PITCH2_SPRITE, PITCH3_SPRITE};

pub struct Menu;

#[derive(Component)]
enum MenuItem {
    Start,
    ChangePitch,
    Quit,
}

#[derive(Component, Clone)]
pub enum Background {
    Pitch1,
    Pitch2,
    Pitch3,
}

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_menu_system).add_system(handle_start_game)
            .add_system_set(SystemSet::on_exit(GameState::InMenu).with_system(despawn_menu));
    }
}

fn despawn_menu(mut commands: Commands, query: Query<Entity>, query_background: Query<&Background>) {
    println!("despawning menu");
    let background = query_background.iter().next().unwrap();
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
    commands.spawn().insert((*background).clone());
}

fn handle_start_game(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    mut query: Query<
        (&Interaction, &MenuItem),
        Changed<Interaction>,
    >,
    mut query_background: Query<(&mut Background, &mut UiImage)>,
    asset_server: Res<AssetServer>,
) {
   // If button clicked, change state
    for (interaction, item) in query.iter() {
        match interaction {
            Interaction::Clicked => {
                match item {
                    MenuItem::Start => {
                        app_state.set(GameState::InGame).expect("Something went wrong!");
                    }
                    MenuItem::ChangePitch => {
                        let (mut pitch_type, mut pitch_image) = query_background.iter_mut().next().unwrap();
                        match *pitch_type {
                            Background::Pitch1 => {
                                *pitch_type = Background::Pitch2;
                                *pitch_image = asset_server.load(PITCH2_SPRITE).into();
                            }
                            Background::Pitch2 => {
                                *pitch_type = Background::Pitch3;
                                *pitch_image = asset_server.load(PITCH3_SPRITE).into();
                            }
                            Background::Pitch3 => {
                                *pitch_type = Background::Pitch1;
                                *pitch_image = asset_server.load(PITCH1_SPRITE).into();
                            }
                        }

                    }
                    MenuItem::Quit => {
                        app_exit_events.send(AppExit);
                    }
                }

            }
            _ => {}
        }
    }
}

fn spawn_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, item: MenuItem) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(20.0), Val::Percent(10.0)),
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                match item {
                    MenuItem::Start => "Start",
                    MenuItem::ChangePitch => "Change Pitch",
                    MenuItem::Quit => "Quit",
                },
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            focus_policy: FocusPolicy::Pass,
            ..Default::default()
        });
    }).insert(item);

}

fn spawn_background(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn_bundle(ImageBundle{
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..Default::default()
        },
        image: asset_server.load(PITCH1_SPRITE).into(),
        ..Default::default()
    }).insert(Background::Pitch1);
}

fn init_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create simple menu, with StartGame and ExitGame buttons

    commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(NodeBundle {
        node: Default::default(),
        style: Style {
           size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
           flex_direction: FlexDirection::ColumnReverse,

           ..Default::default()
       },
        color: Color::NONE.into(),
        image: UiImage(asset_server.load(PITCH1_SPRITE)),
        focus_policy: Default::default(),
        transform: Default::default(),
        global_transform: Default::default(),
        visibility: Visibility {
            is_visible: true,
        }
    }).with_children(|parent| {
        spawn_background(parent, &asset_server);
        spawn_button(parent, &asset_server, MenuItem::Start);
        spawn_button(parent, &asset_server, MenuItem::ChangePitch);
        spawn_button(parent, &asset_server, MenuItem::Quit);
    });




}