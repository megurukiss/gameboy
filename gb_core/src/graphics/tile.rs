/*
Tile is 8x8 pixels, each pixel is 2 bits. Each tile rols is 2 bytes, 16 bytes per tile.
00111100 01111110 corresponds to the tile row below:
    00 10 11 11 11 11 10 00

VRAM 0x8000 - 0x97FF stores tile data, can store 384 tiles.

objects are made of 1 or 2 stacked tiles. 8×8 or 8×16
Several objects can be combined to form a sprite.

$9800-$9BFF, $9C00-$9FFF stores two 32×32 tile maps, one for background and one for window.
Each tile map is a 1 byte pointer to the tile data in VRAM.
32 x 8 = 256, each map has 256 x 256 pixels.
While game boy LCD is 160 x 144 pixels, the screen only displays 160 x 144 pixels = 20 x 18 tiles.
*/
