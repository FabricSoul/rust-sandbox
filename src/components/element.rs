use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Liquid,
    MovableSolid,
    ImmovableSolid,
    Gas,
    Erase,
}

#[derive(Component, Clone, Copy)]
pub struct ColorValue {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorValue {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        ColorValue { r, g, b }
    }

    pub fn to_bevy_color(&self) -> Color {
        Color::srgb(self.r, self.g, self.b)
    }
}

#[derive(Component, Clone)]
pub struct Element {
    pub element_type: ElementType,
    pub element: String,
    pub mass: f32,
    pub friction: f32,
    pub dispersion_rate: f32,
    pub color: ColorValue,
}

impl Element {
    pub fn new(element: String) -> Self {
        match element.as_str() {
            "Water" => Element {
                element_type: ElementType::Liquid,
                element,
                mass: 1.,
                friction: 0.5,
                dispersion_rate: 5.,
                color: ColorValue::new(0.0, 0.0, 1.0),
            },
            "Smoke" => Element {
                element_type: ElementType::Gas,
                element,
                mass: 0.01,
                friction: 0.1,
                dispersion_rate: 20.,
                color: ColorValue::new(0.5, 0.5, 0.5),
            },
            "Sand" => Element {
                element_type: ElementType::MovableSolid,
                element,
                mass: 1.5,
                friction: 0.5,
                dispersion_rate: 5.,
                color: ColorValue::new(1.0, 1.0, 0.6),
            },
            "Stone" => Element {
                element_type: ElementType::ImmovableSolid,
                element,
                mass: 5.0,
                friction: 5.0,
                dispersion_rate: 0.,
                color: ColorValue::new(0.6, 0.6, 0.6),
            },
            "Erase" => Element {
                element_type: ElementType::Erase,
                element,
                mass: 0.0,
                friction: 0.0,
                dispersion_rate: 0.0,
                color: ColorValue::new(1.0, 0.0, 0.0), // Changed to red for visibility
            },
            _ => Element {
                element_type: ElementType::ImmovableSolid,
                element,
                mass: 1.,
                friction: 0.5,
                dispersion_rate: 5.,
                color: ColorValue::new(0.0, 0.0, 0.0),
            },
        }
    }

    pub fn get_color_with_random_alpha(&self) -> Color {
        let mut rng = rand::thread_rng();
        let random_alpha = rng.gen_range(0.5..=1.0);
        Color::srgba(self.color.r, self.color.g, self.color.b, random_alpha)
    }

    pub fn get_color_with_alpha(&self, alpha_value: f32) -> Color {
        Color::srgba(self.color.r, self.color.g, self.color.b, alpha_value)
    }
}
