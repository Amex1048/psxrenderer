use super::shader::Shader;
use super::uniform::*;

use crate::texture::Texture2D;
use crate::GlObject;

use std::ffi::CString;

#[derive(Debug)]
pub struct Program(u32);

impl Program {
    pub fn from_shaders(shaders: impl IntoIterator<Item = Shader>) -> Result<Self, String> {
        unsafe {
            let program = match gl::CreateProgram() {
                0 => return Err("Can't allocate program descriptor".to_string()),
                program => Program(program),
            };

            for shader in shaders {
                gl::AttachShader(program.0, shader.0);
            }

            gl::LinkProgram(program.0);

            let success = {
                let mut success = 0;
                gl::GetProgramiv(program.0, gl::LINK_STATUS, &mut success);

                success != 0
            };

            if !success {
                const BUF_SIZE: usize = 1024;
                let mut buf: Vec<u8> = Vec::with_capacity(BUF_SIZE);
                let mut log_len = 0;
                gl::GetProgramInfoLog(
                    program.0,
                    BUF_SIZE as i32,
                    &mut log_len,
                    buf.as_mut_ptr().cast(),
                );

                buf.set_len(log_len.try_into().unwrap());
                Err(String::from_utf8_lossy(&buf).to_string())
            } else {
                Ok(program)
            }
        }
    }

    pub fn as_context<F, R>(&self, mut closure: F) -> R
    where
        F: FnMut() -> R,
    {
        unsafe {
            gl::UseProgram(self.0);
        }

        let result = closure();

        #[cfg(debug_assertions)]
        unsafe {
            gl::UseProgram(0);
        }

        result
    }

    pub fn load_uniform_vec<T, const N: usize, V: UniformVec<T, N>>(&mut self, name: &str, vec: V) {
        let name = CString::new(name).unwrap();

        self.as_context(|| unsafe {
            let location = gl::GetUniformLocation(self.0, name.as_ptr());

            assert_ne!(location, -1);

            let data: &[T; N] = vec.as_ref();
            V::LOADER(location, 1, data.as_ptr());
        });
    }

    pub fn load_uniform_mat<T, const N: usize, M: UniformMat<T, N>>(
        &self,
        name: &str,
        transpose: bool,
        mat: M,
    ) {
        let name = CString::new(name).unwrap();

        self.as_context(|| unsafe {
            let location = gl::GetUniformLocation(self.0, name.as_ptr());

            assert_ne!(location, -1);

            let data: &[T; N] = mat.as_ref();
            M::LOADER(
                location,
                1,
                if transpose { gl::TRUE } else { gl::FALSE },
                data.as_ptr(),
            );
        });
    }

    pub fn load_uniform_texture2d(&mut self, texture: &Texture2D, tex_unit: u32) {
        self.as_context(|| unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + tex_unit);
            gl::BindTexture(gl::TEXTURE_2D, texture.glid());
        });
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.0) }
    }
}
