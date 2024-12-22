use crate::cpu::{AddressingMode, Registers, ScheduledCycle, TaskCycleVariant, CPU};

fn decrement_cb(value: &u8) -> u8 {
    return value.wrapping_sub(1);
}

fn increment_cb(value: &u8) -> u8 {
    return value.wrapping_add(1);
}

fn decrement_memory(cpu: &mut CPU, addr_mode: AddressingMode) {
    let mut cycles = cpu.queued_modify_memory(addr_mode, Box::new(decrement_cb));

    cycles.push(Box::new(|cpu| {
        let modified_value = match cpu.get_current_instruction_ctx() {
            Some(val) => val.to_le_bytes()[1],
            None => panic!("unexpected lack of instruction ctx after memory modification"),
        };
        cpu.set_status_of_value(modified_value);

        return TaskCycleVariant::Partial;
    }));

    cpu.schedule_instruction(cycles);
}

fn decrement_register(cpu: &mut CPU, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.schedule_instruction(Vec::from([Box::new(move |cpu: &mut CPU| {
                cpu.decrement_register(register);

                return TaskCycleVariant::Full;
            }) as ScheduledCycle]));
        }
        _ => panic!("decrement_register used with incorrect register"),
    }
}

pub fn dec_zp(cpu: &mut CPU) {
    decrement_memory(cpu, AddressingMode::ZeroPage);
}

pub fn dec_zpx(cpu: &mut CPU) {
    decrement_memory(cpu, AddressingMode::ZeroPageX);
}

pub fn dec_a(cpu: &mut CPU) {
    decrement_memory(cpu, AddressingMode::Absolute);
}

pub fn dec_ax(cpu: &mut CPU) {
    decrement_memory(cpu, AddressingMode::AbsoluteX);
}

pub fn dex_im(cpu: &mut CPU) {
    decrement_register(cpu, Registers::IndexX);
}

pub fn dey_im(cpu: &mut CPU) {
    decrement_register(cpu, Registers::IndexY);
}

fn increment_memory(cpu: &mut CPU, addr_mode: AddressingMode) {
    let mut cycles = cpu.queued_modify_memory(addr_mode, Box::new(increment_cb));

    cycles.push(Box::new(|cpu| {
        let modified_value = match cpu.get_current_instruction_ctx() {
            Some(val) => val.to_le_bytes()[1],
            None => panic!("unexpected lack of instruction ctx after memory modification"),
        };
        cpu.set_status_of_value(modified_value);

        return TaskCycleVariant::Partial;
    }));

    cpu.schedule_instruction(cycles);
}

fn increment_register(cpu: &mut CPU, register: Registers) {
    match register {
        Registers::IndexX | Registers::IndexY => {
            cpu.schedule_instruction(Vec::from([Box::new(move |cpu: &mut CPU| {
                cpu.increment_register(register);

                return TaskCycleVariant::Full;
            }) as ScheduledCycle]));
        }
        _ => panic!("increment_register used with incorrect register"),
    }
}

pub fn inc_zp(cpu: &mut CPU) {
    increment_memory(cpu, AddressingMode::ZeroPage);
}

pub fn inc_zpx(cpu: &mut CPU) {
    increment_memory(cpu, AddressingMode::ZeroPageX);
}

pub fn inc_a(cpu: &mut CPU) {
    increment_memory(cpu, AddressingMode::Absolute);
}

pub fn inc_ax(cpu: &mut CPU) {
    increment_memory(cpu, AddressingMode::AbsoluteX);
}

pub fn inx_im(cpu: &mut CPU) {
    increment_register(cpu, Registers::IndexX);
}

pub fn iny_im(cpu: &mut CPU) {
    increment_register(cpu, Registers::IndexY);
}

#[cfg(test)]
mod tests;
