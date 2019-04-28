#![feature(abi_thiscall, decl_macro)]

#[cfg(not(target_os = "windows"))]
compile_error!("this only works for windows");

pub mod consts;
pub mod error;
pub mod hazedumper;
pub mod process;
pub mod sdk;

use self::prelude::*;
use log::{debug, info, trace};
use simplelog::{CombinedLogger, Config as LogConfig, LevelFilter, TermLogger};

pub mod prelude {
    pub use super::error::*;
}

fn dll_attach(thread_param: winapi::shared::minwindef::LPVOID) -> Result<()> {
    let _hinst_dll = thread_param as winapi::shared::minwindef::HINSTANCE;

    unsafe {
        // Allocate a console; if the cheat has been injected
        // twice, this will NOT fail and will simply gracefully return,
        // allowing us to continue loading.
        winapi::um::consoleapi::AllocConsole();
        winapi::um::wincon::SetConsoleTitleA("Yuki Console\0".as_ptr() as *const _);
    }

    // Make sure we know the console works.
    println!("Allocated console; making logger...");

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

    // Test logger and inform the user we will now log.
    info!("Logger has been created!");

    // Fetch a GameProcess of CS:GO.
    let mut process = self::process::GameProcess::current_process();
    debug!("this was injected by the crab gang!");
    debug!("Using HazeDumper data with offset: {}", self::hazedumper::HAZEDUMPER.timestamp);
    debug!("GameProcess::pid() => {}", process.pid());

    // Fetch the client_panorama module from CS:GO.
    let mut client = process.get_module("client_panorama.dll")?;
    trace!("got client! {:?}", client);
    // Make it an interface to the client_panorama module's main inhabitant;
    // namely consts::VCLIENT_INTERFACE_NAME.
    let client_interface = self::sdk::client::ClientInterface::new(
        client.create_interface(self::consts::VCLIENT_INTERFACE_NAME)?,
    );
    trace!("got client interface! {:?}", client_interface);

    // Fetch the engine module from CS:GO.
    let mut engine_module = process.get_module("engine.dll")?;
    trace!("got engine! {:?}", engine_module);
    // Make it an interface to the engine module's main inhabitant;
    // namely consts::ENGINE_INTERFACE_NAME.
    let engine_interface = self::sdk::engine::EngineInterface::new(
        engine_module.create_interface(self::consts::ENGINE_INTERFACE_NAME)?,
    );
    trace!("got engine interface! {:?}", engine_interface);

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
            winapi::um::wincon::FreeConsole();
        }
    }

    Ok(())
}

unsafe extern "system" fn dll_attach_wrapper(base: winapi::shared::minwindef::LPVOID) -> u32 {
    use std::panic;

    match panic::catch_unwind(|| dll_attach(base)) {
        Err(_) => {
            eprintln!("`dll_attach` has panicked");
        }
        Ok(r) => match r {
            Ok(()) => {}
            Err(e) => {
                eprintln!("`dll_attach` returned an Err: {:#?}", e);
            }
        },
    }

    match panic::catch_unwind(dll_detach) {
        Err(_) => {
            eprintln!("`dll_detach` has panicked");
        }
        Ok(r) => match r {
            Ok(()) => {}
            Err(e) => {
                eprintln!("`dll_detach` returned an Err: {:#?}", e);
            }
        },
    }

    winapi::um::libloaderapi::FreeLibraryAndExitThread(base as _, 1);

    unreachable!()
}

#[allow(unused_attributes)] // RLS yells at me during debug mode
#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinst_dll: winapi::shared::minwindef::HINSTANCE,
    fdw_reason: winapi::shared::minwindef::DWORD,
    lpv_reserved: winapi::shared::minwindef::LPVOID,
) -> i32 {
    match fdw_reason {
        winapi::um::winnt::DLL_PROCESS_ATTACH => unsafe {
            winapi::um::libloaderapi::DisableThreadLibraryCalls(hinst_dll);
            winapi::um::processthreadsapi::CreateThread(
                std::ptr::null_mut(),
                0,
                Some(dll_attach_wrapper),
                hinst_dll as _,
                0,
                std::ptr::null_mut(),
            );
        },
        winapi::um::winnt::DLL_PROCESS_DETACH => {
            if !lpv_reserved.is_null() {
                match std::panic::catch_unwind(|| dll_detach()) {
                    Err(e) => {
                        eprintln!("`dll_detach` has panicked: {:#?}", e);
                    }
                    Ok(r) => match r {
                        Ok(()) => {}
                        Err(e) => {
                            eprintln!("`dll_detach` returned an Err: {:#?}", e);
                        }
                    },
                }
            }
        }
        _ => {}
    }

    true as i32
}
