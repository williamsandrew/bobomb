const VRAM_SIZE: usize = 16 * 1024;

// http://wiki.nesdev.com/w/index.php/PPU
// http://wiki.nesdev.com/w/index.php/PPU_programmer_reference
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Ppu {
    vram: Vec<u8>,

    Oamaddr: u8, // $2003
    Oamdata: u8, // $2004
    Oamdma: u8, // $4014

    // In docs, these are prefixed with PPU
    Ctrl: u8, // $2000
    Mask: u8, // $2001
    Status: u8, // $2002
    Scroll: u8, // $2005
    Addr: u8, // $2006
    Data: u8, // $2007
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: vec![0; VRAM_SIZE],
            Oamaddr: 0,
            Oamdata: 0,
            Oamdma: 0,

            // https://wiki.nesdev.com/w/index.php/PPU_power_up_state
            Ctrl: 0,
            Mask: 0,
            Status: 0xa0, // Docs aren't clear if this should be 0x80 or 0xa0 on start
            Scroll: 0,
            Addr: 0,
            Data: 0,
        }
    }

    pub fn read_at(&self, address: u16) -> u8 {
        match address {
            0x2002 => self.Status,
            _ => {
                panic!("ppu not implemented yet. access at {:#x}", address);
            }
        }
    }

    // TODO Replace these methods with something less terrible
    pub fn write_scroll(&mut self, value: u8) {
        self.Scroll = value;
    }

    pub fn write_mask(&mut self, value: u8) {
        self.Mask = value;
    }

    pub fn write_ctrl(&mut self, value: u8) {
        self.Ctrl = value;
    }
}