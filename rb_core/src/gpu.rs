pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

const OAM_BEGIN: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;
const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

#[derive(Copy, Clone)]
enum Tilepixelvalues {
    Zero,
    One,
    Two,
    Three
}
impl Default for Tilepixelvalues {
    fn default() -> Self {
        Tilepixelvalues::Zero
    }
}
type TileRow = [Tilepixelvalues; 8];
type Tile = [TileRow; 8];
#[inline(always)]
fn empty_tile() -> Tile {
    [[Default::default(); 8]; 8]
}
// type Tile = [[Tilepixelvalues; 8]; 8];

// fn emptytile() -> Tile {
//     [[Tilepixelvalues::Zero; 8]; 8]
// }

pub struct GPU {
    vram: [u8; VRAM_SIZE],

    tile_set: [Tile; 384],
}

impl GPU {
    pub fn new() -> GPU {
        GPU { 
            vram: [0; VRAM_SIZE],
            tile_set: [empty_tile(); 384],
        }
    }
    pub fn read_vram(&self, addr: usize) -> u8 {
        self.vram[addr]
    }
    pub fn write_vram(&mut self, addr: usize, val: u8) {
        self.vram[addr] = val;

        if addr >= 0x1800 { return }

        let normalized_addr = addr & 0xfffe;

        let b1 = self.vram[normalized_addr];
        let b2 = self.vram[normalized_addr + 1];

        let tileindex = addr/16;

        let row = (addr % 16)/2;

        for pixel in 0..8 {
            let mask = 1 << (7 - pixel);
            let lsb = b1 & mask;
            let msb = b2 & mask;

            let value = match (lsb != 0, msb != 0) {
                (true, true) => Tilepixelvalues::Three,
                (false, true) => Tilepixelvalues::Two,
                (true, false) => Tilepixelvalues::One,
                (false, false) => Tilepixelvalues::Zero,
            };

            self.tile_set[tileindex][row][pixel] = value;
        }
    }
}