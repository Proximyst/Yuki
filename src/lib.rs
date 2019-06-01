#![feature(abi_thiscall, decl_macro, const_fn, ptr_cast)]
#![warn(rust_2018_idioms)]

use log::{debug, error, info, trace};
use winapi::{
    shared::minwindef,
    um::{consoleapi, wincon},
};

use self::prelude::*;

#[cfg(any(not(target_os = "windows"), not(target_arch = "x86")))]
compile_error!("this only works for windows for i686/x86");

pub mod consts;
pub mod error;
pub mod hazedumper;
pub mod mutmemory;
pub mod process;
pub mod sdk;

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

    use self::process::{GameProcess, Interface};
    // Fetch a GameProcess of CS:GO.
    let mut process: GameProcess = GameProcess::current_process();
    debug!(
        "Using HazeDumper data with offset: {}",
        self::hazedumper::HAZEDUMPER.timestamp,
    );
    debug!("GameProcess::pid() => {}", process.pid());
    info!(
        "Found the game process with PID: {} at 0x{:X}",
        process.pid(),
        *process.base() as usize,
    );

    use self::sdk::client::ClientInterface;
    // Fetch the client_panorama module from CS:GO.
    let mut client_module = process.get_module("client_panorama.dll")?;
    info!(
        "Found the client module at 0x{:X}",
        *client_module.handle() as usize,
    );
    // Make it an interface to the client_panorama module's main inhabitant;
    // namely consts::VCLIENT_INTERFACE_NAME.
    let client_interface =
        ClientInterface::new(client_module.create_interface(self::consts::VCLIENT_INTERFACE_NAME)?);
    trace!("got client interface! {:?}", client_interface);
    info!(
        "Found the client interface at 0x{:X}",
        *client_interface.inner().handle() as usize,
    );

    use self::sdk::engine::EngineInterface;
    // Fetch the engine module from CS:GO.
    let mut engine_module = process.get_module("engine.dll")?;
    info!(
        "Found the engine module at 0x{:X}",
        *engine_module.handle() as usize,
    );
    // Make it an interface to the engine module's main inhabitant;
    // namely consts::ENGINE_INTERFACE_NAME.
    let engine_interface =
        EngineInterface::new(engine_module.create_interface(self::consts::ENGINE_INTERFACE_NAME)?);
    trace!("got engine interface! {:?}", engine_interface);
    info!(
        "Found the engine interface at 0x{:X}",
        *engine_interface.inner().handle() as usize,
    );

    use self::sdk::panel::PanelInterface;
    let mut gui_module = process.get_module("vgui2.dll")?;
    info!(
        "Found the gui module at 0x{:X}",
        *gui_module.handle() as usize,
    );
    let panel_interface =
        PanelInterface::new(gui_module.create_interface(self::consts::PANEL_INTERFACE_NAME)?);
    trace!("got panel interface! {:?}", panel_interface);
    info!(
        "Found the panel interface at 0x{:X}",
        *panel_interface.inner().handle() as usize,
    );

    use self::sdk::entitylist::EntityListInterface;
    let entitylist_interface =
        EntityListInterface::new(client_module.create_interface(self::consts::ENTITYLIST_INTERFACE_NAME)?);
    trace!("got entity list interface! {:?}", entitylist_interface);
    info!(
        "Found the entity list interface at 0x{:X}",
        *entitylist_interface.inner().handle() as usize,
    );

    use self::sdk::clientmode::ClientModeInterface;
    let client_mode_interface: ClientModeInterface = ClientModeInterface::new(Interface::new(
        unsafe {
            **(((*(*(*client_interface.inner().handle() as *mut *mut usize)).offset(10)) + 5)
                as *mut *mut *mut usize)
        },
        &mut client_module as *mut _,
    ));
    info!(
        "Found the client mode interface at 0x{:X}",
        *client_mode_interface.inner().handle() as usize,
    );

    use self::sdk::defs::c_globalvars::CGlobalVars;
    let global_vars: &mut CGlobalVars = unsafe {
        &mut ***(((**(*client_interface.inner().handle() as *mut *mut usize)) + 27)
            as *mut *mut *mut CGlobalVars)
    };
    info!(
        "Found the global vars variable at 0x{:X}",
        global_vars as *mut _ as usize,
    );

    use self::sdk::defs::clientstate::ClientState;
    let client_state: &mut ClientState = unsafe {
        &mut ***(engine_module.pattern_scan(
            &[Some(0xA1), None, None, None, None, Some(0x8B), Some(0x80),
                None, None, None, None, Some(0xC3)]
        ).failure()?.offset(1) as *mut *mut *mut ClientState)
    };
    info!(
        "Found client state variable at 0x{:X}",
        client_state as *mut _ as usize,
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
    debug!("CGlobalVars::client => {}", global_vars.client);
    debug!("ClientState::is_paused => {}", client_state.is_paused());

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

fn dll_attach_wrapper(hinst_dll: SendableContainer<minwindef::HINSTANCE>) {
    use std::panic;

    match panic::catch_unwind(dll_attach) {
        Err(_) => {
            error!("`dll_attach` has panicked");
        }
        Ok(r) => {
            if let Some(e) = r.err() {
                error!("`dll_attach` has returned an Err: {:#?}", e);
            }
        }
    }

    match panic::catch_unwind(dll_detach) {
        Err(_) => {
            error!("`dll_detach` has panicked");
        }
        Ok(r) => {
            if let Some(e) = r.err() {
                error!("`dll_detach` has returned an Err: {:#?}", e);
            }
        }
    }

    unsafe {
        winapi::um::libloaderapi::FreeLibraryAndExitThread(hinst_dll.0, 0);
    }
}

#[allow(unused_attributes)] // RLS yells at me during debug mode
#[no_mangle]
#[export_name = "DllMain"]
pub extern "stdcall" fn dll_main(
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
            let container = SendableContainer(hinst_dll);
            thread::spawn(move || dll_attach_wrapper(container));
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

struct SendableContainer<T>(pub T);

unsafe impl<T> Send for SendableContainer<T> {}

unsafe impl<T> Sync for SendableContainer<T> {}
