use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

//static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };  
}

pub fn init_idt(){
    IDT.load();
}

pub fn init(){
    interrupts.init_idt();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame){
    println!("EXCEPTION BREAKPOINT\n{:#?}", stack_frame);
}