fn main() -> anyhow::Result<()> {
    use dll_syringe::{Process, Syringe};
    use std::env::current_exe;

    let cur_path = current_exe()?.parent().expect("no path? wat").to_owned();
    let target_process = Process::find_first_by_name("ffxiv_dx11.exe").unwrap();
    let syringe = Syringe::new();
    syringe.inject(&target_process, &cur_path.join("cimgui.dll"))?;
    syringe.inject(&target_process, &cur_path.join("xivr_native.dll"))?;

    Ok(())
}
