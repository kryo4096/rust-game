#[macro_use]
extern crate glium;
extern crate cgmath as cg;

use glium::glutin;

use glium::{Display, Surface, VertexBuffer, index, Program, uniforms};

use glutin::{EventsLoop, WindowBuilder, ContextBuilder, Event, WindowEvent, DeviceEvent};

use cg::prelude::*;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32;2],
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
        Vertex { position: [-0.5, -0.5]},
        Vertex { position: [-0.5, 0.5 ]},
        Vertex { position: [ 0.5, 0.0 ]},
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

    let mut position = cg::Vector3::new(2.0, 2.0, 2.0f32);

    while !close {

        t += 0.02;

        t %= 3.141*2.0;


        //let tran = cg::Matrix4::from_translation(position);
        let rot_x = cg::Matrix4::from_angle_x(cg::Deg(angle_y));
        let rot_y = cg::Matrix4::from_angle_y(cg::Deg(angle_x));

        let matrix = rot_y * rot_x;

        let matrix : [[f32;4];4] = matrix.inverse_transform().unwrap().into();

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

                        angle_x += (position.0 - dims.0 as f64 / 2.0) as f32;
                        angle_y -= (position.1 - dims.1 as f64 / 2.0) as f32;

                        window.set_cursor_position(dims.0 as i32 / 2, dims.1 as i32 / 2);

                        ()

                    },
                    _ => (),
                },
                _ => (),
            };
        });

    }

}
