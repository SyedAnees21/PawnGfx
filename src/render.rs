use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};

use crate::{
    draw::Face_NORMALS,
    error::PResult,
    math::{AffineMatrices, Matrix4, Vector4},
    raster,
    scene::Scene,
    shaders::GlobalUniforms,
};

const DEFAULT_BG_COLOR: u32 = 77;
const DEFAULT_DEPTH: f64 = f64::INFINITY;

pub struct Renderer<'a> {
    window: Arc<Window>,
    framebuffer: Pixels<'a>,
    depth_buffer: Vec<f64>,
}

impl<'a> Renderer<'a> {
    pub fn render(&mut self, scene: &mut Scene) -> PResult<()> {
        let win_size = self.get_window().inner_size();
        let aspect = win_size.width as f64 / win_size.height as f64;

        self.reset_buffers();

        let (frame_buffer, depth_buffer) = self.get_buffers();

        let (scale, position, rotation) = scene.object.get_transforms_props();

        let model = Matrix4::from_transforms(position, scale, rotation);
        let view = scene.camera.get_view_matrix();
        let projection = Matrix4::perspective_matrix(90.0_f64.to_radians(), aspect, 0.1, 100.0);

        let affine = AffineMatrices::from_mvp(model, view, projection);

        let global_uniforms = GlobalUniforms {
            uniforms: affine,
            screen_width: win_size.width as f64,
            screen_height: win_size.height as f64,
        };

        raster::draw_call(
            frame_buffer,
            depth_buffer,
            global_uniforms,
            scene.light,
            scene.object.mesh.iter_triangles(),
        );

        self.framebuffer.render()?;
        Ok(())
    }

    pub fn reset_buffers(&mut self) {
        self.depth_buffer.fill(DEFAULT_DEPTH);
        self.framebuffer.frame_mut().fill(DEFAULT_BG_COLOR as u8);
    }

    pub fn get_buffers(&mut self) -> (&mut [u8], &mut [f64]) {
        (self.framebuffer.frame_mut(), &mut self.depth_buffer)
    }

    pub fn resize_buffers(&mut self, width: u32, height: u32) -> PResult<()> {
        self.depth_buffer
            .resize((width * height) as usize, DEFAULT_DEPTH);
        self.framebuffer.resize_surface(width, height)?;
        self.framebuffer.resize_buffer(width, height)?;

        Ok(())
    }
}

impl<'a> Renderer<'a> {
    #[allow(unused)]
    pub fn get_framebuffer(&mut self) -> &mut [u8] {
        self.framebuffer.frame_mut()
    }

    #[allow(unused)]
    pub fn get_depth_buffer(&mut self) -> &mut Vec<f64> {
        &mut self.depth_buffer
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.window.clone()
    }
}

pub fn initialize_renderer<'a, T>(
    window_title: T,
    window_width: usize,
    window_height: usize,
    maximized: bool,
    target: &EventLoopWindowTarget<()>,
) -> PResult<Renderer<'a>>
where
    T: Into<String>,
{
    let window_builder = WindowBuilder::new();

    let inner = if maximized {
        window_builder
            .with_maximized(true)
            .with_title(window_title)
            .build(target)?
    } else {
        window_builder
            .with_inner_size(winit::dpi::LogicalSize::new(
                window_width as f64,
                window_height as f64,
            ))
            .with_title(window_title)
            .build(target)?
    };

    let window = Arc::new(inner);
    let size = window.inner_size();
    let depth_buffer = vec![f64::INFINITY; (size.width * size.height) as usize];
    let frame_buffer = Pixels::new(
        size.width,
        size.height,
        SurfaceTexture::new(size.width, size.height, window.clone()),
    )?;

    Ok(Renderer {
        window,
        framebuffer: frame_buffer,
        depth_buffer,
    })
}
