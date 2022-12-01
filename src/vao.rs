#[repr(transparent)]
pub(crate) struct Vao(u32);

impl Vao {
    pub fn zero() -> Self {
        Self(0)
    }

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
            // println!("bind vao");
            gl::BindVertexArray(self.0);
        }

        let result = closure();

        // #[cfg(debug_assertions)]
        unsafe {
            // println!("unbind vao");
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
