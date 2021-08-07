use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy::render::camera::Camera;

// This is a simple example of a camera that flies around.
// There's an included example of a system that toggles the "enabled"
// property of the fly camera with "T"

fn init(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// Light
	commands.spawn().insert_bundle(PointLightBundle {
		transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
		..Default::default()
	});

	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..Default::default()
	});

	let camera = FlyCamera {
		pitch: 45.0,
		.. Default::default()
	};

	commands
		.spawn()
		.insert_bundle(
			PerspectiveCameraBundle {
				camera: Camera {
					name: Some("Camera3d".to_string()),
					..Default::default()
				},
				perspective_projection: Default::default(),
				visible_entities: Default::default(),
				transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
				global_transform: Default::default(),
			}
			// PerspectiveCameraBundle::with_name("sdf")
		)
		.insert(camera);

	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
		material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
		..Default::default()
	});

	let box_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.25 }));
	let box_material = materials.add(Color::rgb(1.0, 0.2, 0.3).into());

	const AMOUNT: i32 = 6;
	for x in -(AMOUNT / 2)..(AMOUNT / 2) {
		for y in -(AMOUNT / 2)..(AMOUNT / 2) {
			for z in -(AMOUNT / 2)..(AMOUNT / 2) {
				commands.spawn().insert_bundle(PbrBundle {
					mesh: box_mesh.clone(),
					material: box_material.clone(),
					transform: Transform::from_translation(Vec3::new(
						x as f32, y as f32, z as f32,
					)),
					..Default::default()
				});
			}
		}
	}

	println!("Started example!");
}

// Press "T" to toggle keyboard+mouse control over the camera
fn toggle_button_system(
	input: Res<Input<KeyCode>>,
	mut query: Query<&mut FlyCamera>,
) {
	for mut options in query.iter_mut() {
		if input.just_pressed(KeyCode::T) {
			println!("Toggled FlyCamera enabled!");
			options.enabled = !options.enabled;
		}
	}
}

fn toggle_camera_rotation_system(
	buttons: Res<Input<MouseButton>>,
	mut query: Query<&mut FlyCamera>,
) {
	for mut options in query.iter_mut() {
		if buttons.just_pressed(MouseButton::Middle) {
			println!("Enabled rotation");
		}
		if buttons.pressed(MouseButton::Middle) {
			options.mouse_enabled = true;
		}
		if buttons.just_released(MouseButton::Middle) {
			println!("Disabled rotation");
			options.mouse_enabled = false;
		}
	}
}

fn main() {
	App::new()
		.insert_resource(Msaa { samples: 4 })
		.add_plugins(DefaultPlugins)
		.add_startup_system(init.system())
		.add_plugin(FlyCameraPlugin)
		.add_system(toggle_button_system.system())
		.add_system(toggle_camera_rotation_system.system())
		.run();
}
