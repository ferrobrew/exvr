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

pub const FUNCTIONS_BY_NAME: [((&str, &str), *const ()); 5] = [
    (
        ("dxgi.dll", "CreateDXGIFactory"),
        CreateDXGIFactory as *const (),
    ),
    (
        ("d3d9.dll", "D3DPERF_SetOptions"),
        D3DPERF_SetOptions as *const (),
    ),
    (
        ("d3d10.dll", "D3D10CompileShader"),
        D3D10CompileShader as *const (),
    ),
    (
        ("d3dx10_42.dll", "D3DX10CreateDevice"),
        D3DX10CreateDevice as *const (),
    ),
    (
        ("d3dx10_42.dll", "D3DX10GetFeatureLevel1"),
        D3DX10GetFeatureLevel1 as *const (),
    ),
];
