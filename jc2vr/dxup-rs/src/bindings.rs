use std::ffi::c_void;

use windows_sys::{
    core::{GUID, HRESULT, PCSTR},
    Win32::Foundation::HINSTANCE,
};

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
