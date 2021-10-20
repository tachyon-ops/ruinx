mod engine;
mod gui_trait;
mod mode;
mod run;
mod state;

pub use engine::{Engine, RenderError};
pub use gui_trait::GuiTrait;
pub use mode::AppMode;
pub use run::event_loop;
pub use state::State;

pub fn get_win_size(window: &winit::window::Window) -> winit::dpi::LogicalSize<f64> {
    let scale_factor = window.scale_factor();
    let size = window.inner_size();
    size.to_logical(scale_factor)
}

const LOGO_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub fn create_logo_texture(
    device: &wgpu::Device,
    queue: &mut wgpu::Queue,
    image: image::RgbaImage,
) -> wgpu::Texture {
    // Initialise the texture.
    let (width, height) = image.dimensions();
    let logo_tex_extent = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let logo_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("conrod_rust_logo_texture"),
        size: logo_tex_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: LOGO_TEXTURE_FORMAT,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    });

    // Upload the pixel data.
    let data = &image.into_raw()[..];

    // Submit command for copying pixel data to the texture.
    let pixel_size_bytes = 4; // Rgba8, as above.
    let data_layout = wgpu::ImageDataLayout {
        offset: 0,
        bytes_per_row: std::num::NonZeroU32::new(width * pixel_size_bytes),
        rows_per_image: std::num::NonZeroU32::new(height),
    };
    let texture_copy_view = wgpu::ImageCopyTexture {
        texture: &logo_tex,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    };
    let extent = wgpu::Extent3d {
        width: width,
        height: height,
        depth_or_array_layers: 1,
    };
    queue.write_texture(texture_copy_view, data, data_layout, extent);

    logo_tex
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
