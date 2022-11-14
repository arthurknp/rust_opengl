mod graphics;

extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};
use gl::types::*;

use std::ffi::CString;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(1024, 700, "Game", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);



    // codigo de opengl agr

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
         0.5, -0.5, 0.0,  0.0, 1.0, 0.0,
         0.0, 0.5, 0.0,   0.0, 0.0, 1.0
    ];

    let mut vbo: GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut vbo); }
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); }
    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as GLint,
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (6 * std::mem::size_of::<f32>()) as GLint,
            (3 * std::mem::size_of::<f32>()) as *const GLvoid
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

    }

    let vs = graphics::shader::Shader::from_vert_source(&CString::new(include_str!("triangle.glslv")).unwrap()).unwrap();
    let fs = graphics::shader::Shader::from_frag_source(&CString::new(include_str!("triangle.glslf")).unwrap()).unwrap();

    let shader_program = graphics::shader::Program::from_shaders(&[vs, fs]).unwrap();

    unsafe { gl::Viewport(0, 0, 1024, 700); }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // loop principal
        shader_program.set_used();
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );


        }

        window.swap_buffers();

    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
