#![no_std]
#![no_main]

use core::sync::atomic::{self, Ordering};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static GDT: [u64; 3] = [
 0x0000000000000000, // null descriptor
 0x00AF9A000000FFFF, // kernel code segment descriptor
 0x00CF93000000FFFF, // kernel data segment descriptor
];

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[no_mangle]
pub extern “C” fn _start() -> ! {
 // load GDT
 unsafe {
 x86_64::instructions::gdt::load_gdt(&GDT);
 }

 // initialize IDT
IDT.divide_by_zero.set_handler_fn(divide_by_zero_handler);
IDT.debug.set_handler_fn(debug_handler); IDT.non_maskable_interrupt.set_handler_fn(nmi_handler);
IDT.breakpoint.set_handler_fn(breakpoint_handler);
IDT.overflow.set_handler_fn(overflow_handler); IDT.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
IDT.invalid_opcode.set_handler_fn(invalid_opcode_handler); IDT.device_not_available.set_handler_fn(device_not_available_handler);
IDT.double_fault.set_handler_fn(double_fault_handler); IDT.invalid_tss.set_handler_fn(invalid_tss_handler); IDT.segment_not_present.set_handler_fn(segment_not_present_handler);
IDT.stack_segment_fault.set_handler_fn(stack_segment_fault_handler); IDT.general_protection_fault.set_handler_fn(general_protection_fault_handler);
IDT.page_fault.set_handler_fn(page_fault_handler);
IDT.x87_floating_point.set_handler_fn(x87_floating_point_handler);
IDT.alignment_check.set_handler_fn(alignment_check_handler); IDT.machine_check.set_handler_fn(machine_check_handler);
IDT.simd_floating_point.set_handler_fn(simd_floating_point_handler);
IDT.virtualization.set_handler_fn(virtualization);
