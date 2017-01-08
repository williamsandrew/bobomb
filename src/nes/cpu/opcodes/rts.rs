use nes::cpu::{Cpu,FromImplied,Address};

pub struct Rts { }

impl FromImplied for Rts {
    fn from_implied(cpu: &mut Cpu) -> usize {
        cpu.debug_stack();
        let lo = cpu.pop_stack();
        let hi = cpu.pop_stack();
        let addr = Address::new(hi, lo);
        cpu.debug_stack();

        cpu.PC = addr.to_u16().wrapping_add(1);

        6
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_rts() {
        let mut cpu = mock_cpu(&[0x60]);
        cpu.push_stack(0xBE);
        cpu.push_stack(0xEE);

        cpu.execute_instruction();
        assert!(cpu.PC == 0xbeef, "expected 0xbeef, got {:#x}", cpu.PC);
    }
}
