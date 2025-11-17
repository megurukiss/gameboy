/*
160×144 pixel

Tiles:
pixels are grouped into 8 x 8 squares, called tiles, considered as base unit of graphics.
a tile assigns a color indices to each of its pixels, from 0 to 3.
Game Boy graphics are also called 2bpp (2 bits per pixel).

Palette:
A palette consists of an array of colors, 4 in the Game Boy’s case.
Modifying palettes enables graphical effects such as quickly flashing some graphics, fading the screen, “palette swaps”,

Layers:
The Game Boy has three “layers”, from back to front: the Background, the Window, and the Objects.

Background:
The background is composed of a tilemap. A tilemap is a large grid of tiles.
tiles aren’t directly written to tilemaps, they merely contain references to the tiles.
The background can be made to scroll as a whole, writing to two hardware registers.

Window:
Window is like a second background, but limited.
it’s always a rectangle and only the position of the top-left pixel can be controlled.
Possible usage include a fixed status bar in an otherwise scrolling game

Objects:
background layer is useful for elements scrolling as a whole,
but impractical for objects that need to move separately, such as the player.
objects are made of 1 or 2 stacked tiles, and can be displayed anywhere on the screen.

Sprites:
Several objects can be combined to draw a larger graphical element.
*/

use super::Tile;
use crate::core::Error;
use crate::io_registers::IOResgisters;
use std::result::Result;

pub const VRAM_START: u16 = 0x8000;
pub const BACKGROUND_START: u16 = 0x9800;
pub const WINDOW_START: u16 = 0x9C00;
pub const VRAM_END: u16 = 0x9FFF;

pub const BASE_PONTER_1: u16 = 0x8000; // Base pointer for tile data in VRAM when LCD.4 == 1, 0x8800 - 0x8FFF
pub const BASE_PONTER_2: u16 = 0x9000; // Base pointer for tile data in VRAM when LCD.4 == 0, 0x9000 - 0x97FF
                                       // Object always use BASE_PONTER_1, 0x8000 - 0x87FF
                                       // Background and Window use BASE_PONTER_1 or BASE_PONTER_2 depending on LCDC.4

pub const TILE_SIZE: usize = 16; // 8x8 pixels, each pixel is 2 bits, 2 bytes per row, 16 bytes per tile
pub const TILE_COUNT: usize = 384; // 0x8000 - 0x97FF can store 384 tiles
pub const TILE_MAP_SIZE: usize = 32 * 32; // 32x32 tiles, each tile is 1 byte pointer to the tile data in VRAM

pub struct VRAM {
    pub tiles: Vec<u8>,               // 0x8000 - 0x97FF
    pub background_tile_map: Vec<u8>, // 0x9800 - 0x9BFF
    pub window_tile_map: Vec<u8>,     //  0x9C00 - 0x9FFF
}

impl VRAM {
    pub fn new() -> Self {
        Self {
            tiles: vec![0; TILE_COUNT * TILE_SIZE],
            background_tile_map: vec![0; TILE_MAP_SIZE],
            window_tile_map: vec![0; TILE_MAP_SIZE],
        }
    }

    pub fn read_tile(&self, address: u16) -> Result<Tile, Error> {
        // check address in tile store range
        if address < VRAM_START || address >= BACKGROUND_START as u16 {
            return Err(Error::VRAMAddressError);
        }
        // check address is aligned to TILE_SIZE
        if ((address - VRAM_START) as usize) % TILE_SIZE != 0 {
            return Err(Error::VRAMAddressError);
        }
        let address = (address - VRAM_START) as usize;
        let tile_data = &self.tiles[address..address + TILE_SIZE];
        Ok(Tile::from_bytes(tile_data))
    }

    // read the tile as color index, which is a vector of 64 pixels, each pixel is 2 bit.
    pub fn read_tile_as_color_index(&self, address: u16) -> Result<Vec<u8>, Error> {
        let tile = self.read_tile(address)?;
        Ok(tile.to_color_index())
    }

    pub fn read_background_tile_index(&self, address: u16) -> Result<u8, Error> {
        if address < BACKGROUND_START || address >= WINDOW_START {
            return Err(Error::VRAMAddressError);
        }
        let index = (address - BACKGROUND_START) as usize;

        Ok(self.background_tile_map[index] as u8)
    }

    pub fn read_window_tile_index(&self, address: u16) -> Result<u8, Error> {
        if address < WINDOW_START || address >= VRAM_END {
            return Err(Error::VRAMAddressError);
        }
        let index = (address - WINDOW_START) as usize;

        Ok(self.window_tile_map[index] as u8)
    }

    pub fn read_byte(&self, address: u16) -> Result<u8, Error> {
        match address {
            VRAM_START..BACKGROUND_START => {
                let index = (address - VRAM_START) as usize;
                Ok(self.tiles[index])
            }
            BACKGROUND_START..WINDOW_START => {
                let index = (address - BACKGROUND_START) as usize;
                Ok(self.background_tile_map[index])
            }
            WINDOW_START..VRAM_END => {
                let index = (address - WINDOW_START) as usize;
                Ok(self.window_tile_map[index])
            }
            _ => Err(Error::VRAMAddressError),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        match address {
            VRAM_START..BACKGROUND_START => {
                let address = (address - VRAM_START) as usize;
                self.tiles[address] = value;
            }
            BACKGROUND_START..WINDOW_START => {
                let address = (address - BACKGROUND_START) as usize;
                self.background_tile_map[address] = value;
            }
            WINDOW_START..VRAM_END => {
                let address = (address - WINDOW_START) as usize;
                self.window_tile_map[address] = value;
            }
            _ => return Err(Error::VRAMAddressError),
        }

        Ok(())
    }

    // calculate the tile address, based on the tile index, and lcd.4 value
    pub fn get_tile_address(index: u8, lcd: &[bool]) -> u16 {
        if lcd[4] {
            // lcd.4 == 1, use BASE_PONTER_1
            BASE_PONTER_1 + (index as u16 * TILE_SIZE as u16)
        } else {
            // lcd.4 == 0, use BASE_PONTER_2
            // index interpret as signed, -128 to 127
            BASE_PONTER_2 + (((index as i8) as i16) * (TILE_SIZE as i16)) as u16
        }
    }
}
