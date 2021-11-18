mod utils;

extern crate fixedbitset;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width : u32, height : u32) -> Universe {
        let size = (width*height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size { cells.set(i, false)}
        Universe { width, height, cells }
    }

    pub fn tick(&mut self){
        let mut next = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.get_index(x, y);
                let cell = self.cells[i];
                let live_neighbours = self.live_neighbour_count(x, y);
                
                next.set(i, match(cell, live_neighbours) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                });
            }
        }
        
        self.cells = next
    }
    
    pub fn get_image_data(&self) -> Vec<u8> {
        let size = (self.width*self.height*4) as usize;
        let mut image_data: Vec<u8> = vec![255; size];

        for i in 0..(self.width*self.height) as usize {
            let _i : usize = (i*4) as usize;
            if self.cells[i as usize] as bool {
                image_data[_i] = 0;
                image_data[_i+1] = 0;
                image_data[_i+2] = 0;
            }
        }

        image_data
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }
    
    fn live_neighbour_count(&self, x: u32, y: u32) -> u8 {
        let mut count = 0;
        for dy in [self.height-1, 0, 1].iter().cloned() {
            for dx in [self.width-1, 0, 1].iter().cloned() {
                if dy==0 && dx == 0 { continue; }

                let n_y = (y + dy) % self.height;
                let n_x = (x + dx) % self.width;
                let i = self.get_index(n_x, n_y);
                count += self.cells[i] as u8;
            }
        }
        count
    }

    pub fn random(&mut self) {
        let size = (self.width*self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
        for i in 0..size { self.cells.set(i, js_sys::Math::random() < 0.5); }
    }

    pub fn clear(&mut self) {
        let size = (self.width*self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
        for i in 0..size { self.cells.set(i, false); }
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn cells(&self) -> *const u32 { self.cells.as_slice().as_ptr() }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;

        let size = (self.width*self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
        for i in 0..size { self.cells.set(i, false); }
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;

        let size = (self.width*self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
        for i in 0..size { self.cells.set(i, false); }
    }

    pub fn set_cell(&mut self, x : u32, y: u32, value : bool) {
        self.cells.set((x + y * self.width) as usize, value)
    }
}