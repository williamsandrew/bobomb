use crate::nes::cpu::{Cpu,FromAddress,AddressMode};

pub struct Dec { }

impl Dec {
    #[inline]
    fn decrement(cpu: &mut Cpu, word: u8) -> u8 {
        let result = word.wrapping_sub(1);
        cpu.zero_and_negative_status(result);

        result
    }
}

impl FromAddress for Dec {
    fn from_address(cpu: &mut Cpu, mode: AddressMode) -> u32 {
        let (src, _) = cpu.translate_address(mode);
        let word = cpu.read_at(src.to_u16());
        let result = Self::decrement(cpu, word);

        cpu.write_at(src, result);

        match mode {
            AddressMode::ZeroPage => 5,
            AddressMode::ZeroPageX => 6,
            AddressMode::Absolute => 6,
            AddressMode::AbsoluteX => 7,
            _ => { panic!("unimplemented address mode {:?} for DEC", mode); }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nes::cpu::test::*;

    #[test]
    fn test_dec_zero() {
        let mut cpu = mock_cpu(&[0xc6, 0x10]);
        cpu.write_at(0x10, 0xff);

        let mut result = cpu.read_at(0x10);
        assert_equalx!(result, 0xff);
        cpu.step(None);
        result = cpu.read_at(0x10);
        assert_equalx!(result, 0xfe);
        //TODO Make assertions on status registers
    }
}
