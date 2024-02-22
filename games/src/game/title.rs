use bevy::prelude::*;
use bevy_ehttp::prelude::*;
use serde::{Deserialize, Serialize};

use super::Page;
use crate::enums::GameState;
use crate::loader::FontAssets;
use crate::resource::TokenResource;

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

    fn start(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
        if !input.just_pressed(KeyCode::Space) {
            return;
        }
        info!("start game ! loading...");
        let req = Request::get("http://127.0.0.1:8000/api/users/login");
        commands.spawn(RequestBundle::<Response<String>>::new(req));
    }

    fn handle_tasks(
        mut commands: Commands,
        mut state: ResMut<NextState<GameState>>,
        mut requests: EventReader<TypedResponseEvent<Response<String>>>,
    ) {
        for res in &mut requests.read() {
            match res.parse() {
                Some(res) => {
                    info!("请求登录成功: {:#?}", res);
                    commands.insert_resource(TokenResource::new(res.data));
                    state.set(GameState::Game);
                }
                None => {
                    info!("请求失败: {:#?}", res.result);
                }
            }
        }
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

    fn client_setup(app: &mut App) {
        app.register_request_type::<Response<String>>();
        app.add_systems(
            Update,
            (Self::start, Self::handle_tasks).run_if(in_state(Self::state())),
        );
    }

    fn build(app: &mut App) {
        app.add_systems(OnEnter(Self::state()), (Self::setup,));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}
