#[macro_use]
extern crate glium;

use glium::glutin;

use glium::{Display, Surface, VertexBuffer, index, Program, uniforms};

use glutin::{EventsLoop, WindowBuilder, ContextBuilder, Event, WindowEvent};

#[derive(Clone, Copy)]

struct Vertex {
    position: [f32;2],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_dimensions(1280, 720)
        .with_title("Simple Game");

    let context = ContextBuilder::new()
        .with_depth_buffer(24);

    let display = Display::new(window, context, &events_loop).unwrap();

    let shape = vec! (
        Vertex { position: [-0.5, -0.5]},
        Vertex { position: [-0.5, 0.5]},
        Vertex { position: [0.5, 0.0]}
    );

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();

    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let program = Program::from_source(
        &display,
        include_str!("shader/default.vert"),
        include_str!("shader/default.frag"),
        None
    ).unwrap();

    let mut close = false;

    let mut t :f32 = 0.0;

    while !close {

        t += 0.02;

        t %= 3.141*2.0;

        let uniforms = uniform! (
            matrix: [
                [t.cos() , t.sin(), 0.0, 0.0   ],
                [-t.sin(), t.cos(), 0.0, 0.0   ],
                [0.0     , 0.0    , 1.0, 0.0   ],
                [0.0      , 0.0    , 0.0, 1.0f32],
            ]
        );

        let mut target = display.draw();

        target.clear_color(0.1,0.5,1.0,1.0);

        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();

        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::Closed => close = true,
                    _ => (),
                },
                _ => (),
            };
        });

    }

}
