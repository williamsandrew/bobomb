use nes::cpu::{Cpu,Registers,FromAddress,AddressMode};
use super::store::Store;

pub struct Sta { }

impl FromAddress for Sta {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> usize {
        let dest = cpu.translate_address(mode);
        Store::save_destination(cpu, Registers::AC, dest.to_u16());

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::IndirectY => 6,
            AddressMode::Absolute => 4,
            // TODO Make a macro for this
            _ => { panic!("unimplemented address mode {:?} for STA", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_sta_zeropage() {
        let mut cpu = mock_cpu(&[0x85, 0x10]);
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_sta_abs() {
        let mut cpu = mock_cpu(&[0x8d, 0x10, 0x00]);
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }

    #[test]
    fn test_sta_indirect_y() {
        let mut cpu = mock_cpu(&[0x91, 0x10]);
        cpu.mem.write_word(0x0010, 0xaa);
        cpu.Y = 0x10;
        cpu.AC = 0xff;

        let mut result = cpu.mem.read_word(0x00ba);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.execute_instruction();
        result = cpu.mem.read_word(0x00ba);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }
}