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
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
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
    #version 140

    in vec3 position;
    in vec3 normal;
    
    uniform mat4 matrix;
    
    void main() {
        gl_Position = matrix * vec4(position, 1.0);
    }
"#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;
    
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
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
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! { matrix: matrix },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
