use nes::cpu::{Cpu,Absolute};

pub struct Jsr { }

impl Absolute for Jsr {
    fn absolute(cpu: &mut Cpu) -> usize {
        // https://wiki.nesdev.com/w/index.php/RTS_Trick#About_JSR_and_RTS
        let addr = cpu.read_dword_and_increment();
        // PC is now at the next instruction. According to the doc above we are to
        // take this value and subtract one from it, THEN push it on the stack. On pop
        // we then add 1 to the address. I'm not sure why we just cant push the current PC
        // but there is probably a reason.
        let ret = cpu.PC - 1;

        // push the high byte and then the low byte
        cpu.push_stack(((ret & 0xFF00) >> 8) as u8);
        cpu.push_stack((ret & 0x00FF) as u8);

        cpu.PC = addr;

        6
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;

    #[test]
    fn test_jsr() {
        let mut cpu = mock_cpu(&[0x20, 0xef, 0xbe]);

        assert!(cpu.PC == 0x8000, "expected 0x8000, got {:#x}", cpu.PC);

        cpu.execute_instruction();
        assert!(cpu.PC == 0xbeef, "expected 0xbeef, got {:#x}", cpu.PC);

        let mut result = cpu.pop_stack();
        assert!(result == 0x02, "expected 0x02, got {:#x}", result);
        result = cpu.pop_stack();
        assert!(result == 0x80, "expected 0x80, got {:#x}", result);
    }
}