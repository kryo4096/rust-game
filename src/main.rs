#[macro_use]
extern crate glium;
extern crate noise;

use glium::glutin;
use glium::{Display, Surface, VertexBuffer, IndexBuffer, index, Program, uniforms};
use glutin::{EventsLoop, WindowBuilder, ContextBuilder, Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState};

mod math;
mod mesh;
mod input;
mod generator;
mod camera;
mod timer;

use math::*;
use mesh::*;
use input::*;
use generator::*;
use camera::*;
use timer::*;

fn main() {
    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_dimensions(1600, 900)
        .with_title("Advanced Game");

    let context = ContextBuilder::new()
        .with_depth_buffer(24);

    let display = Display::new(window, context, &events_loop).unwrap();

    let noise = TerrainNoise::new();

    let chunk = Chunk::generate(0, 0, noise);

    let vertex_buffer = chunk.mesh().vertex_buffer(&display);

    let indices = chunk.mesh().index_buffer(&display);

    let program = Program::from_source(
        &display,
        include_str!("shader/default.vert"),
        include_str!("shader/default.frag"),
        None,
    ).unwrap();

    let mut close = false;

    let perspective = Mat4::from(Perspective {
        fovy: Rad(3.14/3.),
        aspect: 16./9.,
        near: 0.1,
        far: 1000.,
    });

    let mut input_manager = InputManager::new();
    input_manager.register_axis("x", 30, 32, 2.);
    input_manager.register_axis("y", 42, 57, 2.);
    input_manager.register_axis("z", 17, 31, 2.);

    let mut camera = FirstPersonCamera::new(Vec3::new(0.,0.1,0.));

    let params = glium::DrawParameters {
    depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        .. Default::default()
    },
    .. Default::default()
    };

    let mut delta : f32 = 0.;

    let mut timer = Timer::start();

    while !close {

        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::Closed => close = true,
                    WindowEvent::MouseMoved{position, ..} => {

                        let window = display.gl_window();

                        let dims = window.get_inner_size().unwrap();

                        camera.angle_x -= (position.0 - dims.0 as f64 / 2.0) as f32 / 20.;
                        camera.angle_y -= (position.1 - dims.1 as f64 / 2.0) as f32 / 20.;

                        camera.angle_y = camera.angle_y.min(90.);
                        camera.angle_y = camera.angle_y.max(-90.);

                        window.set_cursor_position(dims.0 as i32 / 2, dims.1 as i32 / 2);
                    },
                    WindowEvent::KeyboardInput {input, ..} => {

                        let state = input.state;
                        let scancode = input.scancode;

                        match state {
                            ElementState::Released => input_manager.release(scancode),
                            ElementState::Pressed => input_manager.press(scancode),
                        };

                    },
                    _ => (),
                },
                _ => (),
            };
        });

        let mut dx = input_manager.axis("x");
        let mut dy = input_manager.axis("y");
        let mut dz = input_manager.axis("z");

        camera.walk(Vec3::new(dx,dy,dz) * timer.delta() * 100.);

        let model_matrix : [[f32;4];4] = chunk.model_m().into();

        let view_matrix : [[f32;4];4] =  camera.get_matrix().into();

        let projection_matrix : [[f32;4];4] = perspective.into();

        let uniforms = uniform! (
            model_m: model_matrix,
            view_m: view_matrix,
            projection_m: projection_matrix,
            fog_color:  [0.1f32,0.5,1.0,1.0],
            light: [1.0f32,-1.0,1.0],
        );

        let mut target = display.draw();

        target.clear_color_and_depth((0.1,0.5,1.0,1.0), 1.0);

        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &params,
        ).unwrap();

        target.finish().unwrap();

        timer.update();
        input_manager.update(timer.delta());
    }

}
