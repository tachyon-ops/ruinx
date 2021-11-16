pub trait GuiTrait {
    // fn init(
    //     &mut self,
    //     ui: conrod_core::Ui,
    //     device: &wgpu::Device,
    //     queue: &mut wgpu::Queue,
    //     texture_format: wgpu::TextureFormat,
    //     image_map: &mut conrod_core::image::Map<conrod_wgpu::Image>,
    // ) -> conrod_core::Ui;
    // fn gui(&mut self, ui: &mut conrod_core::UiCell);
    // fn theme(&mut self) -> conrod_core::Theme;

    // for Clone trait
    fn box_clone(&self) -> Box<dyn GuiTrait>;
}

impl Clone for Box<dyn GuiTrait> {
    fn clone(&self) -> Box<dyn GuiTrait> {
        self.box_clone()
    }
}
