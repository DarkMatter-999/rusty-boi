pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

pub const OAM_BEGIN: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;
pub const OAM_SIZE: usize = OAM_END - OAM_BEGIN + 1;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

const NUMBER_OF_OBJECTS: usize = 40;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileMap {
    X9800,
    X9C00,
}

#[derive(Copy, Clone, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ObjectSize {
    OS8X8,
    OS8X16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BackgroundAndWindowDataSelect {
    X8000,
    X8800,
}

#[derive(Copy, Clone)]
pub enum Color {
    White = 255,
    LightGray = 192,
    DarkGray = 96,
    Black = 0,
}
impl std::convert::From<u8> for Color {
    fn from(n: u8) -> Self {
        match n {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => panic!("Cannot covert {} to color", n),
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum InterruptRequest {
    None,
    VBlank,
    LCDStat,
    Both,
}
impl InterruptRequest {
    fn add(&mut self, other: InterruptRequest) {
        match self {
            InterruptRequest::None => *self = other,
            InterruptRequest::VBlank if other == InterruptRequest::LCDStat => {
                *self = InterruptRequest::Both
            }
            InterruptRequest::LCDStat if other == InterruptRequest::VBlank => {
                *self = InterruptRequest::Both
            }
            _ => {}
        };
    }
}
#[derive (Clone, Copy)]
pub enum Mode {
    HorizontalBlank,
    VerticalBlank,
    OAMAccess,
    VRAMAccess,
}
impl std::convert::From<Mode> for u8 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::HorizontalBlank => 0,
            Mode::VerticalBlank => 1,
            Mode::OAMAccess => 2,
            Mode::VRAMAccess => 3,
        }
    }
}

pub struct BackgroundColors(Color, Color, Color, Color);

impl BackgroundColors {
    fn new() -> BackgroundColors {
        BackgroundColors(
            Color::White,
            Color::LightGray,
            Color::DarkGray,
            Color::Black,
        )
    }
}

impl std::convert::From<u8> for BackgroundColors {
    fn from(value: u8) -> Self {
        BackgroundColors(
            (value & 0b11).into(),
            ((value >> 2) & 0b11).into(),
            ((value >> 4) & 0b11).into(),
            (value >> 6).into(),
        )
    }
}

pub struct Window {
    pub x: u8,
    pub y: u8,
}
pub struct GPU {
    pub vram: [u8; VRAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub object_data: [ObjectData; NUMBER_OF_OBJECTS],
    pub canvas_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
    tile_set: [Tile; 384],
    pub viewport_x_offset: u8,
    pub viewport_y_offset: u8,
    pub background_display_enabled: bool,
    pub object_display_enabled: bool,
    pub line: u8,
    pub background_tile_map: TileMap,
    pub background_colors: BackgroundColors,
    pub object_size: ObjectSize,
    pub mode: Mode,
    cycles: u16,
    pub lcd_display_enabled: bool,
    pub window_display_enabled: bool,
    pub window_tile_map: TileMap,
    pub background_and_window_data_select: BackgroundAndWindowDataSelect,
    pub line_equals_line_check_interrupt_enabled: bool,
    pub oam_interrupt_enabled: bool,
    pub vblank_interrupt_enabled: bool,
    pub hblank_interrupt_enabled: bool,
    pub line_equals_line_check: bool,
    pub line_check: u8,
    pub obj_0_color_1: Color,
    pub obj_0_color_2: Color,
    pub obj_0_color_3: Color,
    pub obj_1_color_1: Color,
    pub obj_1_color_2: Color,
    pub obj_1_color_3: Color,
    pub window: Window,
}

impl GPU {
    pub fn new() -> GPU {
        GPU { 
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            object_data: [Default::default(); NUMBER_OF_OBJECTS],
            canvas_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
            tile_set: [empty_tile(); 384],
            viewport_x_offset: 0,
            viewport_y_offset: 0,
            background_display_enabled: true,
            object_display_enabled: true,
            line: 0,
            background_tile_map: TileMap::X9800,
            background_colors: BackgroundColors::new(),
            object_size: ObjectSize::OS8X8,
            mode: Mode::HorizontalBlank,
            cycles: 0,
            lcd_display_enabled: false,
            window_display_enabled: false,
            window_tile_map: TileMap::X9800,
            background_and_window_data_select: BackgroundAndWindowDataSelect::X8800,
            line_equals_line_check_interrupt_enabled: false,
            oam_interrupt_enabled: false,
            vblank_interrupt_enabled: false,
            hblank_interrupt_enabled: false,
            line_equals_line_check: false,
            line_check: 0,
            obj_0_color_1: Color::LightGray,
            obj_0_color_2: Color::DarkGray,
            obj_0_color_3: Color::Black,
            obj_1_color_1: Color::LightGray,
            obj_1_color_2: Color::DarkGray,
            obj_1_color_3: Color::Black,
            window: Window { x: 0, y: 0 },
        }
    }

    pub fn step(&mut self, cycles: u8) -> InterruptRequest {
        let mut request = InterruptRequest::None;
        if !self.lcd_display_enabled {
            return request;
        }
        self.cycles += cycles as u16;

        let mode = self.mode;
        match mode {
            Mode::HorizontalBlank => {
                if self.cycles >= 200 {
                    self.cycles = self.cycles % 200;
                    self.line += 1;

                    if self.line >= 144 {
                        self.mode = Mode::VerticalBlank;
                        request.add(InterruptRequest::VBlank);
                        if self.vblank_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    } else {
                        self.mode = Mode::OAMAccess;
                        if self.oam_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    }
                    self.set_equal_lines_check(&mut request);
                }
            }
            Mode::VerticalBlank => {
                if self.cycles >= 456 {
                    self.cycles = self.cycles % 456;
                    self.line += 1;
                    if self.line == 154 {
                        self.mode = Mode::OAMAccess;
                        self.line = 0;
                        if self.oam_interrupt_enabled {
                            request.add(InterruptRequest::LCDStat)
                        }
                    }
                    self.set_equal_lines_check(&mut request);
                }
            }
            Mode::OAMAccess => {
                if self.cycles >= 80 {
                    self.cycles = self.cycles % 80;
                    self.mode = Mode::VRAMAccess;
                }
            }
            Mode::VRAMAccess => {
                if self.cycles >= 172 {
                    self.cycles = self.cycles % 172;
                    if self.hblank_interrupt_enabled {
                        request.add(InterruptRequest::LCDStat)
                    }
                    self.mode = Mode::HorizontalBlank;
                    self.render_scan_line()
                }
            }
        }
        request
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

    fn render_scan_line(&mut self) {
        let mut scan_line: [Tilepixelvalues; SCREEN_WIDTH] = [Default::default(); SCREEN_WIDTH];
        if self.background_display_enabled {
            // The x index of the current tile
            let mut tile_x_index = self.viewport_x_offset / 8;
            // The current scan line's y-offset in the entire background space is a combination
            // of both the line inside the view port we're currently on and the amount of the view port is scrolled
            let tile_y_index = self.line.wrapping_add(self.viewport_y_offset);
            // The current tile we're on is equal to the total y offset broken up into 8 pixel chunks
            // and multipled by the width of the entire background (i.e. 32 tiles)
            let tile_offset = (tile_y_index as u16 / 8) * 32u16;

            // Where is our tile map defined?
            let background_tile_map = if self.background_tile_map == TileMap::X9800 {
                0x9800
            } else {
                0x9C00
            };
            // Munge this so that the beginning of VRAM is index 0
            let tile_map_begin = background_tile_map - VRAM_BEGIN;
            // Where we are in the tile map is the beginning of the tile map
            // plus the current tile's offset
            let tile_map_offset = tile_map_begin + tile_offset as usize;

            // When line and scrollY are zero we just start at the top of the tile
            // If they're non-zero we must index into the tile cycling through 0 - 7
            let row_y_offset = tile_y_index % 8;
            let mut pixel_x_index = self.viewport_x_offset % 8;

            if self.background_and_window_data_select == BackgroundAndWindowDataSelect::X8800 {
                panic!("TODO: support 0x8800 background and window data select");
            }

            let mut canvas_buffer_offset = self.line as usize * SCREEN_WIDTH * 4;
            // Start at the beginning of the line and go pixel by pixel
            for line_x in 0..SCREEN_WIDTH {
                // Grab the tile index specified in the tile map
                let tile_index = self.vram[tile_map_offset + tile_x_index as usize];

                let tile_value = self.tile_set[tile_index as usize][row_y_offset as usize]
                    [pixel_x_index as usize];
                let color = self.tile_value_to_background_color(&tile_value);

                self.canvas_buffer[canvas_buffer_offset] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 1] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 2] = color as u8;
                self.canvas_buffer[canvas_buffer_offset + 3] = 255;
                canvas_buffer_offset += 4;
                scan_line[line_x] = tile_value;
                // Loop through the 8 pixels within the tile
                pixel_x_index = (pixel_x_index + 1) % 8;

                // Check if we've fully looped through the tile
                if pixel_x_index == 0 {
                    // Now increase the tile x_offset by 1
                    tile_x_index = tile_x_index + 1;
                }
                if self.background_and_window_data_select == BackgroundAndWindowDataSelect::X8800 {
                    panic!("TODO: support 0x8800 background and window data select");
                }
            }
        }

        if self.object_display_enabled {
            let object_height = if self.object_size == ObjectSize::OS8X16 {
                16
            } else {
                8
            };
            for object in self.object_data.iter() {
                let line = self.line as i16;
                if object.y <= line && object.y + object_height > line {
                    let pixel_y_offset = line - object.y;
                    let tile_index = if object_height == 16 && (!object.yflip && pixel_y_offset > 7)
                        || (object.yflip && pixel_y_offset <= 7)
                    {
                        object.tile + 1
                    } else {
                        object.tile
                    };

                    let tile = self.tile_set[tile_index as usize];
                    let tile_row = if object.yflip {
                        tile[(7 - (pixel_y_offset % 8)) as usize]
                    } else {
                        tile[(pixel_y_offset % 8) as usize]
                    };

                    let canvas_y_offset = line as i32 * SCREEN_WIDTH as i32;
                    let mut canvas_offset = ((canvas_y_offset + object.x as i32) * 4) as usize;
                    for x in 0..8i16 {
                        let pixel_x_offset = if object.xflip { (7 - x) } else { x } as usize;
                        let x_offset = object.x + x;
                        let pixel = tile_row[pixel_x_offset];
                        if x_offset >= 0
                            && x_offset < SCREEN_WIDTH as i16
                            && pixel != Tilepixelvalues::Zero
                            && (object.priority
                                || scan_line[x_offset as usize] == Tilepixelvalues::Zero)
                        {
                            let color = self.tile_value_to_background_color(&pixel);

                            self.canvas_buffer[canvas_offset + 0] = color as u8;
                            self.canvas_buffer[canvas_offset + 1] = color as u8;
                            self.canvas_buffer[canvas_offset + 2] = color as u8;
                            self.canvas_buffer[canvas_offset + 3] = 255;
                        }
                        canvas_offset += 4;
                    }
                }
            }
        }

        if self.window_display_enabled {}
    }
    
    fn tile_value_to_background_color(&self, tile_value: &Tilepixelvalues) -> Color {
        match tile_value {
            Tilepixelvalues::Zero => self.background_colors.0,
            Tilepixelvalues::One => self.background_colors.1,
            Tilepixelvalues::Two => self.background_colors.2,
            Tilepixelvalues::Three => self.background_colors.3,
        }
    }

    fn set_equal_lines_check(&mut self, request: &mut InterruptRequest) {
        let line_equals_line_check = self.line == self.line_check;
        if line_equals_line_check && self.line_equals_line_check_interrupt_enabled {
            request.add(InterruptRequest::LCDStat);
        }
        self.line_equals_line_check = line_equals_line_check;
    }

}