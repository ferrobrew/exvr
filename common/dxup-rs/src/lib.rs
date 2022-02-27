use std::os::raw::c_void;
use windows_sys::core::{GUID, HRESULT, PCSTR};

pub type HINSTANCE = isize;

#[allow(non_snake_case)]
#[allow(dead_code)]
extern "system" {
    // dxgi
    fn CreateDXGIFactory(riid: *const GUID, ppfactory: *mut *mut c_void) -> HRESULT;

    // d3d9
    fn D3DPERF_SetOptions(dwoptions: u32);

    // d3d10
    fn D3D10CompileShader(
        psrcdata: PCSTR,
        srcdatasize: usize,
        pfilename: PCSTR,
        pdefines: *const c_void,
        pinclude: *mut c_void,
        pfunctionname: PCSTR,
        pprofile: PCSTR,
        flags: u32,
        ppshader: *mut c_void,
        pperrormsgs: *mut c_void,
    ) -> HRESULT;

    // d3dx10_42
    fn D3DX10CreateDevice(
        padapter: *mut c_void,
        drivertype: i32,
        software: HINSTANCE,
        flags: u32,
        ppdevice: *mut c_void,
    ) -> HRESULT;
    fn D3DX10GetFeatureLevel1(pdevice: *mut c_void, ppdevice: *mut *mut c_void) -> HRESULT;
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn DllMain(_module: HINSTANCE, reason: u32, _: *mut c_void) -> bool {
    use windows_sys::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

    D3DX10GetFeatureLevel1(std::ptr::null_mut(), std::ptr::null_mut());

    match reason {
        DLL_PROCESS_ATTACH => {
            // iat patch
        }
        _ => {}
    }
    true
}

// IAT replacements
// dxgi.dll:CreateDXGIFactory
// d3d9.dll:D3DPERF_SetOptions
// d3d10.dll:D3D10CompileShader
// d3dx10_42.dll:D3DX10CreateDevice
// d3dx10_42.dll:D3DX10GetFeatureLevel1
