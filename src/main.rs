use bevy::{prelude::*, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, window::{PresentMode, PrimaryWindow}, sprite::MaterialMesh2dBundle};
use particles::consts::G;

#[derive(Component)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    mass: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // for _ in 0..1000 {
    //     commands
    //     .spawn(MaterialMesh2dBundle {
    //         mesh: meshes.add(shape::Circle::new(10.).into()).into(),
    //         material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //         transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    //         ..default()
    //     })
    //     .insert(Particle {
    //         pos: Vec3::new(0.0, 0.0, 0.0),
    //         vel: Vec3::new(50.0, 0.0, 0.0),
    //         mass: 1.0,
    //     });
    // }
}

fn update_particles(time: Res<Time>, mut query: Query<&mut Particle>) {
    let mut particles = query.iter_mut().collect::<Vec<_>>();
    let mut forces = vec![Vec3::ZERO; particles.len()];

    for i in 0..particles.len() {
        for j in 0..particles.len() {
            if i != j {
                let particle_i = &particles[i];
                let particle_j = &particles[j];
                let r = particle_i.pos - particle_j.pos;
                let r_magnitude = r.length();

                if r_magnitude > 0.0 {
                    let f = G * particle_i.mass * particle_j.mass / r_magnitude.powi(2);
                    forces[i] += f;
                    println!("Particle {} force: {:?}", i, forces[i]);
                }
            }
        }
    }

    // Then, update each particle's velocity and position based on its force
    for i in 0..particles.len() {
        let particle = &mut particles[i];
        let mass = particle.mass;
        particle.vel += forces[i] * time.delta_seconds() / mass;
        let vel = particle.vel;
        particle.pos += vel * time.delta_seconds();
    }
}

fn render(mut query: Query<(&Particle, &mut Transform)>) {
    for (particle, mut transform) in query.iter_mut() {
        transform.translation = particle.pos;
    }
}

fn spawn_particles_on_click(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            let worldspace = window_to_world(position, q_windows.single(), &Transform::from_translation(Vec3::new(0., 0., 0.)));

            commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(worldspace),
            ..default()
        })
        .insert(Particle {
            pos: worldspace,
            vel: Vec3::new(50.0, 0.0, 0.0),
            mass: 100.0,
        });
        }
    }
}

fn window_to_world(
    position: Vec2,
    window: &Window,
    camera: &Transform,
) -> Vec3 {

    // Invert y position to match Bevy's coordinate system
    let position = Vec2::new(position.x, window.height() - position.y);

    // Center in screen space
    let norm = Vec3::new(
        position.x - window.width() / 2.,
        position.y - window.height() / 2.,
        0.,
    );

    // Apply camera transform
    *camera * norm
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "N-Body Simulation".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_particles, bevy::window::close_on_esc, render, spawn_particles_on_click))
        .run();
}
