#![feature(abi_thiscall, decl_macro, const_fn)]
#![warn(rust_2018_idioms)]

#[cfg(any(not(target_os = "windows"), not(target_arch = "x86")))]
compile_error!("this only works for windows for i686/x86");

pub mod consts;
pub mod error;
pub mod hazedumper;
pub mod mutmemory;
pub mod process;
pub mod sdk;

use self::prelude::*;
use log::{debug, info, trace};
use winapi::{
    shared::minwindef,
    um::{consoleapi, wincon},
};

pub mod prelude {
    pub use super::error::*;
}

fn dll_attach() -> Result<()> {
    unsafe {
        // Allocate a console; if the cheat has been injected
        // twice, this will NOT fail and will simply gracefully return,
        // allowing us to continue loading.
        consoleapi::AllocConsole();
        wincon::SetConsoleTitleA("Yuki Console\0".as_ptr() as *const _);
    }

    // Make sure we know the console works.
    println!("Allocated console; making logger...");

    {
        use simplelog::{CombinedLogger, Config as LogConfig, LevelFilter, TermLogger};
        // Create the logger and initialise it.
        CombinedLogger::init(vec![TermLogger::new(
            if cfg!(debug_assertions) {
                // Trace mode for debug compile
                LevelFilter::Trace
            } else {
                // Info mode for release compile
                LevelFilter::Info
            },
            LogConfig::default(),
        )
        .failure()?])?;
    }

    // Test logger and inform the user we will now log.
    info!("Logger has been created!");
    debug!("this was injected by the crab gang!");

    // Fetch a GameProcess of CS:GO.
    let mut process = self::process::GameProcess::current_process();
    debug!(
        "Using HazeDumper data with offset: {}",
        self::hazedumper::HAZEDUMPER.timestamp
    );
    debug!("GameProcess::pid() => {}", process.pid());
    info!(
        "Found the game process with PID: {} at 0x{:X}",
        process.pid(),
        *process.base() as usize
    );

    // Fetch the client_panorama module from CS:GO.
    let mut client_module = process.get_module("client_panorama.dll")?;
    info!(
        "Found the client module at 0x{:X}",
        *client_module.handle() as usize
    );
    // Make it an interface to the client_panorama module's main inhabitant;
    // namely consts::VCLIENT_INTERFACE_NAME.
    let client_interface = self::sdk::client::ClientInterface::new(
        client_module.create_interface(self::consts::VCLIENT_INTERFACE_NAME)?,
    );
    trace!("got client interface! {:?}", client_interface);
    info!(
        "Found the client interface at 0x{:X}",
        *client_interface.inner().handle() as usize
    );

    // Fetch the engine module from CS:GO.
    let mut engine_module = process.get_module("engine.dll")?;
    info!(
        "Found the engine module at 0x{:X}",
        *engine_module.handle() as usize
    );
    // Make it an interface to the engine module's main inhabitant;
    // namely consts::ENGINE_INTERFACE_NAME.
    let engine_interface = self::sdk::engine::EngineInterface::new(
        engine_module.create_interface(self::consts::ENGINE_INTERFACE_NAME)?,
    );
    trace!("got engine interface! {:?}", engine_interface);
    info!(
        "Found the engine interface at 0x{:X}",
        *engine_interface.inner().handle() as usize
    );

    let mut gui_module = process.get_module("vgui2.dll")?;
    info!(
        "Found the gui module at 0x{:X}",
        *gui_module.handle() as usize
    );
    let panel_interface = self::sdk::panel::PanelInterface::new(
        gui_module.create_interface(self::consts::PANEL_INTERFACE_NAME)?,
    );
    trace!("got panel interface! {:?}", panel_interface);
    info!(
        "Found the panel interface at 0x{:X}",
        *panel_interface.inner().handle() as usize
    );

    let client_mode_interface =
        self::sdk::clientmode::ClientModeInterface::new(self::process::Interface::new(
            unsafe {
                **(((*(*client_interface.inner().handle() as *const *const u32)).offset(10)
                    as *const u8)
                    .offset(0x5) as *const *const *const usize)
            },
            &mut client_module as *mut _,
        ));
    info!(
        "Found the client mode interface at 0x{:X}",
        *client_interface.inner().handle() as usize
    );

    let global_vars: &mut self::sdk::defs::c_globalvars::CGlobalVars = unsafe {
        &mut ***(((*(*client_interface.inner().handle() as *const *const u32)) as *const u8)
            .offset(0x1Busize as isize)
            as *const *const *mut self::sdk::defs::c_globalvars::CGlobalVars)
    };
    info!("Found the global vars variable at 0x{:X}",
        global_vars as *mut _ as usize
    );

    // Print some data which is nice to ensure the data read from CS:GO is correct.
    // This also allows us to see whether the basic functionality works.
    debug!(
        "ClientInterface::get_all_classes() => {:?}",
        client_interface.get_all_classes()?
    );
    debug!(
        "EngineInterface::get_local_player() => {:?}",
        engine_interface.get_local_player()
    );
    debug!(
        "EngineInterface::get_screen_size() => {:?} (width x height)",
        engine_interface.get_screen_size()
    );

    Ok(())
}

fn dll_detach() -> Result<()> {
    info!("Detachment has been called.");

    if !cfg!(debug_assertions) {
        unsafe {
            wincon::FreeConsole();
        }
    } else {
        // New line so that the next injection doesn't look as horrible
        println!();
    }

    Ok(())
}

fn dll_attach_wrapper() -> u32 {
    use std::panic;

    match panic::catch_unwind(dll_attach) {
        Err(_) => {
            eprintln!("`dll_attach` has panicked");
        }
        Ok(r) => {
            if let Some(e) = r.err() {
                eprintln!("`dll_detach` returned an Err: {:#?}", e);
            }
        }
    }

    match panic::catch_unwind(dll_detach) {
        Err(_) => {
            eprintln!("`dll_detach` has panicked");
        }
        Ok(r) => {
            if let Some(e) = r.err() {
                eprintln!("`dll_detach` returned an Err: {:#?}", e);
            }
        }
    }

    0
}

#[allow(unused_attributes)] // RLS yells at me during debug mode
#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinst_dll: minwindef::HINSTANCE,
    fdw_reason: minwindef::DWORD,
    lpv_reserved: minwindef::LPVOID,
) -> i32 {
    use std::{panic, thread};
    use winapi::um::{libloaderapi, winnt};

    match fdw_reason {
        winnt::DLL_PROCESS_ATTACH => {
            unsafe {
                libloaderapi::DisableThreadLibraryCalls(hinst_dll);
            }
            thread::spawn(dll_attach_wrapper);
        }
        winnt::DLL_PROCESS_DETACH => {
            if !lpv_reserved.is_null() {
                match panic::catch_unwind(dll_detach) {
                    Err(e) => {
                        eprintln!("`dll_detach` has panicked: {:#?}", e);
                    }
                    Ok(r) => {
                        if let Some(e) = r.err() {
                            eprintln!("`dll_detach` returned an Err: {:#?}", e);
                        }
                    }
                }
            }
        }
        _ => {}
    }

    true as i32
}
