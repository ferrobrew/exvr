use std::os::windows::prelude::FromRawHandle;

use anyhow::Context;
use dll_syringe::process::{OwnedProcess, Process};
use windows::Win32::{Foundation::HANDLE, System::Threading};

const APP_ID: u32 = 8190;
const PROCESS_NAME: &str = "JustCause2.exe";
const DXUP_NAME: &str = "dxup_rs.dll";

fn spawn_process() -> anyhow::Result<(OwnedProcess, HANDLE)> {
    let startup_info = Threading::STARTUPINFOW::default();
    let mut process_info = Threading::PROCESS_INFORMATION::default();

    let game_path = steamlocate::SteamDir::locate()
        .context("failed to locate steamdir")?
        .app(&APP_ID)
        .context("failed to locate app")?
        .path
        .clone();

    let game_path_str = game_path.clone().into_os_string();
    let executable_path_str = game_path.join(PROCESS_NAME).into_os_string();

    let environment: Vec<u16> = std::env::vars()
        .chain(
            ["SteamGameId", "SteamAppId"]
                .iter()
                .map(|s| (s.to_string(), APP_ID.to_string())),
        )
        .fold(String::new(), |a, (k, v)| format!("{}{}={}\0", a, k, v))
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        Threading::CreateProcessW(
            executable_path_str,
            Default::default(),
            std::ptr::null(),
            std::ptr::null(),
            false,
            Threading::CREATE_UNICODE_ENVIRONMENT | Threading::CREATE_SUSPENDED,
            environment.as_ptr() as _,
            game_path_str,
            &startup_info,
            &mut process_info,
        )
        .as_bool()
        .then(|| {
            (
                OwnedProcess::from_raw_handle(process_info.hProcess.0 as _),
                process_info.hThread,
            )
        })
        .context("failed to spawn process")
    }
}

fn inject(process: &OwnedProcess) -> anyhow::Result<()> {
    let syringe = dll_syringe::Syringe::for_process(process.try_clone()?);
    let payload_path = std::env::current_exe()?
        .parent()
        .context("unable to find parent path")?
        .join(DXUP_NAME);
    syringe.inject(payload_path)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let (process, main_thread) = spawn_process()?;
    match inject(&process) {
        Ok(_) => unsafe {
            Threading::ResumeThread(main_thread);
            Ok(())
        },
        Err(err) => {
            process.kill()?;
            Err(err)
        }
    }
}
