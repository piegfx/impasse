use std::ffi::c_void;

use super::Color;

#[derive(Debug)]
pub enum PieErrorType {
    ShaderError
}

#[derive(Debug)]
pub struct PieError {
    pub error_type: PieErrorType,
    pub description: String
}

impl PieError {
    pub fn new(error_type: PieErrorType, description: String) -> Self {
        Self {
            error_type,
            description
        }
    }
}

pub struct GraphicsDevice {
    vao: u32
}

impl GraphicsDevice {
    pub fn new() -> Self {
        unsafe { 
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            Self {
                vao
            }
        }
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn create_buffer<T: Sized>(&self, buffer_type: BufferType, data: &[T], dynamic: bool) -> GraphicsBuffer {
        unsafe {
            let mut buffer = 0;
            gl::GenBuffers(1, &mut buffer);

            let gl_type = match buffer_type {
                BufferType::VertexBuffer => gl::ARRAY_BUFFER,
                BufferType::IndexBuffer => gl::ELEMENT_ARRAY_BUFFER,
                BufferType::UniformBuffer => gl::UNIFORM_BUFFER,
            };

            gl::BindBuffer(gl_type, buffer);
            gl::BufferData(gl_type, (data.len() * std::mem::size_of::<T>()) as isize, data.as_ptr() as *const _, if dynamic { gl::DYNAMIC_DRAW } else { gl::STATIC_DRAW });

            GraphicsBuffer {
                buffer,
                buffer_type
            }
        }
    }

    pub fn create_shader(&self, attachments: &[ShaderAttachment]) -> Result<Shader, PieError> {
        unsafe {
            let program = gl::CreateProgram();
            let mut gl_shaders = Vec::with_capacity(attachments.len());

            // Compile & attach the shaders to the program.
            for attachment in attachments {
                let gl_stage = match attachment.stage {
                    ShaderStage::Vertex => gl::VERTEX_SHADER,
                    ShaderStage::Fragment => gl::FRAGMENT_SHADER,
                };

                let gl_shader = gl::CreateShader(gl_stage);
                gl_shaders.push(gl_shader);

                // I hate everything about this.
                if let Ok(c_shader) = std::ffi::CString::new(attachment.source.as_bytes()) {
                    gl::ShaderSource(gl_shader, 1, &c_shader.as_ptr(), std::ptr::null());
                } else {
                    return Err(PieError::new(PieErrorType::ShaderError, String::from("Failed to convert shader attachment to C-compatible string. Make sure it contains only ASCII characters.")));
                }

                gl::CompileShader(gl_shader);
                let mut status = 0;
                gl::GetShaderiv(gl_shader, gl::COMPILE_STATUS, &mut status);

                // I hate everything about this EVEN MORE
                if status != gl::TRUE as i32 {
                    let mut length = 0;
                    gl::GetShaderiv(gl_shader, gl::INFO_LOG_LENGTH, &mut length);
                    let mut text_buf = Vec::with_capacity(length as usize);
                    text_buf.set_len((length as usize) - 1);
                    gl::GetShaderInfoLog(gl_shader, text_buf.len() as i32, std::ptr::null_mut(), text_buf.as_mut_ptr() as *mut gl::types::GLchar);

                    if let Ok(r_str) = std::str::from_utf8(&text_buf) {
                        let err = format!("Error compiling {:?} shader: {}", attachment.stage, r_str);
                        return Err(PieError::new(PieErrorType::ShaderError, err));
                    } else {
                        return Err(PieError::new(PieErrorType::ShaderError, String::from("Failed to convert OpenGL string to rust string.")));
                    }
                }

                gl::AttachShader(program, gl_shader);
            }

            // I hate this too but this gets a pass cause its pretty much a duplicate of the shader attachment code.
            // Link program.
            gl::LinkProgram(program);
            let mut status = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

            if status != gl::TRUE as i32 {
                let mut length = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length);
                let mut text_buf = Vec::with_capacity(length as usize);
                text_buf.set_len((length as usize) - 1);
                gl::GetProgramInfoLog(program, text_buf.len() as i32, std::ptr::null_mut(), text_buf.as_mut_ptr() as *mut _);

                if let Ok(r_str) = std::str::from_utf8(&text_buf) {
                    let err = format!("Error linking program:\n{}", r_str);
                    return Err(PieError::new(PieErrorType::ShaderError, err));
                } else {
                    return Err(PieError::new(PieErrorType::ShaderError, String::from("Failed to convert OpenGL string to rust-compatible string.")));
                }
            }

            // Detach and delete shaders.
            for shader in gl_shaders {
                gl::DetachShader(program, shader);
                gl::DeleteShader(shader);
            }

            Ok(Shader { program })
        }
    }

    pub fn create_input_layout(&self, descriptions: &[InputLayoutDescription]) -> InputLayout {
        InputLayout {
            descriptions: descriptions.to_vec()
        }
    }

    pub fn set_shader(&self, shader: &Shader) {
        unsafe {
            gl::UseProgram(shader.program);
        }
    }

    pub fn set_vertex_buffer(&self, buffer: &GraphicsBuffer, stride: u32, layout: &InputLayout) {
        unsafe {
            let mut location = 0;
            for description in &layout.descriptions {
                gl::EnableVertexAttribArray(location);

                match description.format {
                    Format::R8G8B8A8UNorm => gl::VertexAttribPointer(location, 4, gl::UNSIGNED_BYTE, gl::TRUE, stride as i32, description.offset as *const _),
                    Format::R32Float => gl::VertexAttribPointer(location, 1, gl::FLOAT, gl::FALSE, stride as i32, description.offset as *const _),
                    Format::R32G32Float => gl::VertexAttribPointer(location, 2, gl::FLOAT, gl::FALSE, stride as i32, description.offset as *const _),
                    Format::R32G32B32Float => gl::VertexAttribPointer(location, 3, gl::FLOAT, gl::FALSE, stride as i32, description.offset as *const _),
                    Format::R32G32B32A32Float => gl::VertexAttribPointer(location, 4, gl::FLOAT, gl::FALSE, stride as i32, description.offset as *const _),
                }

                location += 1;
            }

            let gl_type = match buffer.buffer_type {
                BufferType::VertexBuffer => gl::ARRAY_BUFFER,
                BufferType::IndexBuffer => gl::ELEMENT_ARRAY_BUFFER,
                BufferType::UniformBuffer => gl::UNIFORM_BUFFER,
            };

            gl::BindBuffer(gl_type, buffer.buffer);
        }
    }

    pub fn set_index_buffer(&self, buffer: &GraphicsBuffer) {
        unsafe {
            let gl_type = match buffer.buffer_type {
                BufferType::VertexBuffer => gl::ARRAY_BUFFER,
                BufferType::IndexBuffer => gl::ELEMENT_ARRAY_BUFFER,
                BufferType::UniformBuffer => gl::UNIFORM_BUFFER,
            };

            gl::BindBuffer(gl_type, buffer.buffer);
        }
    }

    pub fn draw_indexed(&self, index_count: u32) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, index_count as i32, gl::UNSIGNED_INT, 0 as *const _);
        }
    }
}

pub enum BufferType {
    VertexBuffer,
    IndexBuffer,
    UniformBuffer
}

pub struct GraphicsBuffer {
    buffer:      u32,
    buffer_type: BufferType
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderStage {
    Vertex,
    Fragment
}

#[derive(Debug, Clone, Copy)]
pub struct ShaderAttachment<'a> {
    pub stage:  ShaderStage,
    pub source: &'a str
}

pub struct Shader {
    program: u32
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    R8G8B8A8UNorm,

    R32Float,
    R32G32Float,
    R32G32B32Float,
    R32G32B32A32Float
}

#[derive(Debug, Clone, Copy)]
pub struct InputLayoutDescription {
    pub format: Format,
    pub offset:  u32
}

pub struct InputLayout {
    descriptions: Vec<InputLayoutDescription>
}