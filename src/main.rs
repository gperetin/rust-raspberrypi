//! Some docs here

// Attributes
#![feature(format_args_nl)]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod driver;
mod memory;
mod panic_wait;
mod print;
mod runtime_init;
mod synchronization;

/// Only a single core must be active and running this function
/// The init calls must appear in the correct order
unsafe fn kernel_init() -> ! {
    use driver::interface::DriverManager;

    for i in bsp::driver::driver_manager().all_device_drivers().iter() {
        if i.init().is_err() {
            panic!("Error loading driver: {}", i.compatible())
        }
    }
    bsp::driver::driver_manager().post_device_driver_init();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

/// The main function running after the early init.
fn kernel_main() -> ! {
    use console::interface::All;
    use driver::interface::DriverManager;

    // Wait for user to hit Enter.
    loop {
        if bsp::console::console().read_char() == '\n' {
            break;
        }
    }

    println!("[0] Booting on: {}", bsp::board_name());

    println!("[1] Drivers loaded:");
    for (i, driver) in bsp::driver::driver_manager()
        .all_device_drivers()
        .iter()
        .enumerate()
    {
        println!("      {}. {}", i + 1, driver.compatible());
    }

    println!(
        "[2] Chars written: {}",
        bsp::console::console().chars_written()
    );
    println!("[3] Echoing input now");

    loop {
        let c = bsp::console::console().read_char();
        bsp::console::console().write_char(c);
    }
}
