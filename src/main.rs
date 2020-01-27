use gl::types::{GLchar, GLuint};
use glfw::{Action, Context, CursorMode, Key, OpenGlProfileHint, WindowHint};
use solar_system::object;
use solar_system::shaders;
use solar_system::texture;

const camera_position: glm::Vector3<f32> = glm::Vector3 {
    x: 0.0,
    y: 0.0,
    z: 6.0,
};
const camera_front: glm::Vector3<f32> = glm::Vector3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};
const camera_up: glm::Vector3<f32> = glm::Vector3 {
    x: 0.0,
    y: -1.0,
    z: 0.0,
};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::Samples(Some(4)));
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(1920, 1080, "Solar System", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // window.set_key_polling(true);
    window.make_current();

    window.set_sticky_keys(true);

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

    let matrix_id = unsafe { gl::GetUniformLocation(program_id, "MVP".as_ptr() as *const GLchar) };
    let texture_id =
        unsafe { gl::GetUniformLocation(program_id, "myTextureSampler".as_ptr() as *const GLchar) };

    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    window.set_cursor_mode(CursorMode::Disabled);

    let projection = glm::ext::perspective(glm::radians(45.0), 16.0 / 9.0, 0.1, 100.0);
    let view = glm::ext::look_at(camera_position, camera_position + camera_front, camera_up);

    // 3D OBJECT: SUN
    let sun_model = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let sun_mvp = projection * view * sun_model;

    let (sun_vertexes, sun_uvs, sun_normals) = object::load("./resources/objects/sun.obj");

    let mut sun_vertex_buffer: GLuint = 0;

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

    // 3D OBJECT: EARTH
    let earth_model = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let earth_mvp = projection * view * earth_model;

    let (earth_vertexes, earth_uvs, earth_normals) =
        object::load("./resources/objects/earth_apocalypse.obj");

    let mut earth_vertex_buffer: GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut earth_vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, earth_vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (earth_vertexes.len() * std::mem::size_of::<glm::Vector3<f32>>()) as isize,
            earth_vertexes.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
    }

    let mut earth_uv_buffer: GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut earth_uv_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, earth_uv_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (earth_uvs.len() * std::mem::size_of::<glm::Vector2<f32>>()) as isize,
            earth_uvs.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
    }

    // 3D OBJECT: MOON
    let moon_model = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let moon_mvp = projection * view * moon_model;

    let (moon_vertexes, moon_uvs, moon_normals) = object::load("./resources/objects/moon.obj");

    let mut moon_vertex_buffer: GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut moon_vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, moon_vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (moon_vertexes.len() * std::mem::size_of::<glm::Vector3<f32>>()) as isize,
            moon_vertexes.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
    }

    let mut moon_uv_buffer: GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut moon_uv_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, moon_uv_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (moon_uvs.len() * std::mem::size_of::<glm::Vector2<f32>>()) as isize,
            moon_uvs.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
    }

    // TEXTURE LOADER
    let mut textures: [GLuint; 3] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };

    unsafe {
        gl::GenTextures(3, textures.as_mut_ptr());
    }

    texture::load(textures[0], "./resources/textures/2k_sun.jpg");
    texture::load(textures[1], "./resources/textures/earth_apocalypse.jpg");
    texture::load(textures[2], "./resources/textures/2k_moon.jpg");

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    unsafe {
        gl::DeleteProgram(program_id);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
