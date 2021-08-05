use crate::engine::dom::view_properties::PROPERTY_BORDER_WIDTH;

use super::view_properties::{
    ViewProperties, PROPERTY_BORDER_COLOR, PROPERTY_HIDDEN, PROPERTY_MAIN_COLOR,
};

// use sdl2::pixels::Color;
// SDL
// use sdl2::{rect::Rect, render::Canvas, video::Window};

use uuid::Uuid;

#[derive(Default, Debug, Clone)]
pub struct View {
    pub uuid: Uuid,
    pub id: String,
    pub children: Vec<View>,
    properties: ViewProperties,
}

impl View {
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        eprintln!("View::new UUID: {}", uuid);
        Self {
            uuid,
            ..View::default()
        }
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = String::from(id);
    }

    pub fn add(&mut self, mut view: View) {
        // TODO: we need a cache of children and parents and to do this on draw only!
        let (self_x, self_y) = self.get_properties().get_origin();
        let (x, y) = view.get_properties().get_origin();
        view.set_origin(x + self_x, y + self_y);
        self.children.push(view)
    }

    pub fn get_properties(&mut self) -> &mut ViewProperties {
        &mut self.properties
    }

    pub fn set_property(&mut self, property_key: u32, property_value: String) {
        self.properties.set(property_key, property_value);
    }

    pub fn set_origin(&mut self, x: u32, y: u32) {
        self.properties.set_origin(x, y)
    }

    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.properties.set_bounds(width, height);
    }

    pub fn set_background_color(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.set_property(
            PROPERTY_MAIN_COLOR,
            format!("{} {} {} {}", red, green, blue, alpha),
        );
    }

    pub fn set_border_color(&mut self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.set_property(
            PROPERTY_BORDER_COLOR,
            format!("{} {} {} {}", red, green, blue, alpha),
        );
    }

    pub fn set_border_width(&mut self, width: u32) {
        self.set_property(PROPERTY_BORDER_WIDTH, format!("{}", width));
    }

    pub fn get_views_in_point(&mut self, x: u32, y: u32) -> Uuid {
        let mut found_id: Option<Uuid> = None;
        let mut hidden_widgets: Vec<Uuid> = vec![];

        self.children.iter_mut().for_each(|view| {
            let is_hidden = view.get_properties().get_bool(PROPERTY_HIDDEN);
            let widget_xy = view.get_properties().get_origin();
            let widget_wh = view.get_properties().get_bounds();

            if !is_hidden {
                if x >= widget_xy.0
                    && x <= widget_xy.0 + widget_wh.0
                    && y >= widget_xy.1
                    && y <= widget_xy.1 + widget_wh.1
                {
                    if !hidden_widgets.contains(&view.uuid) {
                        found_id = Some(view.uuid);
                        // TODO: we need to figure recursiveness
                        // view.get_views_in_point(x, y); // test
                    }
                }
            } else {
                hidden_widgets.push(view.uuid);
            }
        });
        match found_id {
            Some(uuid) => uuid,
            None => self.uuid,
        }
    }

    // pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
    pub fn draw(&mut self) {
        todo!();
        // //Window size: canvas.window().size();

        // // let is_hidden = &self.root.properties().get_bool(PROPERTY_HIDDEN);
        // let base_color: Color = self.properties.get_color(PROPERTY_MAIN_COLOR);

        // // This is the border paint color.
        // let border_color = self.properties.get_color(PROPERTY_BORDER_COLOR);

        // // Border width
        // let border_width = self.properties.get_value(PROPERTY_BORDER_WIDTH);

        // let bounds = self.properties.get_bounds();

        // let widget_xy = self.properties.get_origin();
        // let widget_wh = self.properties.get_bounds();

        // self.texture_store
        //     .create_or_resize_texture(canvas, bounds.0, bounds.1);

        // canvas
        //     .with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
        //         // Fill the texture
        //         texture.set_draw_color(base_color);
        //         texture.clear();

        //         if border_width > 0 {
        //             // Draw the border with the color of the border
        //             texture.set_draw_color(border_color);

        //             // This creates as many 1px rects inside the view as border_width there is
        //             for border_width_count in 0..border_width {
        //                 texture
        //                     .draw_rect(Rect::new(
        //                         border_width_count,
        //                         border_width_count,
        //                         bounds.0 - border_width_count as u32 * 2,
        //                         bounds.1 - border_width_count as u32 * 2,
        //                     ))
        //                     .unwrap();
        //             }
        //         }
        //     })
        //     .unwrap();

        // match self.texture_store.get_optional_ref() {
        //     Some(texture) => {
        //         canvas
        //             .copy(
        //                 texture,
        //                 None,
        //                 Rect::new(
        //                     widget_xy.0 as i32,
        //                     widget_xy.1 as i32,
        //                     widget_wh.0,
        //                     widget_wh.1,
        //                 ),
        //             )
        //             .unwrap();
        //     }
        //     None => eprintln!("No texture presented: ID={}", &self.id),
        // }

        // &self
        //     .children
        //     .iter_mut()
        //     .for_each(|child| child.draw(canvas));
    }
}

// pub trait ViewTrait {
//     fn properties(&mut self) -> &mut ViewProperties;

//     // fn draw(&mut self) -> Option<u32> {
//     //     None
//     // }
// }
