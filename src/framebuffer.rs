use crate::{texture::Texture2D, GlObject};

#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct Framebuffer(u32);

impl Framebuffer {
    pub fn new() -> Self {
        unsafe {
            let mut fbo = 0;
            gl::GenFramebuffers(1, &mut fbo);

            assert_ne!(fbo, 0);

            Self(fbo)
        }
    }

    pub fn render_buffer(width: u32, height: u32) -> Result<Self, ()> {
        let mut fbo = Self::new();
        let color_buffer = Texture2D::new(
            gl::REPEAT,
            gl::REPEAT,
            gl::NEAREST,
            gl::NEAREST,
            None,
            gl::RGBA,
            gl::UNSIGNED_SHORT_5_5_5_1,
            (width, height),
        );

        let depth_buffer = Texture2D::new(
            gl::REPEAT,
            gl::REPEAT,
            gl::NEAREST,
            gl::NEAREST,
            None,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            (width, height),
        );

        fbo.as_context(|| unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                color_buffer.glid(),
                0,
            );

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                depth_buffer.glid(),
                0,
            );
        })?;

        Ok(fbo)
    }

    pub fn as_context<F, R>(&mut self, mut closure: F) -> Result<R, ()>
    where
        F: FnMut() -> R,
    {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.0);
        }

        let result = closure();

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Ok(result)
    }
}

impl crate::GlObject for Framebuffer {
    fn glid(&self) -> gl::types::GLuint {
        self.0
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.0);
        }
    }
}
