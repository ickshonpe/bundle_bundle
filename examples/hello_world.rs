use bevy::prelude::*;
use bundle_bundle::*;

#[derive(Component)]
struct HelloWorldMessageMarker;

#[derive(Component)]
struct AnotherMessageMarker;

#[derive(Component)]
struct UiElementMarker;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLUE),
                ..Default::default()
            }
            .bundle(UiElementMarker),
        )
        .with_children(|b| {
            b.spawn(
                TextBundle::from_section(
                    "Hello, world!",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER)
                .bundle(BackgroundColor(Color::RED))
                .bundle(HelloWorldMessageMarker)
                .bundle(UiElementMarker),
            );

            b.spawn(
                TextBundle::from_section(
                    "Some Other Message",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..Default::default()
                })
                .bundle(BackgroundColor(Color::GREEN))
                .bundle(AnotherMessageMarker)
                .bundle(UiElementMarker),
            );
        });
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
