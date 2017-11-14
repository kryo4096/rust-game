#[macro_use]
extern crate glium;
extern crate noise;

mod math;
mod mesh;
mod generator;

use glium::glutin;

use glium::{Display, Surface, VertexBuffer, IndexBuffer, index, Program, uniforms};

use glutin::{EventsLoop, WindowBuilder, ContextBuilder, Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState};

use math::*;

use mesh::*;

use generator::*;

/// Computes the View Matrix for a first person shooter-type camera
struct FirstPersonCamera {
    pub angle_x: f32,
    pub angle_y: f32,
    pub position: Vec3,
}

impl FirstPersonCamera {
    pub fn new(position: Vec3) -> Self {
        FirstPersonCamera{ angle_x: 0., angle_y: 0. , position}
    }

    pub fn get_matrix(&self) -> Mat4 {
        let rotation_x = Mat4::from_angle_x(Deg(-self.angle_y));
        let rotation_y = Mat4::from_angle_y(Deg(-self.angle_x));

        let translation = Mat4::from_translation(-self.position);

        rotation_x * rotation_y * translation
    }

    pub fn walk (&mut self, amount: Vec3){

        let rotation = Mat4::from_angle_y(Deg(self.angle_x));

        self.position += rotation.transform_vector(amount);

    }
}

fn main() {
    let mut events_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_dimensions(1280, 720)
        .with_title("Advanced Game");

    let context = ContextBuilder::new()
        .with_depth_buffer(24);

    let display = Display::new(window, context, &events_loop).unwrap();

    let mut mesh = generate_plane(256, 256, 1.0);


    let vertex_buffer = mesh.vertex_buffer(&display);


    let indices = mesh.index_buffer(&display);

    let program = Program::from_source(
        &display,
        include_str!("shader/default.vert"),
        include_str!("shader/default.frag"),
        None,
    ).unwrap();

    let mut close = false;

    let mut dx = 0f32;
    let mut dz = 0f32;

    let perspective = Mat4::from(Perspective {
        fovy: Rad(3.14/3.),
        aspect: 16./9.,
        near: 0.1,
        far: 1000.,
    });

    let mut camera = FirstPersonCamera::new(Vec3::new(0.,0.1,0.));

    let params = glium::DrawParameters {
    depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        .. Default::default()
    },
    .. Default::default()
    };


    while !close {

        camera.walk(Vec3::new(dx,0.,dz)/10.);

        let matrix : [[f32;4];4] =  (perspective * camera.get_matrix()).into();

        let uniforms = uniform! (
            vp: matrix
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
                            ElementState::Released => {
                                match scancode {
                                        17 => dz = 0.,
                                        31 => dz = 0.,
                                        30 => dx = 0.,
                                        32 => dx = 0.,
                                        _ => (),
                                    }
                            },
                            ElementState::Pressed =>
                                match scancode {
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
