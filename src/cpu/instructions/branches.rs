use std::rc::Rc;

use crate::{
    consts::{Byte, Word},
    cpu::{ScheduledTask, TaskCycleVariant, CPU},
};

fn branch(cpu: &mut CPU, condition: fn(&CPU) -> bool) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(move |cpu: &mut CPU| {
        let operand = cpu.access_memory(cpu.program_counter);
        cpu.increment_program_counter();

        cpu.set_ctx_lo(operand);
        if condition(cpu) {
            cpu.set_ctx_hi(0x1); // lsb of hi portion of instruction ctx holds information whether the condition has been met
        }

        return TaskCycleVariant::Full;
    }));

    let mut offset_cycles = offset_program_counter();
    cycles.append(&mut offset_cycles);

    cpu.schedule_instruction(cycles);
}

pub fn bcc(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_carry_flag();
    });
}

pub fn bcs(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_carry_flag();
    });
}

pub fn beq(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_zero_flag();
    });
}

pub fn bmi(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_negative_flag();
    });
}

pub fn bne(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_zero_flag();
    });
}

pub fn bpl(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_negative_flag();
    });
}

pub fn bvs(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return cpu.processor_status.get_overflow_flag();
    });
}

pub fn bvc(cpu: &mut CPU) {
    branch(cpu, |cpu: &CPU| -> bool {
        return !cpu.processor_status.get_overflow_flag();
    });
}

fn offset_program_counter() -> Vec<ScheduledTask> {
    let mut cycles: Vec<ScheduledTask> = Vec::new();

    cycles.push(Rc::new(|cpu: &mut CPU| {
        let [offset, condition_met] = match cpu.get_current_instruction_ctx() {
            Some(val) => val.to_le_bytes(),
            None => panic!("context for offseting program counter is unexpectedly not set after previous cycle"),
        };

        if condition_met == 0 {
            return TaskCycleVariant::Aborted;
        }

        let [program_counter_lo, program_counter_hi] = cpu.program_counter.to_le_bytes();
        let negative_offset_direction = 0b10000000 & offset > 0;
        let directionless_offset = if negative_offset_direction {
            (offset ^ 0b11111111) + 1
        } else {
            offset
        };
        let offset_program_counter_lo: Byte;
        let carry: bool;

        if negative_offset_direction {
            (offset_program_counter_lo, carry) =
                program_counter_lo.overflowing_sub(directionless_offset);
        } else {
            (offset_program_counter_lo, carry) =
                program_counter_lo.overflowing_add(directionless_offset);
        }

        cpu.program_counter = Word::from_le_bytes([offset_program_counter_lo, program_counter_hi]);
        cpu.set_ctx_hi(carry.into());

        return TaskCycleVariant::Full;
    }));

    cycles.push(Rc::new(|cpu: &mut CPU| {
        let [offset, carry] = match cpu.get_current_instruction_ctx() {
            Some(val) => val.to_le_bytes(),
            None => panic!("context for offseting program counter is unexpectedly not set after previous cycle"),
        };

        if carry == 0 {
            return TaskCycleVariant::Aborted;
        }

        let negative_offset_direction = 0b10000000 & offset > 0;
        let [program_counter_lo, program_counter_hi] = cpu.program_counter.to_le_bytes();
        let offset_program_counter_hi: Byte;
        if negative_offset_direction {
            offset_program_counter_hi = program_counter_hi.wrapping_sub(1);
        } else {
            offset_program_counter_hi = program_counter_hi.wrapping_add(1);
        }
        cpu.program_counter =
            Word::from_le_bytes([program_counter_lo, offset_program_counter_hi]);

        return TaskCycleVariant::Full;
    }));

    return cycles;
}

#[cfg(test)]
mod tests;
