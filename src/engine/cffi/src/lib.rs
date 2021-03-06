use std::char::from_u32_unchecked;
use std::mem::MaybeUninit;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::{KimeInputResultType as InputResultType, KimeModifierState as ModifierState};

pub const MODIFIER_CONTROL: ModifierState = 1;
pub const MODIFIER_SUPER: ModifierState = 2;
pub const MODIFIER_SHIFT: ModifierState = 4;
pub const MODIFIER_ALT: ModifierState = 8;

#[derive(Clone, Copy, Debug)]
pub struct InputResult {
    pub ty: InputResultType,
    pub char1: char,
    pub char2: char,
}

pub struct InputEngine {
    engine: *mut ffi::KimeInputEngine,
}

impl InputEngine {
    pub fn new() -> Self {
        Self {
            engine: unsafe { ffi::kime_engine_new() },
        }
    }

    pub fn update_hangul_state(&self) {
        unsafe { ffi::kime_engine_update_hangul_state(self.engine) }
    }

    pub fn press_key(
        &mut self,
        config: &Config,
        hardware_code: u16,
        state: ModifierState,
    ) -> InputResult {
        let ret =
            unsafe { ffi::kime_engine_press_key(self.engine, config.config, hardware_code, state) };

        unsafe {
            InputResult {
                ty: ret.ty,
                char1: from_u32_unchecked(ret.char1),
                char2: from_u32_unchecked(ret.char2),
            }
        }
    }

    /// `NULL` mean empty
    pub fn preedit_char(&self) -> char {
        unsafe { from_u32_unchecked(ffi::kime_engine_preedit_char(self.engine)) }
    }

    /// `NULL` mean empty
    pub fn reset(&mut self) -> char {
        unsafe { from_u32_unchecked(ffi::kime_engine_reset(self.engine)) }
    }
}

impl Drop for InputEngine {
    fn drop(&mut self) {
        unsafe {
            ffi::kime_engine_delete(self.engine);
        }
    }
}

pub struct Config {
    config: *mut ffi::KimeConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            config: unsafe { ffi::kime_config_load() },
        }
    }

    pub fn xim_font(&self) -> (&str, f64) {
        unsafe {
            let mut ptr = MaybeUninit::uninit();
            let mut len = MaybeUninit::uninit();
            let mut size = MaybeUninit::uninit();
            ffi::kime_config_xim_preedit_font(
                self.config,
                ptr.as_mut_ptr(),
                len.as_mut_ptr(),
                size.as_mut_ptr(),
            );

            (
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    ptr.assume_init(),
                    len.assume_init(),
                )),
                size.assume_init(),
            )
        }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            ffi::kime_config_delete(self.config);
        }
    }
}
