use std::path::Path;

#[derive(Debug)]
pub struct Shader(pub(super) u32);

impl Shader {
    pub fn from_file<P: AsRef<Path>>(
        path: P,
        shader_type: gl::types::GLenum,
    ) -> Result<Shader, String> {
        let source = std::fs::read_to_string(path).unwrap();
        Shader::from_source(&source, shader_type)
    }

    pub fn from_source(source: &str, shader_type: gl::types::GLenum) -> Result<Shader, String> {
        unsafe {
            let shader = match gl::CreateShader(shader_type) {
                0 => return Err("Can't allocate shader descriptor".to_string()),
                shader => Shader(shader),
            };

            gl::ShaderSource(
                shader.0,
                1,
                &source.as_bytes().as_ptr().cast(),
                &(source.as_bytes().len().try_into().unwrap()),
            );

            gl::CompileShader(shader.0);

            let success = {
                let mut success = 0;
                gl::GetShaderiv(shader.0, gl::COMPILE_STATUS, &mut success);

                success != 0
            };

            if !success {
                const BUF_SIZE: usize = 1024;
                let mut buf: Vec<u8> = Vec::with_capacity(BUF_SIZE);
                let mut log_len = 0;
                gl::GetShaderInfoLog(
                    shader.0,
                    BUF_SIZE as i32,
                    &mut log_len,
                    buf.as_mut_ptr().cast(),
                );

                buf.set_len(log_len.try_into().unwrap());
                Err(String::from_utf8_lossy(&buf).to_string())
            } else {
                Ok(shader)
            }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.0) }
    }
}
