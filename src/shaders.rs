use gl::types::{GLboolean, GLchar, GLenum, GLint, GLuint};
use std::ffi::CStr;
use std::fs;

pub fn load_shaders(vertex_file_path: &str, fragment_file_path: &str) -> GLuint {
    let vertex_shader_code = fs::read_to_string(vertex_file_path).unwrap();
    let fragment_shader_code = fs::read_to_string(fragment_file_path).unwrap();

    let vertex_shader_id = create_shader(&vertex_shader_code, gl::VERTEX_SHADER);
    let fragment_shader_id = create_shader(&fragment_shader_code, gl::FRAGMENT_SHADER);

    let program_id = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program_id, vertex_shader_id);
        gl::AttachShader(program_id, fragment_shader_id);
    }

    unsafe {
        gl::LinkProgram(program_id);
    }
    check_shader_error(
        program_id,
        gl::LINK_STATUS,
        true,
        "Error: Program linking failed: ",
    );

    unsafe {
        gl::ValidateProgram(program_id);
    }
    check_shader_error(
        program_id,
        gl::VALIDATE_STATUS,
        true,
        "Error: Program validation failed: ",
    );

    unsafe {
        gl::DetachShader(program_id, vertex_shader_id);
        gl::DetachShader(program_id, fragment_shader_id);
    }

    unsafe {
        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);
    }

    program_id
}

fn create_shader(text: &str, shader_type: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };

    // TODO: make better error handling
    if shader == 0 {
        eprintln!("Error: Shader creation failed");
    }

    let shader_source_strings: [*const GLchar; 1] = [text.as_ptr() as *const GLchar];
    let shader_source_string_lengths: [GLint; 1] = [text.len() as GLint];

    unsafe {
        gl::ShaderSource(
            shader,
            shader_source_string_lengths.len() as i32,
            shader_source_strings.as_ptr(),
            shader_source_string_lengths.as_ptr(),
        );
        gl::CompileShader(shader);
    }

    check_shader_error(
        shader,
        gl::COMPILE_STATUS,
        false,
        "Error: Shader compilation failed",
    );

    shader
}

// TODO: make error handling better, like using Result
fn check_shader_error(shader: GLuint, flag: GLuint, is_program: bool, error_message: &str) {
    let mut success: GLint = 0;
    let mut error: [GLchar; 1024] = [0; 1024];

    if is_program {
        unsafe {
            gl::GetProgramiv(shader, flag, &mut success);
        }
    } else {
        unsafe {
            gl::GetShaderiv(shader, flag, &mut success);
        }
    }

    if success as GLboolean == gl::FALSE {
        if is_program {
            unsafe {
                gl::GetProgramInfoLog(
                    shader,
                    error.len() as i32,
                    std::ptr::null_mut(),
                    &mut error[0] as *mut i8,
                );
            }
        } else {
            unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    error.len() as i32,
                    std::ptr::null_mut(),
                    &mut error[0] as *mut i8,
                );
            }
        }

        let error_rust_str = {
            let c_str = unsafe { CStr::from_ptr(&error[0]) };
            c_str.to_str().unwrap()
        };

        eprintln!("{}: '{}'", error_message, error_rust_str)
    }
}
