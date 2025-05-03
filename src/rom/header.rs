pub enum HeaderVersion {
	INES1_0,
	INES2_0,
	Unknown,
    }

pub struct BaseHeader {
    pub name: [char; 4], // Should be 'N', 'E', 'S', '\x1A'
    pub prg_count: u8,   // PRG-ROM size in 16KB units
    pub chr_count: u8,   // CHR-ROM size in 8KB units
}

pub struct INES_1_0Header {
    pub base: BaseHeader,
    pub mapper: [u8; 2], // [flags 6, flags 7]. Flags 6 – Mapper, mirroring, battery, trainer. Flags 7 – Mapper high nibble, Vs.
    pub prg_ram_size: u8, // PRG-RAM size in 8KB units (rarely used extension)
    pub tv_system: [u8; 2], // [flags 9, flags 10]. Flags 9 – TV system (rarely used extension). Flags 10 – TV system, PRG-RAM presence (unofficial, rarely used extension).
    pub unused: [u8; 4],    // Unused padding bytes (should be 0x00)
}

pub struct INES_2_0Header {
    base: BaseHeader,
    mapper: [u8; 3], // [flags 6, flags 7, flags 8]. Flags 6 – Mapper, mirroring, battery, trainer. Flags 7 – Mapper, VS/Playchoice, NES 2.0. Flags 8 – Mapper highest nibble, mapper variant.
    upper_bits_rom_size: u8, // Upper bits of PRG-ROM size.
    prg_ram_size: u8, // PRG RAM size (logarithmic; battery and non-battery)
    vram_size: u8,   // VRAM size (logarithmic; battery and non-battery)
    tv_system: u8,   // TV system
    vs_ppu: u8,      // Vs. PPU variant
    unused: [u8; 2], // Unused padding bytes (should be 0x00)
}

pub trait HeaderParser {
    fn parse(data: &[u8]) -> Self;
}

// iNES 1.0
impl HeaderParser for INES_1_0Header {
    fn parse(data: &[u8]) -> Self {
        let base = BaseHeader {
            name: [
                data[0] as char,
                data[1] as char,
                data[2] as char,
                data[3] as char,
            ],
            prg_count: data[4],
            chr_count: data[5],
        };
        let mapper = [data[6], data[7]];
        let prg_ram_size = data[8];
        let tv_system = [data[9], data[10]];
        let unused = [data[11], data[12], data[13], data[14]];

        INES_1_0Header {
            base,
            mapper,
            prg_ram_size,
            tv_system,
            unused,
        }
    }
}

// iNES 2.0
impl HeaderParser for INES_2_0Header {
    fn parse(data: &[u8]) -> Self {
        let base = BaseHeader {
            name: [
                data[0] as char,
                data[1] as char,
                data[2] as char,
                data[3] as char,
            ],
            prg_count: data[4],
            chr_count: data[5],
        };
        let mapper = [data[6], data[7], data[8]];
        let upper_bits_rom_size = data[9];
        let prg_ram_size = data[10];
        let vram_size = data[11];
        let tv_system = data[12];
        let vs_ppu = data[13];
        let unused = [data[14], data[15]];

        INES_2_0Header {
            base,
            mapper,
            upper_bits_rom_size,
            prg_ram_size,
            vram_size,
            tv_system,
            vs_ppu,
            unused,
        }
    }
}

pub struct ROMHeader<T> {
    pub header: T,
}

impl<T: HeaderParser> ROMHeader<T> {
    pub fn new(data: &[u8]) -> Self {
        if &data[0..4] != b"NES\x1A" {
            panic!("Invalid iNES header signature");
        }

        let header = T::parse(data);

        ROMHeader { header }
    }
}
