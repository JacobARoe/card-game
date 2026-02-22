use bevy::prelude::*;

pub fn test_ui(mut commands: Commands) {
    let center = Vec2::new(500.0, 500.0);
    // draw a center dot
    commands.spawn(NodeBundle {
        style: Style { position_type: PositionType::Absolute, left: Val::Px(center.x - 5.0), bottom: Val::Px(center.y - 5.0), width: Val::Px(10.0), height: Val::Px(10.0), ..default() },
        background_color: Color::WHITE.into(),
        ..default()
    });
    // draw a diagonal line using NodeBundle
    let length = 200.0;
    let height = 10.0;
    commands.spawn(NodeBundle {
        style: Style { position_type: PositionType::Absolute, left: Val::Px(center.x - length/2.0), bottom: Val::Px(center.y - height/2.0), width: Val::Px(length), height: Val::Px(height), ..default() },
        background_color: Color::srgb(1.0, 0.0, 0.0).into(),
        transform: Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
        ..default()
    });
}
