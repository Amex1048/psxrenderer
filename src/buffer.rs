use crate::GlObject;

pub enum BufferType {
    Array,
    ElementArray,
    Texture,
}

impl From<BufferType> for gl::types::GLenum {
    fn from(buffer: BufferType) -> Self {
        match buffer {
            BufferType::Array => gl::ARRAY_BUFFER,
            BufferType::ElementArray => gl::ELEMENT_ARRAY_BUFFER,
            BufferType::Texture => gl::TEXTURE_BUFFER,
        }
    }
}

pub enum DrawType {
    Stream,
    Static,
    Dynamic,
}

impl From<DrawType> for gl::types::GLenum {
    fn from(draw: DrawType) -> Self {
        match draw {
            DrawType::Stream => gl::STREAM_DRAW,
            DrawType::Static => gl::STATIC_DRAW,
            DrawType::Dynamic => gl::DYNAMIC_DRAW,
        }
    }
}

pub enum ObjectType {
    Float,
    Int,
}

impl From<ObjectType> for gl::types::GLenum {
    fn from(object_type: ObjectType) -> Self {
        match object_type {
            ObjectType::Float => gl::FLOAT,
            ObjectType::Int => gl::INT,
        }
    }
}

pub(crate) trait Buffer: crate::GlObject {
    const BUFFER_TYPE: BufferType;

    fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(Self::BUFFER_TYPE.into(), self.glid());
        }
    }

    fn fill_with(&mut self, data: &[u8], draw_type: DrawType) {
        unsafe {
            self.bind();
            gl::BufferData(
                Self::BUFFER_TYPE.into(),
                std::mem::size_of_val(data) as isize,
                data.as_ptr().cast(),
                draw_type.into(),
            );
        }
    }
}

#[repr(transparent)]
pub(crate) struct Vbo(u32);

impl Vbo {
    pub fn new() -> Self {
        unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);

            assert_ne!(vbo, 0);

            Self(vbo)
        }
    }

    pub fn set_attrib_ptr(
        &mut self,
        location: u32,
        size: i32,
        object_type: ObjectType,
        normalized: bool,
    ) {
        unsafe {
            // gl::BindBuffer(Self::BUFFER_TYPE.into(), self.glid());
            self.bind();
            gl::VertexAttribPointer(
                location,
                size,
                object_type.into(),
                if normalized { gl::TRUE } else { gl::FALSE },
                0,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(location);
        }
    }
}

impl GlObject for Vbo {
    fn glid(&self) -> gl::types::GLuint {
        self.0
    }
}

impl Buffer for Vbo {
    const BUFFER_TYPE: BufferType = BufferType::Array;
}

#[repr(transparent)]
pub(crate) struct Ebo(u32);

impl Ebo {
    pub fn new() -> Self {
        unsafe {
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);

            assert_ne!(ebo, 0);

            Self(ebo)
        }
    }
}

impl GlObject for Ebo {
    fn glid(&self) -> gl::types::GLuint {
        self.0
    }
}

impl Buffer for Ebo {
    const BUFFER_TYPE: BufferType = BufferType::ElementArray;
}
