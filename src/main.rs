#[macro_use]
extern crate glium;
extern crate cgmath as cg;

use glium::glutin;

use glium::{Display, Surface, VertexBuffer, index, Program, uniforms};

use glutin::{EventsLoop, WindowBuilder, ContextBuilder, Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState};

use cg::prelude::*;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32;3],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_dimensions(1280, 720)
        .with_title("Advanced Game");

    let context = ContextBuilder::new()
        .with_depth_buffer(24);

    let display = Display::new(window, context, &events_loop).unwrap();

    let shape = vec! (
        Vertex { position: [-1.0, -1.0, 0.]},
        Vertex { position: [1.0, -1.0, 0.]},
        Vertex { position: [ 0.0, 1.0, 0.]},
        Vertex { position: [3.0, -1.0, 0.]},
        Vertex { position: [5.0, -1.0, 0.]},
        Vertex { position: [ 4.0, 1.0, 0.]},
        Vertex { position: [-8.0, -1.0, 0.]},
        Vertex { position: [-6.0, -1.0, 0.]},
        Vertex { position: [ -7.0, 1.0, 0.]},
    );

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();

    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let program = Program::from_source(
        &display,
        include_str!("shader/default.vert"),
        include_str!("shader/default.frag"),
        None, 
    ).unwrap();

    let mut close = false;

    let mut t = 0.0f32;

    let mut angle_x = 0f32;
    let mut angle_y = 0f32;

    let mut dx = 0f32;
    let mut dz = 0f32;

    let persp = cg::PerspectiveFov{
        fovy: cg::Rad(3.14/3f32),
        aspect: 16.0/9.0f32,
        near: 0.1f32,
        far: 100.0,
    };

    let persp = cg::Matrix4::from(persp);

    let mut position = cg::Vector3::new(0., 0.5, 1.);

    while !close {

        let rot_x = cg::Matrix4::from_angle_x(cg::Deg(-angle_y));
        let rot_y = cg::Matrix4::from_angle_y(cg::Deg(-angle_x));

        let rot_matrix = rot_x * rot_y;

        position += cg::Matrix4::from_angle_y(cg::Deg(angle_x)).transform_vector(cg::Vector3::new(dx,0.0,dz) / 100.);

        let trans = cg::Matrix4::from_translation(-position);

        let matrix = persp * rot_matrix * trans;

        let matrix : [[f32;4];4] = matrix.into();

        let uniforms = uniform! (
            matrix: matrix
        );




        let mut target = display.draw();

        target.clear_color_and_depth((0.1,0.5,1.0,1.0), 1.0);

        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default(),
        ).unwrap();

        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::Closed => close = true,
                    WindowEvent::MouseMoved{position, ..} => {

                        let window = display.gl_window();

                        let dims = window.get_inner_size().unwrap();

                        angle_x -= (position.0 - dims.0 as f64 / 2.0) as f32 / 20.;
                        angle_y -= (position.1 - dims.1 as f64 / 2.0) as f32 / 20.;

                        angle_y = angle_y.min(90.);
                        angle_y = angle_y.max(-90.);

                        window.set_cursor_position(dims.0 as i32 / 2, dims.1 as i32 / 2);
                    },
                    WindowEvent::KeyboardInput {input, ..} => {

                        let state = input.state;
                        let scancode = input.scancode;

                        match state {
                            ElementState::Released => {
                                match scancode {
                                        17 => dz = 0.,
                                        31 => dz = 0.,
                                        30 => dx = 0.,
                                        32 => dx = 0.,
                                        _ => (),
                                    }
                            },
                            ElementState::Pressed =>     match scancode {
                                    17 => dz = -1.,
                                    31 => dz = 1.,
                                    30 => dx = -1.,
                                    32 => dx = 1.,
                                    _ => (),
                                }
                        };



                        println!("{}", scancode);

                    },
                    _ => (),
                },
                _ => (),
            };
        });

    }

}
