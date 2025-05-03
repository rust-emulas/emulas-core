pub mod rom;

use rom::header::{ROMHeader, INES_1_0Header};

fn main() {
    // Example 16-byte iNES 1.0 header
    let ines_1_0_data: [u8; 16] = [
        b'N', b'E', b'S', 0x1A, // Magic
        2,    // PRG-ROM size (2 * 16KB = 32KB)
        1,    // CHR-ROM size (1 * 8KB = 8KB)
        0x10, // Flags 6
        0x00, // Flags 7
        0x01, // PRG-RAM size (1 * 8KB)
        0x00, // Flags 9 (TV system)
        0x00, // Flags 10
        0x00, 0x00, 0x00, 0x00, // Unused
        0x00, // Padding for 16 bytes
    ];

    // Parse as iNES 1.0
    let rom_header: ROMHeader<INES_1_0Header> = ROMHeader::new(&ines_1_0_data);

    println!("=== Parsed iNES 1.0 Header ===");
    println!("Magic: {:?}", rom_header.header.base.name);
    println!("PRG-ROM size: {} x 16KB", rom_header.header.base.prg_count);
    println!("CHR-ROM size: {} x 8KB", rom_header.header.base.chr_count);
    println!("Mapper bytes: {:?}", rom_header.header.mapper);
    println!("PRG-RAM size: {} x 8KB", rom_header.header.prg_ram_size);
    println!("TV system: {:?}", rom_header.header.tv_system);
    println!("Unused: {:?}", rom_header.header.unused);
}
