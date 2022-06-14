pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

const NUMBER_OF_OBJECTS: usize = 40;

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
#[derive(Copy, Clone)]
pub struct ObjectData {
    x: i16,
    y: i16,
    tile: u8,
    palette: ObjectPalette,
    xflip: bool,
    yflip: bool,
    priority: bool,
}

impl Default for ObjectData {
    fn default() -> Self {
        ObjectData {
            x: -16,
            y: -8,
            tile: Default::default(),
            palette: Default::default(),
            xflip: Default::default(),
            yflip: Default::default(),
            priority: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
enum ObjectPalette {
    Zero,
    One,
}
impl Default for ObjectPalette {
    fn default() -> Self {
        ObjectPalette::Zero
    }
}
pub struct GPU {
    pub vram: [u8; VRAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub object_data: [ObjectData; NUMBER_OF_OBJECTS],
    tile_set: [Tile; 384],
}

impl GPU {
    pub fn new() -> GPU {
        GPU { 
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            object_data: [Default::default(); NUMBER_OF_OBJECTS],
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

    pub fn write_oam(&mut self, index: usize, value: u8) {
        self.oam[index] = value;
        let object_index = index / 4;
        if object_index > NUMBER_OF_OBJECTS {
            return;
        }

        let byte = index % 4;

        let mut data = self.object_data.get_mut(object_index).unwrap();
        match byte {
            0 => data.y = (value as i16) - 0x10,
            1 => data.x = (value as i16) - 0x8,
            2 => data.tile = value,
            _ => {
                data.palette = if (value & 0x10) != 0 {
                    ObjectPalette::One
                } else {
                    ObjectPalette::Zero
                };
                data.xflip = (value & 0x20) != 0;
                data.yflip = (value & 0x40) != 0;
                data.priority = (value & 0x80) == 0;
            }
        }
    }
}