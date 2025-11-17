/*
Tile is 8x8 pixels, each pixel is 2 bits. Each tile rols is 2 bytes, 16 bytes per tile.
00111100 01111110 corresponds to the tile row below:
    00 10 11 11 11 11 10 00

VRAM 0x8000 - 0x97FF stores tile data, can store 384 tiles.

objects are made of 1 or 2 stacked tiles. 8×8 or 8×16
Several objects can be combined to form a sprite.

3 blocks,
block 0: 8000 - 87FF, objects, bg/win lcdc.4 = 1
block 1: 8800 - 8FFF, objects, bg/win lcdc.4 = 1, lcdc.4 = 0
block 2: 8800 - 8FFF, 9000–97FF, bg/win lcdc.4 = 0

$9800-$9BFF, $9C00-$9FFF stores two 32×32 tile maps, one for background and one for window.
Each tile map is a 1 byte pointer to the tile data in VRAM.
32 x 8 = 256, each map has 256 x 256 pixels.
While game boy LCD is 160 x 144 pixels, the screen only displays 160 x 144 pixels = 20 x 18 tiles.

SCX and SCY registers control the background scrolling.
if visible area will be wrapped around, meaning no overflow.

The content of the window is not scrollable.
The only way to modify the window is to change its position using the WX and WY registers.
*/

pub struct Tile {
    data: [u8; 16],
}

impl Tile {
    pub fn from_bytes(bytes: &[u8]) -> Tile {
        let mut data = [0u8; 16];
        for i in 0..bytes.len() {
            data[i] = bytes[i];
        }
        Tile { data }
    }

    pub fn to_color_index(&self) -> Vec<u8> {
        // Create a vector to hold 64 pixels (8x8 tile).
        let mut pixel_ids = Vec::with_capacity(64);

        // Iterate over the 8 rows of the tile.
        for row in 0..8 {
            // Each row is represented by two bytes.
            let byte1 = self.data[row * 2]; // Least significant bits
            let byte2 = self.data[row * 2 + 1]; // Most significant bits

            // Iterate over the 8 pixels in the current row.
            // We go from 7 down to 0 because the first pixel is in the most significant bit.
            for col in (0..8).rev() {
                // Extract the LSB and MSB for the current pixel.
                let lsb = (byte1 >> col) & 1;
                let msb = (byte2 >> col) & 1;

                // Combine them to get the 2-bit color ID (00, 01, 10, or 11).
                let color_id = (msb << 1) | lsb;

                pixel_ids.push(color_id);
            }
        }
        pixel_ids
    }

    pub fn to_color_index_bytes(&self) -> Vec<u8> {
        let pixel_data = self.to_color_index();
        let mut pixel_bytes = Vec::with_capacity(16);
        let mut byte_data = 0u8;
        for i in 0..pixel_data.len() {
            byte_data = byte_data << 2 | pixel_data[i];
            if i % 4 == 3 {
                pixel_bytes.push(byte_data);
                byte_data = 0;
            }
        }
        pixel_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    #[test_log::test]
    fn test_tile_to_pixels() {
        let tile_data = Tile::from_bytes(
            [
                0x3c, 0x7e, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7e, 0x5e, 0x7e, 0x0a, 0x7c, 0x56,
                0x38, 0x7c,
            ]
            .as_ref(),
        );

        let pixels = tile_data.to_color_index();
        assert_eq!(pixels.len(), 64);
        debug!("Tile pixels: {:?}", pixels);

        let pixel_bytes = tile_data.to_color_index_bytes();
        assert_eq!(pixel_bytes.len(), 16);
        for (i, byte) in pixel_bytes.iter().enumerate() {
            debug!("Byte {}: {:02x}", i, byte);
        }
    }
}
