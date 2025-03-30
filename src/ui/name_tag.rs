use bevy::prelude::*;

use crate::simulation::body::Body;

#[derive(Component)]
pub struct NameTagId(u32);

pub fn name_tag_setup_system(mut commands: Commands, bodies: Query<&Body>) {
    for body in bodies.iter() {
        commands
            .spawn((
                Node {
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                Visibility::Hidden,
                BackgroundColor(Color::BLACK.with_alpha(0.5)),
                BorderColor(body.metadata.color),
                NameTagId(body.metadata.id.unwrap()),
            ))
            .with_children(|node| {
                node.spawn((
                    Text(
                        body.metadata
                            .name
                            .clone()
                            .unwrap_or("<unknown>".to_string()),
                    ),
                    TextColor::from(Color::WHITE),
                ));
            });
    }
}

const NAME_OFFSET_AU: f32 = 0.005;

pub fn name_tag_update_system(
    window: Query<&Window>,
    mut nodes: Query<(&mut Node, &ComputedNode, &mut Visibility, &NameTagId)>,
    bodies: Query<(&Body, &Transform, &GlobalTransform)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = cameras.single();
    let window = window.single();

    for (body, _transform, global_transform) in bodies.iter() {
        let transform = global_transform.translation();

        let on_viewport = camera.world_to_viewport(
            camera_transform,
            transform.with_y(transform.y + NAME_OFFSET_AU),
        );

        if let Ok(coordinate) = on_viewport {
            for (mut node, computed_node, mut visibility, id) in nodes.iter_mut() {
                let size = computed_node.size();

                if id.0 == body.metadata.id.unwrap() {
                    if coordinate.x < 0.0
                        || coordinate.x > window.width()
                        || coordinate.y < 0.0
                        || coordinate.y > window.height()
                    {
                        *visibility = Visibility::Hidden;
                        continue;
                    } else {
                        *visibility = Visibility::Visible;
                    }

                    node.left = Val::Px(coordinate.x - size.x / 2.0);
                    node.top = Val::Px(coordinate.y - size.y);
                }
            }
        }
    }
}
