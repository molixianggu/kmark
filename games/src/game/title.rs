use bevy::prelude::*;

use super::Page;
use crate::enums::GameState;
use crate::loader::FontAssets;

#[derive(Component)]
pub struct TitlePage;

impl TitlePage {
    fn setup(mut commands: Commands, font: Res<FontAssets>) {
        commands
            .spawn((NodeBundle { ..default() }, Self))
            .with_children(|parent| {
                // 添加标题
                parent.spawn(TextBundle::from_section(
                    "KM Game",
                    TextStyle {
                        font: font.fira_sans.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                ));
            });
    }

    fn start(mut state: ResMut<NextState<GameState>>, input: Res<ButtonInput<KeyCode>>) {
        if !input.just_pressed(KeyCode::Space) {
            return;
        }
        info!("start game !");
        state.set(GameState::Game);
        
    }
}

impl Page for TitlePage {
    type SelfType = Self;

    fn name() -> &'static str {
        "title"
    }
    fn state() -> GameState {
        GameState::Title
    }

    fn build(app: &mut App) {
        app.add_systems(OnEnter(Self::state()), (Self::setup,));

        app.add_systems(Update, (Self::start).run_if(in_state(Self::state())));
    }
}
