extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

#[wasm_bindgen]
pub struct Plasma {
    width: u32,
    height: u32,
    sine: Vec<i32>,
    palette: Vec<Color>,
    buffer: Vec<u8>,
    pos1: u16, 
    pos3: u16, 
    tpos1: u16, 
    tpos2: u16, 
    tpos3: u16, 
    tpos4: u16
}

#[wasm_bindgen]
impl Plasma {

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn buffer(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    fn create_sine_table() -> Vec<i32> {
        let mut idx: f64 = 0.0;
        let table = (0..512).map(|_| {
            idx += 1.0;
            (((0.703125f64 * idx) * 0.0174532f64).sin() * 1024f64) as i32
        })
        .collect();
        return table;
    }
    
    fn create_palette() -> Vec<Color> {
       let palette: Vec<Color> = (0u16..256u16) 
       .map(|i| {
           let double = (i as u8) << 2;
           let invert = 255 - (double + 1);
           if i < 64 {               
            Color {
                r: double,
                g: invert,
                b: 0
            }
           }
           else if i < 128 {
               Color {
                   r: 255,
                   g: double + 1,
                   b: 0
               }
           }
           else if i < 192 {
               Color {
                   r: invert,
                   g: invert,
                   b: 0
               }
           }
           else {
               Color {
                   r: 0,
                   g: double + 1,
                   b: 0
               }
           }
       })
       .collect();       
       return palette;
    }

    pub fn new() -> Plasma {
        utils::set_panic_hook();
        let width = 320u32;
        let height = 200u32;
        let palette = Plasma::create_palette();
        let sine = Plasma::create_sine_table();
        let buffer = vec![0; (width as usize)*(height as usize)*4usize];
        log!("Plasma initialized.");
        Plasma {
            width,
            height,
            sine,
            palette,
            buffer,
            pos1: 0,
            pos3: 0,
            tpos1: 0,
            tpos2: 0,
            tpos3: 0,
            tpos4: 0
        }
    }

    pub fn tick(&mut self) {
        
        let mut next = self.buffer.clone();
        
        self.tpos4 = 0;
        self.tpos3 = self.pos3;
        
        for idx in 0..self.height {
            self.tpos1 = self.pos1 + 5;
            self.tpos2 = 3;
            self.tpos3 &= 511;
            self.tpos4 &= 511;
            for jdx in 0..self.width {
                self.tpos1 &= 511;
                self.tpos2 &= 511;
                let x = self.sine[self.tpos1 as usize] + self.sine[self.tpos2 as usize] +
                    self.sine[self.tpos3 as usize] + self.sine[self.tpos4 as usize];
                let pidx: usize = (128 + (x >> 4)) as usize % 256;                                
                let base = (((idx * self.width) + jdx)*4u32) as usize;
                let ref color = self.palette[pidx];
                next[base] = color.r;
                next[base+1] = color.g;
                next[base+2] = color.b;
                next[base+3] = 255;
                self.tpos1 += 5;
                self.tpos2 += 3;
            }
            self.tpos3 += 1;
            self.tpos4 += 3;            
        }
        self.pos1 += 9;
        self.pos3 += 8;
        self.buffer = next;
    }
}