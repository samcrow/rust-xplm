// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.


use xplm_sys::graphics::*;

/// Low-level windows
pub mod window;


/// Stores various flags that can be enabled or disabled
#[derive(Debug,Clone)]
pub struct GraphicsState {
    /// Enable status of fog
    ///
    /// During 3-d rendering fog is set up to cause a fade-to-fog effect at the visibility limit.
    pub fog: bool,
    /// Enable status of 3D lighting
    pub lighting: bool,
    /// Enable status of alpha testing
    ///
    /// Alpha testing stops pixels from being rendered to the frame buffer if their alpha is zero.
    pub alpha_testing: bool,
    /// Enable status of alpha blending
    pub alpha_blending: bool,
    /// Enable status of depth testing
    pub depth_testing: bool,
    /// Enable status of depth writing
    pub depth_writing: bool,
    /// The number of textures that are enabled for use
    pub textures: i32,
}

/// Sets the graphics state
pub fn set_state(state: &GraphicsState) {
    unsafe {
        XPLMSetGraphicsState(state.fog as i32,
                             state.textures,
                             state.lighting as i32,
                             state.alpha_testing as i32,
                             state.alpha_blending as i32,
                             state.depth_testing as i32,
                             state.depth_writing as i32);
    }
}

/// Binds a texture ID to a texture number
///
/// This function should be used instead of glBindTexture
pub fn bind_texture(texture_number: i32, texture_id: i32) {
    unsafe {
        XPLMBindTexture2d(texture_number, texture_id);
    }
}

/// Generates texture numbers in a range not reserved for X-Plane.
///
/// This function should be used instead of glGenTextures.
///
/// Texture IDs are placed in the provided slice. If the slice contains more than i32::max_value()
/// elements, no more than i32::max_value() texture IDs will be generated.
pub fn generate_texture_numbers(numbers: &mut [i32]) {
    let count = if numbers.len() < (i32::max_value() as usize) {
        numbers.len() as i32
    } else {
        i32::max_value()
    };
    unsafe {
        XPLMGenerateTextureNumbers(numbers.as_mut_ptr(), count);
    }
}

///
/// Generates a single texture number
///
/// See generate_texture_numbers for more detail.
///
pub fn generate_texture_number() -> i32 {
    let number = 0;
    generate_texture_numbers(&mut [number]);
    number
}
