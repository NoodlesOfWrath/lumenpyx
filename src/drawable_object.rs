use std::any::TypeId;

use crate::load_image;
use crate::shaders::draw_generate_normals;
use crate::LumenpyxProgram;
use crate::Transform;
use crate::Vertex;
use crate::DEFAULT_BEHAVIOR;
use glium;
use glium::glutin::surface::WindowSurface;
use glium::uniform;
use glium::Surface;

const BASE_VERTEX_SHADER_SRC: &str = include_str!("../shaders/ahr_shaders/sprite_ahr_shader.vert");
const BASE_FRAGMENT_SHADER_SRC: &str =
    include_str!("../shaders/ahr_shaders/sprite_ahr_shader.frag");

pub trait Drawable {
    /// Draw the object to the screen
    fn draw(
        &self,
        program: &LumenpyxProgram,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    );

    /// Load the shaders for the object
    /// This is called every frame, so make sure to check
    /// if the shader is already loaded or your performance will suffer
    fn try_load_shaders(&self, program: &mut LumenpyxProgram);
}

pub struct Sprite {
    albedo_texture: glium::texture::Texture2d,
    height_texture: glium::texture::Texture2d,
    roughness_texture: glium::texture::Texture2d,
    normal_texture: glium::texture::Texture2d,
    transform: Transform,
}

impl Sprite {
    pub fn new(
        albedo_path: &str,
        height_path: &str,
        roughness_path: &str,
        display: &glium::Display<WindowSurface>,
        indices: &glium::index::NoIndices,
        transform: Transform,
    ) -> Sprite {
        let albedo_image = load_image(albedo_path);
        let albedo_texture = glium::texture::Texture2d::new(display, albedo_image).unwrap();

        let height_image = load_image(height_path);
        let height_texture = glium::texture::Texture2d::new(display, height_image).unwrap();

        let roughness_image = load_image(roughness_path);
        let roughness_texture = glium::texture::Texture2d::new(display, roughness_image).unwrap();

        let normal_texture = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            albedo_texture.get_width(),
            albedo_texture
                .get_height()
                .expect("Failed to get height of albedo texture"),
        )
        .unwrap();

        {
            let height_uniform = glium::uniforms::Sampler(&height_texture, DEFAULT_BEHAVIOR);
            let albedo_uniform = glium::uniforms::Sampler(&albedo_texture, DEFAULT_BEHAVIOR);
            let mut normal_framebuffer =
                glium::framebuffer::SimpleFrameBuffer::new(display, &normal_texture).unwrap();

            draw_generate_normals(
                display,
                height_uniform,
                albedo_uniform,
                &indices,
                &mut normal_framebuffer,
            )
        }

        Sprite {
            albedo_texture,
            height_texture,
            normal_texture,
            roughness_texture,
            transform,
        }
    }
}

fn generate_shape(dims_1: [u32; 2], dims_2: [u32; 2]) -> Vec<Vertex> {
    let scaling_factor = [
        dims_2[0] as f32 / dims_1[0] as f32,
        dims_2[1] as f32 / dims_1[1] as f32,
    ];

    vec![
        Vertex {
            position: [-1.0 * scaling_factor[0], -1.0 * scaling_factor[1]],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [1.0 * scaling_factor[0], -1.0 * scaling_factor[1]],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [1.0 * scaling_factor[0], 1.0 * scaling_factor[1]],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [1.0 * scaling_factor[0], 1.0 * scaling_factor[1]],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0 * scaling_factor[0], 1.0 * scaling_factor[1]],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-1.0 * scaling_factor[0], -1.0 * scaling_factor[1]],
            tex_coords: [0.0, 0.0],
        },
    ]
}

impl Drawable for Sprite {
    fn draw(
        &self,
        program: &LumenpyxProgram,
        albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
    ) {
        let indices = &program.indices;
        let display = &program.display;

        let shader = program.get_shader("sprite_shader").unwrap();
        let sprite_dimensions = [
            self.albedo_texture.get_width(),
            self.albedo_texture.get_height().unwrap(),
        ];

        let shape = generate_shape(program.dimensions, sprite_dimensions);

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

        let matrix = self.transform.matrix;

        let mut image = glium::uniforms::Sampler(&self.albedo_texture, DEFAULT_BEHAVIOR);

        let uniform = &uniform! {
            matrix: matrix,
            image: image,
        };

        albedo_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniform,
                &Default::default(),
            )
            .unwrap();

        image = glium::uniforms::Sampler(&self.height_texture, DEFAULT_BEHAVIOR);
        let uniform = &uniform! {
            matrix: matrix,
            image: image,
        };
        height_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniform,
                &Default::default(),
            )
            .unwrap();

        image = glium::uniforms::Sampler(&self.roughness_texture, DEFAULT_BEHAVIOR);
        let uniform = &uniform! {
            matrix: matrix,
            image: image,
        };
        roughness_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniform,
                &Default::default(),
            )
            .unwrap();

        image = glium::uniforms::Sampler(&self.normal_texture, DEFAULT_BEHAVIOR);
        let uniform = &uniform! {
            matrix: matrix,
            image: image,
        };
        normal_framebuffer
            .draw(
                &vertex_buffer,
                indices,
                &shader,
                uniform,
                &Default::default(),
            )
            .unwrap();
    }

    fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
        if program.get_shader("sprite_shader").is_some() {
            return;
        }

        let new_shader = glium::Program::from_source(
            &program.display,
            BASE_VERTEX_SHADER_SRC,
            BASE_FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();

        program.add_shader(new_shader, "sprite_shader");
    }
}
