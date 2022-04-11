

use alloc::string::String;
use crate::drivers::console;
use crate::{println, print};
use crate::mem;
// use crate::sched::{scheduler, Scheduler};
// use crate::process::Process;

const SHELL_NUM: usize = 3;
const SHELL_PROCESS_SIZE: usize = 100 * 1024;
#[link_section = ".bss.stack_shell"]
static mut SHELL_STACK:[u8; SHELL_PROCESS_SIZE] = [0; SHELL_PROCESS_SIZE];

static SHELL_COMMAND: [&'static str; SHELL_NUM] = [
    "cmd",
    "mem", 
    "lsof",
];
static SHELL_HANDLE:[fn(); SHELL_NUM] = [
    shell_show_commands,
    shell_command_memory, 
    shell_command_lsof,
];

pub unsafe fn shell_init() {

    // let shell = Arc::new(Process::new(
    //     "shell", 
    //     shell_entry, 
    //     SHELL_STACK.as_ptr() as usize, 
    //     SHELL_PROCESS_SIZE
    // ));

    // scheduler().lock(|sc| {
    //     println!("test");
    // } );

    // scheduler().lock(|sc| 
    //     sc.push(Arc::clone(&shell))
    // );
}

pub fn shell_entry() -> ! {

    use console::{console, Interface};

    println!(">>>>>>>> fake console <<<<<<<<");
    print!(">>> ");

    let mut input = String::new();
    loop {
        let ch = console().read_char();
        let mut flag = false;

        if ch == '\n' {
            for i in 0..SHELL_NUM {
                if input.eq(SHELL_COMMAND[i]) {
                    SHELL_HANDLE[i]();
                    flag = true;
                    break;
                }
            }
            println!();
            if !flag && input.len() > 0 {
                println!("{}: command not found", input);
            }
            input.clear();
            print!(">>> ");
        } else {
            input.push(ch);
            console().write_char(ch);
        } 

    }
}

fn shell_show_commands() {
    println!();
    for i in 0..SHELL_NUM {
        print!("{} ", SHELL_COMMAND[i]);
    }
}

fn shell_command_memory() {
    print!("{:?}", mem::allocator());
}

fn shell_command_lsof() {
    // scheduler().lock(|sc| println!("{}", sc));
}


