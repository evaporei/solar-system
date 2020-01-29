#[macro_use]
extern crate lazy_static;

use gl::types::{GLchar, GLuint};
use gl::types::{GLfloat};
use glfw::{Action, Context, CursorMode, Key, OpenGlProfileHint, WindowHint};
use solar_system::object;
use solar_system::shaders;
use solar_system::texture;
use std::sync::Mutex;

lazy_static! {
    static ref DELTA_TIME: Mutex<f32> = Mutex::new(0.0);
    static ref LAST_FRAME: Mutex<f32> = Mutex::new(0.0);
    static ref CAMERA_POSITION: Mutex<glm::Vector3<f32>> = Mutex::new(glm::Vector3 {
        x: 0.0,
        y: 0.0,
        z: 6.0,
    });
    static ref CAMERA_FRONT: Mutex<glm::Vector3<f32>> = Mutex::new(glm::Vector3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    });
    static ref CAMERA_UP: Mutex<glm::Vector3<f32>> = Mutex::new(glm::Vector3 {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    });
    static ref LAST_X: Mutex<f32> = Mutex::new(512.0);
    static ref LAST_Y: Mutex<f32> = Mutex::new(384.0);
    static ref YAW: Mutex<f32> = Mutex::new(-90.0);
    static ref PITCH: Mutex<f32> = Mutex::new(0.0);
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::Samples(Some(4)));
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    // glfw.window_hint(WindowHint::ContextVersionMajor(3));
    // glfw.window_hint(WindowHint::ContextVersionMinor(3));
    // glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(1920, 1080, "Solar System", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // window.set_key_polling(true);
    window.make_current();
    window.set_framebuffer_size_polling(true);

    // window.set_sticky_keys(true);

    gl::load_with(|s| window.get_proc_address(s)); // kind like glew init

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    let mut vertex_array_id: GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }

    let program_id = shaders::load(
        "./resources/shaders/TransformVertexShader.vertexshader",
        "./resources/shaders/TextureFragmentShader.fragmentshader",
    );

    let mvp = "MVP";
    let my_texture_sampler = "myTextureSampler";

    let matrix_id = unsafe { gl::GetUniformLocation(program_id, mvp.as_ptr() as *const GLchar) };
    let texture_id =
        unsafe { gl::GetUniformLocation(program_id, my_texture_sampler.as_ptr() as *const GLchar) };
    println!("program_id: {}", program_id);
    println!("matrix_id: {}", matrix_id);
    println!("texture_id: {}", texture_id);

    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    // window.set_cursor_mode(CursorMode::Disabled);

    let projection = glm::ext::perspective(glm::radians(45.0), 16.0 / 9.0, 0.1, 100.0);
    let mut view = {
        let camera_position_guard = CAMERA_POSITION.lock().unwrap();
        let camera_up_guard = CAMERA_UP.lock().unwrap();
        let camera_front_guard = CAMERA_FRONT.lock().unwrap();

        glm::ext::look_at(
            *camera_position_guard,
            *camera_position_guard + *camera_front_guard,
            *camera_up_guard,
        )
    };

    // 3D OBJECT: SUN
    let mut sun_model = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let mut sun_mvp = projection * view * sun_model;

    let (sun_vertexes, sun_uvs, _sun_normals) = object::load("./resources/objects/sun.obj");
    // println!("{:?}", sun_vertexes.len());
    // println!("{:?}", sun_uvs.len());
    // println!("{:?}", _sun_normals.len());
    // println!("{:?}", sun_mvp.c1);
    // println!("{:?}", sun_mvp.c2);
    // println!("{:?}", sun_mvp.c3);

    let mut sun_vertex_buffer: GLuint = 0;
    println!("vec3: {}", (sun_vertexes.len() * std::mem::size_of::<glm::Vector3<f32>>()) as isize);
    println!("vec2: {}", (sun_uvs.len() * std::mem::size_of::<glm::Vector2<f32>>()) as isize);

    unsafe {
        gl::GenBuffers(1, &mut sun_vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, sun_vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (sun_vertexes.len() * std::mem::size_of::<glm::Vector3<f32>>()) as isize,
            sun_vertexes.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
    }

    let mut sun_uv_buffer: GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut sun_uv_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, sun_uv_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (sun_uvs.len() * std::mem::size_of::<glm::Vector2<f32>>()) as isize,
            sun_uvs.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
    }

    // // 3D OBJECT: EARTH
    // let mut earth_model: GLuint;
    // let mut earth_mvp: GLuint;
    //
    // let (earth_vertexes, earth_uvs, _earth_normals) =
    //     object::load("./resources/objects/earth_apocalypse.obj");
    //
    // let mut earth_vertex_buffer: GLuint = 0;
    //
    // unsafe {
    //     gl::GenBuffers(1, &mut earth_vertex_buffer);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, earth_vertex_buffer);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (earth_vertexes.len() * std::mem::size_of::<glm::Vector3<f32>>()) as isize,
    //         earth_vertexes.as_ptr() as *const std::ffi::c_void,
    //         gl::STATIC_DRAW,
    //     );
    // }
    //
    // let mut earth_uv_buffer: GLuint = 0;
    //
    // unsafe {
    //     gl::GenBuffers(1, &mut earth_uv_buffer);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, earth_uv_buffer);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (earth_uvs.len() * std::mem::size_of::<glm::Vector2<f32>>()) as isize,
    //         earth_uvs.as_ptr() as *const std::ffi::c_void,
    //         gl::STATIC_DRAW,
    //     );
    // }
    //
    // // 3D OBJECT: MOON
    // let mut moon_model: GLuint;
    // let mut moon_mvp: GLuint;
    //
    // let (moon_vertexes, moon_uvs, _moon_normals) = object::load("./resources/objects/moon.obj");
    //
    // let mut moon_vertex_buffer: GLuint = 0;
    //
    // unsafe {
    //     gl::GenBuffers(1, &mut moon_vertex_buffer);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, moon_vertex_buffer);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (moon_vertexes.len() * std::mem::size_of::<glm::Vector3<f32>>()) as isize,
    //         moon_vertexes.as_ptr() as *const std::ffi::c_void,
    //         gl::STATIC_DRAW,
    //     );
    // }
    //
    // let mut moon_uv_buffer: GLuint = 0;
    //
    // unsafe {
    //     gl::GenBuffers(1, &mut moon_uv_buffer);
    //     gl::BindBuffer(gl::ARRAY_BUFFER, moon_uv_buffer);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (moon_uvs.len() * std::mem::size_of::<glm::Vector2<f32>>()) as isize,
    //         moon_uvs.as_ptr() as *const std::ffi::c_void,
    //         gl::STATIC_DRAW,
    //     );
    // }

    // TEXTURE LOADER
    let mut textures: [GLuint; 3] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

    unsafe {
        gl::GenTextures(3, textures.as_mut_ptr());
    }

    texture::load(textures[0], "./resources/textures/2k_sun.jpg");
    // texture::load(textures[1], "./resources/textures/earth_apocalypse.jpg");
    // texture::load(textures[2], "./resources/textures/2k_moon.jpg");

    let mut counter = 0.0;
    let _rotate_speed = 0.5;

    // let mut earth_rotation = 0.0;
    // let mut moon_rotation = 0.0;

    while window.get_key(Key::Escape) != Action::Press && !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                _ => {}
            }
        }
        counter += 0.01;
        // earth_rotation += 0.3;
        // moon_rotation += 1.0;

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program_id);
        }

        /*
        ====================== FIRST OBJECT ======================
        */
        // sun_model = glm::ext::rotate(&sun_model, 0.001, glm::vec3(0.0, 1.0, 0.0));
        // sun_mvp = projection * view * sun_model;

        // println!("texture_id: {}", texture_id);
        // println!("texture[0]: {}", textures[0]);
        // println!("sun_vertexes.len: {}", sun_vertexes.len());
        unsafe {
            gl::Uniform1i(texture_id, 0);
            gl::UniformMatrix4fv(matrix_id, 1, gl::FALSE, &sun_mvp[0][0]);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, textures[0]);
        }

        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, sun_vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }

        unsafe {
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, sun_uv_buffer);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        }

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, sun_vertexes.len() as i32);
        }

        // /*
        // ====================== SECOND OBJECT ======================
        // */
        // earth_model = glm::ext::translate(
        //     &sun_model,
        //     glm::vec3(
        //         4.0 * glm::sin(counter * 0.1),
        //         0.0,
        //         4.0 * glm::cos(counter * 0.1),
        //     ),
        // );
        //
        // earth_model = glm::ext::scale(&earth_model, glm::vec3(0.8, 0.8, 0.8));
        // earth_model = glm::ext::rotate(
        //     &earth_model,
        //     glm::radians(earth_rotation),
        //     glm::vec3(0.0, 1.0, 0.0),
        // );
        //
        // earth_mvp = projection * view * earth_model;
        //
        // unsafe {
        //     gl::Uniform1i(texture_id, 0);
        //     gl::UniformMatrix4fv(matrix_id, 1, gl::FALSE, &earth_mvp[0][0]);
        //     gl::ActiveTexture(gl::TEXTURE0);
        //     gl::BindTexture(gl::TEXTURE_2D, textures[1]);
        // }
        //
        // unsafe {
        //     gl::EnableVertexAttribArray(0);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, earth_vertex_buffer);
        //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        // }
        //
        // unsafe {
        //     gl::EnableVertexAttribArray(1);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, earth_uv_buffer);
        //     gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        // }
        //
        // unsafe {
        //     gl::DrawArrays(gl::TRIANGLES, 0, earth_vertexes.len() as i32);
        // }
        //
        // /*
        // ====================== THIRD OBJECT ======================
        //  */
        // moon_model = glm::ext::translate(
        //     &earth_model,
        //     glm::vec3(
        //         2.0 * glm::sin(counter * 0.1),
        //         0.0,
        //         2.0 * glm::cos(counter * 0.1),
        //     ),
        // );
        // moon_model = glm::ext::scale(&moon_model, glm::vec3(0.4, 0.4, 0.4));
        // moon_model = glm::ext::rotate(
        //     &moon_model,
        //     glm::radians(moon_rotation),
        //     glm::vec3(0.0, -1.0, 0.0),
        // );
        //
        // moon_mvp = projection * view * moon_model;
        //
        // unsafe {
        //     gl::Uniform1i(texture_id, 0);
        //     gl::UniformMatrix4fv(matrix_id, 1, gl::FALSE, &moon_mvp[0][0]);
        //     gl::ActiveTexture(gl::TEXTURE0);
        //     gl::BindTexture(gl::TEXTURE_2D, textures[2]);
        // }
        //
        // unsafe {
        //     gl::EnableVertexAttribArray(0);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, moon_vertex_buffer);
        //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        // }
        //
        // unsafe {
        //     gl::EnableVertexAttribArray(1);
        //     gl::BindBuffer(gl::ARRAY_BUFFER, moon_uv_buffer);
        //     gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        // }
        //
        // unsafe {
        //     gl::DrawArrays(gl::TRIANGLES, 0, moon_vertexes.len() as i32);
        // }
        //
        // // process_input(&window);
        // view = {
        //     let camera_position_guard = CAMERA_POSITION.lock().unwrap();
        //     let camera_up_guard = CAMERA_UP.lock().unwrap();
        //     let camera_front_guard = CAMERA_FRONT.lock().unwrap();
        //
        //     glm::ext::look_at(
        //         *camera_position_guard,
        //         *camera_position_guard + *camera_front_guard,
        //         *camera_up_guard,
        //     )
        // };
        //
        // let current_frame = glfw.get_time() as f32;
        // {
        //     let mut delta_time_guard = DELTA_TIME.lock().unwrap();
        //     let mut last_frame_guard = LAST_FRAME.lock().unwrap();
        //     *delta_time_guard = current_frame - *last_frame_guard;
        //     *last_frame_guard = current_frame;
        // }
        //
        // unsafe {
        //     gl::DisableVertexAttribArray(0);
        //     gl::DisableVertexAttribArray(1);
        //     gl::DisableVertexAttribArray(2);
        // }

        window.swap_buffers();

        glfw.poll_events();
    }

    unsafe {
        gl::DeleteBuffers(1, &sun_vertex_buffer);
        gl::DeleteBuffers(1, &sun_uv_buffer);

        // gl::DeleteBuffers(1, &earth_vertex_buffer);
        // gl::DeleteBuffers(1, &earth_uv_buffer);
        //
        // gl::DeleteBuffers(1, &moon_vertex_buffer);
        // gl::DeleteBuffers(1, &moon_uv_buffer);

        gl::DeleteProgram(program_id);

        gl::DeleteVertexArrays(1, &vertex_array_id);
    }
}

fn process_input(window: &glfw::Window) {
    let delta_time_guard = DELTA_TIME.lock().unwrap();

    let camera_speed = 1.0 * *delta_time_guard;

    let mut camera_position_guard = CAMERA_POSITION.lock().unwrap();
    let camera_up_guard = CAMERA_UP.lock().unwrap();
    let camera_front_guard = CAMERA_FRONT.lock().unwrap();

    if window.get_key(Key::W) == Action::Press {
        camera_position_guard.x += camera_speed * camera_front_guard.x;
        camera_position_guard.y += camera_speed * camera_front_guard.y;
        camera_position_guard.z += camera_speed * camera_front_guard.z;
    }
    if window.get_key(Key::S) == Action::Press {
        camera_position_guard.x -= camera_speed * camera_front_guard.x;
        camera_position_guard.y -= camera_speed * camera_front_guard.y;
        camera_position_guard.z -= camera_speed * camera_front_guard.z;
    }
    if window.get_key(Key::A) == Action::Press {
        let value =
            glm::normalize(glm::cross(*camera_front_guard, *camera_up_guard)) * camera_speed;
        camera_position_guard.x -= value.x;
        camera_position_guard.y -= value.y;
        camera_position_guard.z -= value.z;
    }
    if window.get_key(Key::D) == Action::Press {
        let value =
            glm::normalize(glm::cross(*camera_front_guard, *camera_up_guard)) * camera_speed;
        camera_position_guard.x += value.x;
        camera_position_guard.y += value.y;
        camera_position_guard.z += value.z;
    }
}

fn handle_window_event(_window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::CursorPos(cursor_x, cursor_y) => {
            let cursor_x = cursor_x as f32;
            let cursor_y = cursor_y as f32;

            let mut last_x_guard = LAST_X.lock().unwrap();
            let mut last_y_guard = LAST_Y.lock().unwrap();
            let mut yaw_guard = YAW.lock().unwrap();
            let mut pitch_guard = PITCH.lock().unwrap();

            let mut x_offset = cursor_x - *last_x_guard;
            let mut y_offset = *last_y_guard - cursor_y;
            *last_x_guard = cursor_x;
            *last_y_guard = cursor_y;

            let sensitivity = 0.05;
            x_offset *= sensitivity;
            y_offset *= sensitivity;

            *yaw_guard -= x_offset;
            *pitch_guard -= y_offset;

            if *pitch_guard > 89.0 {
                *pitch_guard = 89.0;
            }
            if *pitch_guard < -89.0 {
                *pitch_guard = -89.0;
            }

            let front: glm::Vector3<f32> = glm::Vector3 {
                x: (glm::cos(glm::radians(*pitch_guard)) * glm::cos(glm::radians(*yaw_guard))),
                y: glm::sin(glm::radians(*pitch_guard)),
                z: glm::cos(glm::radians(*pitch_guard)) * glm::sin(glm::radians(*yaw_guard)),
            };

            let mut camera_front_guard = CAMERA_FRONT.lock().unwrap();

            *camera_front_guard = glm::normalize(front);
        }
        _ => {}
    }
}
