use bevy::color::palettes::css::{BLACK, DARK_GRAY, GRAY, RED};
use bevy::prelude::*;

use crate::game_plugin::components::{Door, DoorStatus};
use crate::game_plugin::events::*;
use crate::game_plugin::states::GameState;
use crate::states::AppState;

use self::components::{DoorLock, LockPick, LockPicking};
use self::resources::LockPickingData;

pub struct LockPickingPlugin;

impl Plugin for LockPickingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LockPickingData::default())
            .add_systems(OnEnter(AppState::Game), setup_lock_picking)
            .add_systems(Update, rotate_lock)
            .add_observer(on_move_left_event)
            .add_observer(on_move_right_event)
            .add_observer(on_rotate_event)
            .add_observer(on_lock_picking_event);
    }
}

fn setup_lock_picking(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::Game),
        LockPicking,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..default()
        },
        Visibility::Hidden,
        children![
            (Text::new("Press A and D to move lock pick\n And SPACE to rotate lock"),),
            (
                DoorLock,
                Node {
                    width: px(420),
                    height: px(420),
                    border_radius: BorderRadius::all(px(240)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Relative,
                    ..default()
                },
                UiTransform::from_rotation(Rot2::default()),
                BackgroundColor(GRAY.with_alpha(0.75).into()),
                children![
                    (
                        Node {
                            width: px(48),
                            height: px(300),
                            border_radius: BorderRadius::all(px(24)),
                            ..default()
                        },
                        BackgroundColor(BLACK.with_alpha(0.75).into())
                    ),
                    (
                        Node {
                            width: px(300),
                            height: px(64),
                            top: px(72),
                            left: px(200),
                            border_radius: BorderRadius::new(px(12), px(0), px(0), px(12)),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        BackgroundColor(DARK_GRAY.into())
                    ),
                    (
                        LockPick,
                        Node {
                            width: px(32),
                            height: px(370),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::End,
                            top: px(150),
                            ..default()
                        },
                        UiTransform::from_rotation(Rot2::default()),
                        children![(
                            Node {
                                width: percent(100),
                                height: px(200),
                                border_radius: BorderRadius::new(px(16), px(16), px(0), px(0)),
                                ..default()
                            },
                            BackgroundColor(RED.into()),
                        )]
                    )
                ]
            ),
        ],
    ));
}

fn rotate_lock(
    mut commands: Commands,
    time: Res<Time>,
    mut data: ResMut<LockPickingData>,
    mut game_state: ResMut<NextState<GameState>>,
    mut lock_query: Query<&mut UiTransform, With<DoorLock>>,
    mut visibility_query: Query<&mut Visibility, With<LockPicking>>,
    mut door_query: Query<&mut Door>,
) {
    if !data.solved {
        return;
    }

    let Ok(mut ui_transform) = lock_query.single_mut() else {
        return;
    };

    ui_transform
        .rotation
        .smooth_nudge(&Rot2::degrees(180.0), 10.0, 2.0 * time.delta_secs());

    if ui_transform.rotation.as_degrees() == -180.0 {
        let Ok(mut visibility) = visibility_query.single_mut() else {
            return;
        };

        let Ok(mut door) = door_query.get_mut(data.entity.unwrap()) else {
            return;
        };

        door.status = DoorStatus::Closed;

        *visibility = Visibility::Hidden;

        game_state.set(GameState::Playing);

        commands.trigger(DoorOpenEvent {
            entity: data.entity.unwrap(),
        });

        data.entity = None;
        data.solved = false;
    }
}

fn on_lock_picking_event(
    event: On<LockPickingEvent>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut visibility_query: Query<&mut Visibility, With<LockPicking>>,
) {
    let Ok(mut visibility) = visibility_query.single_mut() else {
        return;
    };

    commands.insert_resource(LockPickingData {
        entity: Some(event.entity),
        solved: false,
    });

    game_state.set(GameState::InLockPicking);

    *visibility = Visibility::default();
}

fn on_rotate_event(
    _: On<LockPickingRotateEvent>,
    mut data: ResMut<LockPickingData>,
    lock_pick_query: Query<&UiTransform, With<LockPick>>,
) {
    let Ok(lock_pick_ui_transform) = lock_pick_query.single() else {
        return;
    };

    if (90.0..=180.0).contains(&lock_pick_ui_transform.rotation.as_degrees()) {
        data.solved = true;
    }
}

fn on_move_left_event(_: On<LockPickingMoveLeftEvent>, mut lock_pick_query: Query<&mut UiTransform, With<LockPick>>) {
    let Ok(mut ui_transform) = lock_pick_query.single_mut() else {
        return;
    };

    ui_transform.rotation = Rot2::degrees(ui_transform.rotation.as_degrees() + 1.0);
}

fn on_move_right_event(_: On<LockPickingMoveRightEvent>, mut lock_pick_query: Query<&mut UiTransform, With<LockPick>>) {
    let Ok(mut ui_transform) = lock_pick_query.single_mut() else {
        return;
    };

    ui_transform.rotation = Rot2::degrees(ui_transform.rotation.as_degrees() - 1.0);
}

mod components {
    use bevy::ecs::component::Component;

    #[derive(Component)]
    pub struct DoorLock;

    #[derive(Component)]
    pub struct LockPick;

    #[derive(Component)]
    pub struct LockPicking;
}

mod resources {
    use bevy::prelude::*;

    #[derive(Default, Resource)]
    pub struct LockPickingData {
        pub entity: Option<Entity>,
        pub solved: bool,
    }
}
