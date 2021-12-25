//fn main() {
//    println!("Hello, world!");
//}

extern crate glium;
use glium::uniform;
use glium::Surface;

#[path = "teapot.rs"]
mod teapot;

fn main() {
    use glium::glutin;
    // 1. The **winit::EventsLoop** for handling events.
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let vertex_shader_src = r#"
    #version 150
    in vec3 position;
    in vec3 normal;
    out vec3 v_normal;
    uniform mat4 matrix;
    void main() {
        v_normal = transpose(inverse(mat3(matrix))) * normal;
        gl_Position = matrix * vec4(position, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 150
    in vec3 v_normal;
    out vec4 color;
    uniform vec3 u_light;
    void main() {
        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec3 dark_color = vec3(0.6, 0.0, 0.0);
        vec3 regular_color = vec3(1.0, 0.0, 0.0);
        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
"#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    events_loop.run(move |ev, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        let light = [-1.0, 0.4, 0.9f32];

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! { matrix: matrix, u_light: light },
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    });
}
