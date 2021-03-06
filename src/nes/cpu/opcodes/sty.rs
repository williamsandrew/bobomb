use crate::nes::cpu::{Cpu,Registers,FromAddress,AddressMode};
use super::store::Store;

pub struct Sty { }

impl FromAddress for Sty {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (dest, _) = cpu.translate_address(mode);
        Store::save_destination(cpu, Registers::Y, dest.into());

        match mode {
            AddressMode::ZeroPage => 3,
            AddressMode::ZeroPageX => 4,
            AddressMode::Absolute => 4,
            // TODO Make a macro for this
            _ => { panic!("unimplemented address mode {:?} for STY", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;

    #[test]
    fn test_sty_zero() {
        let mut cpu = mock_cpu(&[0x84, 0x10]);
        cpu.Y = 0xff;

        let mut result = cpu.read_at(0x0010);
        assert!(result == 0x00, "expected 0x00, got {:#x}", result);
        cpu.step(None);
        result = cpu.read_at(0x0010);
        assert!(result == 0xff, "expected 0xff, got {:#x}", result);
    }
}
