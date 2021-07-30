use std::collections::HashMap;

// use sdl2::pixels::Color;

// pub const DEFAULT_COLOR: Color = Color::RGBA(255, 255, 255, 255);

// pub const PROPERTY_NATIVE_WIDGET_ADDER: u32 = 0;
// pub const PROPERTY_INVALIDATED: u32 = 1;
pub const PROPERTY_HIDDEN: u32 = 2;
pub const PROPERTY_ORIGIN: u32 = 3;
pub const PROPERTY_SIZE: u32 = 4;
// pub const PROPERTY_TEXT: u32 = 5;
pub const PROPERTY_MAIN_COLOR: u32 = 6;
pub const PROPERTY_BORDER_COLOR: u32 = 7;
pub const PROPERTY_BORDER_WIDTH: u32 = 8;

/// This is a structure that stores properties for Widgets, which can be used to define the object's
/// behavior.
#[derive(Debug, Clone, Default)]
pub struct ViewProperties {
    properties: HashMap<u32, String>,
}

impl ViewProperties {
    /// Sets a value for a property based on its numerical key.
    pub fn set(&mut self, key: u32, value: String) {
        self.properties.insert(key, value);
    }

    // /// Deletes a property value for the given numerical key.
    // pub fn delete(&mut self, key: u32) {
    //     self.properties.remove(&key);
    // }

    // /// Retrieves the value for a property.
    // pub fn get(&mut self, key: u32) -> String {
    //     self.properties
    //         .get(&key)
    //         .unwrap_or(&String::from(""))
    //         .clone()
    // }

    // /// Returns a flag indicating whether or not a property for a numerical key has been set.
    // pub fn is_key_set(&mut self, key: u32) -> bool {
    //     self.properties.contains_key(&key)
    // }

    // /// Stores the color for the specified key.  Format is "r g b a", as numerical values, base 10.
    // /// Sets the invalidate flag afterward.
    // pub fn set_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
    //     self.set(PROPERTY_MAIN_COLOR, format!("{} {} {} {}", r, g, b, a))
    // }

    // pub fn set_color_green(&mut self) {
    //     self.set(PROPERTY_MAIN_COLOR, format!("{} {} {} {}", 0, 255, 0, 255));
    // }

    // pub fn set_border_color(&mut self, color: Color) {
    //     self.set(
    //         PROPERTY_BORDER_COLOR,
    //         format!("{} {} {} {}", color.r, color.g, color.b, color.a),
    //     );
    // }

    /// Sets the size of the `Widget`.
    pub fn set_bounds(&mut self, w: u32, h: u32) {
        self.set(PROPERTY_SIZE, format!("{} {}", w, h));
    }

    /// Sets the origin for the `Widget`.  Does not set the invalidate flag, as the repositioning of
    /// the `Widget` does not require a repaint.
    pub fn set_origin(&mut self, x: u32, y: u32) {
        self.set(PROPERTY_ORIGIN, format!("{} {}", x, y));
    }

    /// Retrieves a color based on the given property key.  If the color cannot be found, the
    /// `default_color` specified will be returned.
    // pub fn get_color(&self, key: u32) -> Color {
    //     if self.properties.contains_key(&key) {
    //         let tokens: Vec<u8> = self
    //             .properties
    //             .get(&key)
    //             .unwrap()
    //             .split(' ')
    //             .map(|x| (u32::from_str_radix(x, 10).unwrap()) as u8)
    //             .collect();

    //         Color::RGBA(tokens[0], tokens[1], tokens[2], tokens[3])
    //     } else {
    //         DEFAULT_COLOR
    //     }
    // }

    /// Retrieves the stored bounds as a tuple.  If the bounds cannot be found, invisible bounds
    /// are returned (0x0).
    pub fn get_bounds(&self) -> (u32, u32) {
        self.get_tuples(PROPERTY_SIZE, (0_u32, 0_u32))
    }

    /// Retrieves the origin of the `Widget`.  If the origin cannot be found, an origin of 0x0 is
    /// returned.
    pub fn get_origin(&self) -> (u32, u32) {
        self.get_tuples(PROPERTY_ORIGIN, (0_u32, 0_u32))
    }

    /// Retrieves the boolean value for a specified property.  If the property has not been set
    /// with `set_bool`, the return will be `false`.  Otherwise, if the specified key exists, and
    /// the value is set to `1`, the return value will be `true`.
    pub fn get_bool(&self, key: u32) -> bool {
        self.properties.contains_key(&key) && self.properties.get(&key).unwrap() == "1"
    }

    /// Retrieves a numeric value assigned to a property as an `i32` value.
    pub fn get_value(&self, key: u32) -> i32 {
        i32::from_str_radix(self.properties.get(&key).unwrap_or(&String::from("0")), 10).unwrap()
    }

    /*
     * PRIVATE MEMBERS
     */
    #[inline]
    fn get_tuples(&self, key: u32, default_tuple: (u32, u32)) -> (u32, u32) {
        if self.properties.contains_key(&key) {
            let tokens: Vec<&str> = self.properties.get(&key).unwrap().split(' ').collect();

            (
                u32::from_str_radix(tokens[0], 10).unwrap(),
                u32::from_str_radix(tokens[1], 10).unwrap(),
            )
        } else {
            default_tuple
        }
    }
}
