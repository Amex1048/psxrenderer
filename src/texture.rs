use gl::types::GLenum;

#[derive(Debug)]
pub struct Texture2D(u32);

impl Texture2D {
    pub fn new(
        wraps: GLenum,
        wrapt: GLenum,
        mag_filter: GLenum,
        min_filter: GLenum,
        data: Option<&[u8]>,
        format: GLenum,
        gl_type: GLenum,
        dimensions: (u32, u32),
    ) -> Self {
        let texture_id = {
            let mut id = 0;
            unsafe {
                gl::GenTextures(1, &mut id);
                assert_ne!(id, 0);
            }
            id
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wraps as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapt as i32);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                dimensions.0 as i32,
                dimensions.1 as i32,
                0,
                format,
                gl_type,
                data.map(|x| x.as_ptr()).unwrap_or(std::ptr::null()) as *const _,
            );

            if min_filter != gl::NEAREST || mag_filter != gl::NEAREST {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Self(texture_id)
    }
}

impl crate::GlObject for Texture2D {
    fn glid(&self) -> gl::types::GLuint {
        self.0
    }
}
