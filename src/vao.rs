#[derive(Debug)]
#[repr(transparent)]
pub(crate) struct Vao(u32);

impl Vao {
    pub fn new() -> Self {
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);

            assert_ne!(vao, 0);

            Self(vao)
        }
    }

    pub fn as_context<F, R>(&mut self, mut closure: F) -> R
    where
        F: FnMut() -> R,
    {
        unsafe {
            gl::BindVertexArray(self.0);
        }

        let result = closure();

        unsafe {
            gl::BindVertexArray(0);
        }

        result
    }
}

impl crate::GlObject for Vao {
    fn glid(&self) -> gl::types::GLuint {
        self.0
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.0);
        }
    }
}
