use gl::types::GLuint;

pub fn load(texture: GLuint, path: &str) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    let data = stb_image::image::load(path);

    match data {
        stb_image::image::LoadResult::Error(message) => panic!(
            "Error loading image of path: {} Error message: {}",
            path, message
        ),
        stb_image::image::LoadResult::ImageU8(image) => unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                image.width as i32,
                image.height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        },
        stb_image::image::LoadResult::ImageF32(image) => unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                image.width as i32,
                image.height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        },
    }
}
