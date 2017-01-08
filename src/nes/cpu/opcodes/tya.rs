use nes::cpu::{Cpu,Implied};

pub struct Tya { }

impl Implied for Tya {
    fn implied(cpu: &mut Cpu) -> usize {
        cpu.AC = cpu.Y;

        2
    }
}

#[cfg(test)]
mod test {
    use nes::cpu::test::*;
    use nes::cpu::Registers;

    #[test]
    fn test_tya() {
        let mut cpu = mock_cpu(&[0x98]);
        cpu.Y = 0xf0;
        cpu.AC = 0x00;

        assert_cpu_register!(cpu, Registers::Y, 0xf0);
        assert_cpu_register!(cpu, Registers::AC, 0x00);
        cpu.execute_instruction();
        assert_cpu_register!(cpu, Registers::AC, 0xf0);
        //TODO Make assertions on status registers
    }
}