#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::debugger::Debugger;
use crate::debugger::d3d_payload::D3DPayload;
use crate::hooks::Patcher;
use crate::ct_config::*;

use windows::Win32::Graphics::Direct3D11::{
    D3D_PRIMITIVE_TOPOLOGY, D3D11_MAPPED_SUBRESOURCE, D3D11_MAP, D3D11_VIEWPORT, D3D11_BOX,
    D3D11_DEVICE_CONTEXT_TYPE, D3D11_TILED_RESOURCE_COORDINATE, D3D11_TILE_REGION_SIZE,
    D3D11_CONTEXT_TYPE
};
use windows::Win32::Graphics::Dxgi::{DXGI_FORMAT};
use windows::Win32::Foundation::{BOOL, RECT, HANDLE, PWSTR};

use windows::runtime::*;
use std::os::raw::c_void;

struct ID3D11DeviceContext {
    vtbl: *const ID3D11DeviceContextVtbl,
}

struct ID3D11DeviceContextVtbl {
    QueryInterface: unsafe extern "C" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT,
    AddRef: unsafe extern "C" fn(*mut c_void) -> u32,
    Release: unsafe extern "C" fn(*mut c_void) -> u32,
    GetDevice: unsafe extern "C" fn(*mut c_void, *mut *mut c_void),
    GetPrivateData: unsafe extern "C" fn(*mut c_void, *const GUID, *mut u32, *mut c_void) -> HRESULT,
    SetPrivateData: unsafe extern "C" fn(*mut c_void, *const GUID, u32, *mut c_void) -> HRESULT,
    SetPrivateDataInterface: unsafe extern "C" fn(*mut c_void, *const GUID, *mut IUnknown) -> HRESULT,
    VSSetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    PSSetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    PSSetShader: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *const c_void, u32),
    PSSetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    VSSetShader: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *const c_void, u32),
    DrawIndexed: unsafe extern "C" fn(*mut c_void, u32, u32, i32),
    Draw: unsafe extern "C" fn(*mut c_void, u32, u32),
    Map: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, D3D11_MAP, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT,
    Unmap: unsafe extern "C" fn(*mut c_void, *mut c_void, u32),
    PSSetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    IASetInputLayout: unsafe extern "C" fn(*mut c_void, *mut c_void),
    IASetVertexBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void, *mut u32, *mut u32),
    IASetIndexBuffer: unsafe extern "C" fn(*mut c_void, *mut c_void, DXGI_FORMAT, u32),
    DrawIndexedInstanced: unsafe extern "C" fn(*mut c_void, u32, u32, u32, i32, u32),
    DrawInstanced: unsafe extern "C" fn(*mut c_void, u32, u32, u32, u32),
    GSSetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    GSSetShader: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *const c_void, u32),
    IASetPrimitiveTopology: unsafe extern "C" fn(*mut c_void, D3D_PRIMITIVE_TOPOLOGY),
    VSSetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    VSSetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    Begin: unsafe extern "C" fn(*mut c_void, *mut c_void),
    End: unsafe extern "C" fn(*mut c_void, *mut c_void),
    GetData: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, u32, u32) -> HRESULT,
    SetPredication: unsafe extern "C" fn(*mut c_void, *mut c_void, BOOL),
    GSSetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    GSSetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    OMSetRenderTargets: unsafe extern "C" fn(*mut c_void, u32, *mut *const c_void, *mut c_void),
    OMSetRenderTargetsAndUnorderedAccessViews: unsafe extern "C" fn(*mut c_void, u32, *mut *const c_void, *mut c_void, u32, u32, *mut *const c_void, *mut u32),
    OMSetBlendState: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut f32, u32),
    OMSetDepthStencilState: unsafe extern "C" fn(*mut c_void, *mut c_void, u32),
    SOSetTargets: unsafe extern "C" fn(*mut c_void, u32, *mut *const c_void, *mut u32),
    DrawAuto: unsafe extern "C" fn(*mut c_void),
    DrawIndexedInstancedIndirect: unsafe extern "C" fn(*mut c_void, *mut c_void, u32),
    DrawInstancedIndirect: unsafe extern "C" fn(*mut c_void, *mut c_void, u32),
    Dispatch: unsafe extern "C" fn(*mut c_void, u32, u32, u32),
    DispatchIndirect: unsafe extern "C" fn(*mut c_void, *mut c_void, u32),
    RSSetState: unsafe extern "C" fn(*mut c_void, *mut c_void),
    RSSetViewports: unsafe extern "C" fn(*mut c_void, u32, *mut D3D11_VIEWPORT),
    RSSetScissorRects: unsafe extern "C" fn(*mut c_void, u32, *mut RECT),
    CopySubresourceRegion: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, u32, u32, u32, *mut c_void, u32, *mut D3D11_BOX),
    CopyResource: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
    UpdateSubresource: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut D3D11_BOX, *mut c_void, u32, u32),
    CopyStructureCount: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_void),
    ClearRenderTargetView: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut f32),
    ClearUnorderedAccessViewUint: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut u32),
    ClearUnorderedAccessViewFloat: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut f32),
    ClearDepthStencilView: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, f32, u8),
    GenerateMips: unsafe extern "C" fn(*mut c_void, *mut c_void),
    SetResourceMinLOD: unsafe extern "C" fn(*mut c_void, *mut c_void, f32),
    GetResourceMinLOD: unsafe extern "C" fn(*mut c_void, *mut c_void) -> f32,
    ResolveSubresource: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_void, u32, DXGI_FORMAT),
    ExecuteCommandList: unsafe extern "C" fn(*mut c_void, *mut c_void, BOOL),
    HSSetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    HSSetShader: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *const c_void, u32),
    HSSetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    HSSetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    DSSetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    DSSetShader: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *const c_void, u32),
    DSSetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    DSSetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    CSSetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    CSSetUnorderedAccessViews: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void, *mut u32),
    CSSetShader: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *const c_void, u32),
    CSSetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    CSSetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void),
    VSGetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    PSGetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    PSGetShader: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut *mut c_void, *mut u32),
    PSGetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    VSGetShader: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut *mut c_void, *mut u32),
    PSGetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    IAGetInputLayout: unsafe extern "C" fn(*mut c_void, *mut *mut c_void),
    IAGetVertexBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    IAGetIndexBuffer: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut DXGI_FORMAT, *mut u32),
    GSGetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    GSGetShader: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut *mut c_void, *mut u32),
    IAGetPrimitiveTopology: unsafe extern "C" fn(*mut c_void, *mut D3D_PRIMITIVE_TOPOLOGY),
    VSGetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    VSGetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    GetPredication: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut BOOL),
    GSGetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    GSGetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    OMGetRenderTargets: unsafe extern "C" fn(*mut c_void, u32, *mut *mut c_void, *mut *mut c_void),
    OMGetRenderTargetsAndUnorderedAccessViews: unsafe extern "C" fn(*mut c_void, u32, *mut *mut c_void, *mut *mut c_void, u32, u32, *mut *mut c_void),
    OMGetBlendState: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut f32, *mut u32),
    OMGetDepthStencilState: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut u32),
    SOGetTargets: unsafe extern "C" fn(*mut c_void, u32, *mut *mut c_void),
    RSGetState: unsafe extern "C" fn(*mut c_void, *mut *mut c_void),
    RSGetViewports: unsafe extern "C" fn(*mut c_void, *mut u32, *mut D3D11_VIEWPORT),
    RSGetScissorRects: unsafe extern "C" fn(*mut c_void, *mut u32, *mut RECT),
    HSGetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    HSGetShader: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut *mut c_void, *mut u32),
    HSGetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    HSGetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    DSGetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    DSGetShader: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut *mut c_void, *mut u32),
    DSGetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    DSGetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    CSGetShaderResources: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    CSGetUnorderedAccessViews: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    CSGetShader: unsafe extern "C" fn(*mut c_void, *mut *mut c_void, *mut *mut c_void, *mut u32),
    CSGetSamplers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    CSGetConstantBuffers: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void),
    ClearState: unsafe extern "C" fn(*mut c_void),
    Flush: unsafe extern "C" fn(*mut c_void),
    GetType: unsafe extern "C" fn(*mut c_void) -> D3D11_DEVICE_CONTEXT_TYPE,
    GetContextFlags: unsafe extern "C" fn(*mut c_void) -> u32,
    FinishCommandList: unsafe extern "C" fn(*mut c_void, BOOL, *mut *mut c_void) -> HRESULT,
    CopySubresourceRegion1: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, u32, u32, u32, *mut c_void, u32, *mut D3D11_BOX, u32),
    UpdateSubresource1: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut D3D11_BOX, *mut c_void, u32, u32, u32),
    DiscardResource: unsafe extern "C" fn(*mut c_void, *mut c_void),
    DiscardView: unsafe extern "C" fn(*mut c_void, *mut c_void),
    VSSetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void, *mut u32, *mut u32),
    HSSetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void, *mut u32, *mut u32),
    DSSetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void, *mut u32, *mut u32),
    GSSetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void, *mut u32, *mut u32),
    PSSetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void, *mut u32, *mut u32),
    CSSetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *const c_void, *mut u32, *mut u32),
    VSGetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    HSGetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    DSGetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    GSGetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    PSGetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    CSGetConstantBuffers1: unsafe extern "C" fn(*mut c_void, u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    SwapDeviceContextState: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *mut c_void),
    ClearView: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut f32, *mut RECT, u32),
    DiscardView1: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut RECT, u32),
    UpdateTileMappings: unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut D3D11_TILED_RESOURCE_COORDINATE, *mut D3D11_TILE_REGION_SIZE, *mut c_void, u32, *mut u32, *mut u32, *mut u32, u32) -> HRESULT,
    CopyTileMappings: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut D3D11_TILED_RESOURCE_COORDINATE, *mut c_void, *mut D3D11_TILED_RESOURCE_COORDINATE, *mut D3D11_TILE_REGION_SIZE, u32) -> HRESULT,
    CopyTiles: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut D3D11_TILED_RESOURCE_COORDINATE, *mut D3D11_TILE_REGION_SIZE, *mut c_void, u64, u32),
    UpdateTiles: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut D3D11_TILED_RESOURCE_COORDINATE, *mut D3D11_TILE_REGION_SIZE, *mut c_void, u32),
    ResizeTilePool: unsafe extern "C" fn(*mut c_void, *mut c_void, u64) -> HRESULT,
    TiledResourceBarrier: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void),
    IsAnnotationEnabled: unsafe extern "C" fn(*mut c_void) -> BOOL,
    SetMarkerInt: unsafe extern "C" fn(*mut c_void, *const PWSTR, i32),
    BeginEventInt: unsafe extern "C" fn(*mut c_void, *const PWSTR, i32),
    EndEvent: unsafe extern "C" fn(*mut c_void),
    Flush1: unsafe extern "C" fn(*mut c_void, D3D11_CONTEXT_TYPE, HANDLE),
    SetHardwareProtectionState: unsafe extern "C" fn(*mut c_void, BOOL),
    GetHardwareProtectionState: unsafe extern "C" fn(*mut c_void, *mut BOOL),
}

static mut ORIGINAL_VTABLE: Option<*const ID3D11DeviceContextVtbl> = None;
static HOOKED_VTABLE: ID3D11DeviceContextVtbl = ID3D11DeviceContextVtbl {
    QueryInterface: QueryInterface_hook,
    AddRef: AddRef_hook,
    Release: Release_hook,
    GetDevice: GetDevice_hook,
    GetPrivateData: GetPrivateData_hook,
    SetPrivateData: SetPrivateData_hook,
    SetPrivateDataInterface: SetPrivateDataInterface_hook,
    VSSetConstantBuffers: VSSetConstantBuffers_hook,
    PSSetShaderResources: PSSetShaderResources_hook,
    PSSetShader: PSSetShader_hook,
    PSSetSamplers: PSSetSamplers_hook,
    VSSetShader: VSSetShader_hook,
    DrawIndexed: DrawIndexed_hook,
    Draw: Draw_hook,
    Map: Map_hook,
    Unmap: Unmap_hook,
    PSSetConstantBuffers: PSSetConstantBuffers_hook,
    IASetInputLayout: IASetInputLayout_hook,
    IASetVertexBuffers: IASetVertexBuffers_hook,
    IASetIndexBuffer: IASetIndexBuffer_hook,
    DrawIndexedInstanced: DrawIndexedInstanced_hook,
    DrawInstanced: DrawInstanced_hook,
    GSSetConstantBuffers: GSSetConstantBuffers_hook,
    GSSetShader: GSSetShader_hook,
    IASetPrimitiveTopology: IASetPrimitiveTopology_hook,
    VSSetShaderResources: VSSetShaderResources_hook,
    VSSetSamplers: VSSetSamplers_hook,
    Begin: Begin_hook,
    End: End_hook,
    GetData: GetData_hook,
    SetPredication: SetPredication_hook,
    GSSetShaderResources: GSSetShaderResources_hook,
    GSSetSamplers: GSSetSamplers_hook,
    OMSetRenderTargets: OMSetRenderTargets_hook,
    OMSetRenderTargetsAndUnorderedAccessViews: OMSetRenderTargetsAndUnorderedAccessViews_hook,
    OMSetBlendState: OMSetBlendState_hook,
    OMSetDepthStencilState: OMSetDepthStencilState_hook,
    SOSetTargets: SOSetTargets_hook,
    DrawAuto: DrawAuto_hook,
    DrawIndexedInstancedIndirect: DrawIndexedInstancedIndirect_hook,
    DrawInstancedIndirect: DrawInstancedIndirect_hook,
    Dispatch: Dispatch_hook,
    DispatchIndirect: DispatchIndirect_hook,
    RSSetState: RSSetState_hook,
    RSSetViewports: RSSetViewports_hook,
    RSSetScissorRects: RSSetScissorRects_hook,
    CopySubresourceRegion: CopySubresourceRegion_hook,
    CopyResource: CopyResource_hook,
    UpdateSubresource: UpdateSubresource_hook,
    CopyStructureCount: CopyStructureCount_hook,
    ClearRenderTargetView: ClearRenderTargetView_hook,
    ClearUnorderedAccessViewUint: ClearUnorderedAccessViewUint_hook,
    ClearUnorderedAccessViewFloat: ClearUnorderedAccessViewFloat_hook,
    ClearDepthStencilView: ClearDepthStencilView_hook,
    GenerateMips: GenerateMips_hook,
    SetResourceMinLOD: SetResourceMinLOD_hook,
    GetResourceMinLOD: GetResourceMinLOD_hook,
    ResolveSubresource: ResolveSubresource_hook,
    ExecuteCommandList: ExecuteCommandList_hook,
    HSSetShaderResources: HSSetShaderResources_hook,
    HSSetShader: HSSetShader_hook,
    HSSetSamplers: HSSetSamplers_hook,
    HSSetConstantBuffers: HSSetConstantBuffers_hook,
    DSSetShaderResources: DSSetShaderResources_hook,
    DSSetShader: DSSetShader_hook,
    DSSetSamplers: DSSetSamplers_hook,
    DSSetConstantBuffers: DSSetConstantBuffers_hook,
    CSSetShaderResources: CSSetShaderResources_hook,
    CSSetUnorderedAccessViews: CSSetUnorderedAccessViews_hook,
    CSSetShader: CSSetShader_hook,
    CSSetSamplers: CSSetSamplers_hook,
    CSSetConstantBuffers: CSSetConstantBuffers_hook,
    VSGetConstantBuffers: VSGetConstantBuffers_hook,
    PSGetShaderResources: PSGetShaderResources_hook,
    PSGetShader: PSGetShader_hook,
    PSGetSamplers: PSGetSamplers_hook,
    VSGetShader: VSGetShader_hook,
    PSGetConstantBuffers: PSGetConstantBuffers_hook,
    IAGetInputLayout: IAGetInputLayout_hook,
    IAGetVertexBuffers: IAGetVertexBuffers_hook,
    IAGetIndexBuffer: IAGetIndexBuffer_hook,
    GSGetConstantBuffers: GSGetConstantBuffers_hook,
    GSGetShader: GSGetShader_hook,
    IAGetPrimitiveTopology: IAGetPrimitiveTopology_hook,
    VSGetShaderResources: VSGetShaderResources_hook,
    VSGetSamplers: VSGetSamplers_hook,
    GetPredication: GetPredication_hook,
    GSGetShaderResources: GSGetShaderResources_hook,
    GSGetSamplers: GSGetSamplers_hook,
    OMGetRenderTargets: OMGetRenderTargets_hook,
    OMGetRenderTargetsAndUnorderedAccessViews: OMGetRenderTargetsAndUnorderedAccessViews_hook,
    OMGetBlendState: OMGetBlendState_hook,
    OMGetDepthStencilState: OMGetDepthStencilState_hook,
    SOGetTargets: SOGetTargets_hook,
    RSGetState: RSGetState_hook,
    RSGetViewports: RSGetViewports_hook,
    RSGetScissorRects: RSGetScissorRects_hook,
    HSGetShaderResources: HSGetShaderResources_hook,
    HSGetShader: HSGetShader_hook,
    HSGetSamplers: HSGetSamplers_hook,
    HSGetConstantBuffers: HSGetConstantBuffers_hook,
    DSGetShaderResources: DSGetShaderResources_hook,
    DSGetShader: DSGetShader_hook,
    DSGetSamplers: DSGetSamplers_hook,
    DSGetConstantBuffers: DSGetConstantBuffers_hook,
    CSGetShaderResources: CSGetShaderResources_hook,
    CSGetUnorderedAccessViews: CSGetUnorderedAccessViews_hook,
    CSGetShader: CSGetShader_hook,
    CSGetSamplers: CSGetSamplers_hook,
    CSGetConstantBuffers: CSGetConstantBuffers_hook,
    ClearState: ClearState_hook,
    Flush: Flush_hook,
    GetType: GetType_hook,
    GetContextFlags: GetContextFlags_hook,
    FinishCommandList: FinishCommandList_hook,
    CopySubresourceRegion1: CopySubresourceRegion1_hook,
    UpdateSubresource1: UpdateSubresource1_hook,
    DiscardResource: DiscardResource_hook,
    DiscardView: DiscardView_hook,
    VSSetConstantBuffers1: VSSetConstantBuffers1_hook,
    HSSetConstantBuffers1: HSSetConstantBuffers1_hook,
    DSSetConstantBuffers1: DSSetConstantBuffers1_hook,
    GSSetConstantBuffers1: GSSetConstantBuffers1_hook,
    PSSetConstantBuffers1: PSSetConstantBuffers1_hook,
    CSSetConstantBuffers1: CSSetConstantBuffers1_hook,
    VSGetConstantBuffers1: VSGetConstantBuffers1_hook,
    HSGetConstantBuffers1: HSGetConstantBuffers1_hook,
    DSGetConstantBuffers1: DSGetConstantBuffers1_hook,
    GSGetConstantBuffers1: GSGetConstantBuffers1_hook,
    PSGetConstantBuffers1: PSGetConstantBuffers1_hook,
    CSGetConstantBuffers1: CSGetConstantBuffers1_hook,
    SwapDeviceContextState: SwapDeviceContextState_hook,
    ClearView: ClearView_hook,
    DiscardView1: DiscardView1_hook,
    UpdateTileMappings: UpdateTileMappings_hook,
    CopyTileMappings: CopyTileMappings_hook,
    CopyTiles: CopyTiles_hook,
    UpdateTiles: UpdateTiles_hook,
    ResizeTilePool: ResizeTilePool_hook,
    TiledResourceBarrier: TiledResourceBarrier_hook,
    IsAnnotationEnabled: IsAnnotationEnabled_hook,
    SetMarkerInt: SetMarkerInt_hook,
    BeginEventInt: BeginEventInt_hook,
    EndEvent: EndEvent_hook,
    Flush1: Flush1_hook,
    SetHardwareProtectionState: SetHardwareProtectionState_hook,
    GetHardwareProtectionState: GetHardwareProtectionState_hook,
};

fn push_back_payload(payload: D3DPayload) {
    if let Some(debugger) = Debugger::get_mut() {
        let mut command_stream = debugger.command_stream.lock().unwrap();
        command_stream.add_d3d_command(payload).unwrap();
    }
}
unsafe extern "C" fn QueryInterface_hook(This: *mut c_void, riid: *const GUID, ppvObject: *mut *mut c_void) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).QueryInterface)(This, riid, ppvObject);
    push_back_payload(D3DPayload::QueryInterface(riid, ppvObject));
    ret
}
unsafe extern "C" fn AddRef_hook(This: *mut c_void) -> u32 {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).AddRef)(This, );
    push_back_payload(D3DPayload::AddRef());
    ret
}
unsafe extern "C" fn Release_hook(This: *mut c_void) -> u32 {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).Release)(This, );
    push_back_payload(D3DPayload::Release());
    ret
}
unsafe extern "C" fn GetDevice_hook(This: *mut c_void, ppDevice: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GetDevice)(This, ppDevice);
    push_back_payload(D3DPayload::GetDevice(ppDevice));
    ret
}
unsafe extern "C" fn GetPrivateData_hook(This: *mut c_void, guid: *const GUID, pDataSize: *mut u32, pData: *mut c_void) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GetPrivateData)(This, guid, pDataSize, pData);
    push_back_payload(D3DPayload::GetPrivateData(guid, pDataSize, pData));
    ret
}
unsafe extern "C" fn SetPrivateData_hook(This: *mut c_void, guid: *const GUID, DataSize: u32, pData: *mut c_void) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SetPrivateData)(This, guid, DataSize, pData);
    push_back_payload(D3DPayload::SetPrivateData(guid, DataSize, pData));
    ret
}
unsafe extern "C" fn SetPrivateDataInterface_hook(This: *mut c_void, guid: *const GUID, pData: *mut IUnknown) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SetPrivateDataInterface)(This, guid, pData);
    push_back_payload(D3DPayload::SetPrivateDataInterface(guid, pData));
    ret
}
unsafe extern "C" fn VSSetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSSetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::VSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn PSSetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSSetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::PSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn PSSetShader_hook(This: *mut c_void, pPixelShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSSetShader)(This, pPixelShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::PSSetShader(pPixelShader, ppClassInstances, NumClassInstances));
    ret
}
unsafe extern "C" fn PSSetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSSetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::PSSetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn VSSetShader_hook(This: *mut c_void, pVertexShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSSetShader)(This, pVertexShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::VSSetShader(pVertexShader, ppClassInstances, NumClassInstances));
    ret
}
unsafe extern "C" fn DrawIndexed_hook(This: *mut c_void, IndexCount: u32, StartIndexLocation: u32, BaseVertexLocation: i32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DrawIndexed)(This, IndexCount, StartIndexLocation, BaseVertexLocation);
    push_back_payload(D3DPayload::DrawIndexed(IndexCount, StartIndexLocation, BaseVertexLocation));
    ret
}
unsafe extern "C" fn Draw_hook(This: *mut c_void, VertexCount: u32, StartVertexLocation: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).Draw)(This, VertexCount, StartVertexLocation);
    push_back_payload(D3DPayload::Draw(VertexCount, StartVertexLocation));
    ret
}
unsafe extern "C" fn Map_hook(This: *mut c_void, pResource: *mut c_void, Subresource: u32, MapType: D3D11_MAP, MapFlags: u32, pMappedResource: *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).Map)(This, pResource, Subresource, MapType, MapFlags, pMappedResource);
    push_back_payload(D3DPayload::Map(pResource, Subresource, MapType, MapFlags, pMappedResource));
    ret
}
unsafe extern "C" fn Unmap_hook(This: *mut c_void, pResource: *mut c_void, Subresource: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).Unmap)(This, pResource, Subresource);
    push_back_payload(D3DPayload::Unmap(pResource, Subresource));
    ret
}
unsafe extern "C" fn PSSetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSSetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::PSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn IASetInputLayout_hook(This: *mut c_void, pInputLayout: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IASetInputLayout)(This, pInputLayout);
    push_back_payload(D3DPayload::IASetInputLayout(pInputLayout));
    ret
}
unsafe extern "C" fn IASetVertexBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppVertexBuffers: *mut *const c_void, pStrides: *mut u32, pOffsets: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IASetVertexBuffers)(This, StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets);
    push_back_payload(D3DPayload::IASetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets));
    ret
}
unsafe extern "C" fn IASetIndexBuffer_hook(This: *mut c_void, pIndexBuffer: *mut c_void, Format: DXGI_FORMAT, Offset: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IASetIndexBuffer)(This, pIndexBuffer, Format, Offset);
    push_back_payload(D3DPayload::IASetIndexBuffer(pIndexBuffer, Format, Offset));
    ret
}
unsafe extern "C" fn DrawIndexedInstanced_hook(This: *mut c_void, IndexCountPerInstance: u32, InstanceCount: u32, StartIndexLocation: u32, BaseVertexLocation: i32, StartInstanceLocation: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DrawIndexedInstanced)(This, IndexCountPerInstance, InstanceCount, StartIndexLocation, BaseVertexLocation, StartInstanceLocation);
    push_back_payload(D3DPayload::DrawIndexedInstanced(IndexCountPerInstance, InstanceCount, StartIndexLocation, BaseVertexLocation, StartInstanceLocation));
    ret
}
unsafe extern "C" fn DrawInstanced_hook(This: *mut c_void, VertexCountPerInstance: u32, InstanceCount: u32, StartVertexLocation: u32, StartInstanceLocation: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DrawInstanced)(This, VertexCountPerInstance, InstanceCount, StartVertexLocation, StartInstanceLocation);
    push_back_payload(D3DPayload::DrawInstanced(VertexCountPerInstance, InstanceCount, StartVertexLocation, StartInstanceLocation));
    ret
}
unsafe extern "C" fn GSSetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSSetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::GSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn GSSetShader_hook(This: *mut c_void, pShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSSetShader)(This, pShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::GSSetShader(pShader, ppClassInstances, NumClassInstances));
    ret
}
unsafe extern "C" fn IASetPrimitiveTopology_hook(This: *mut c_void, Topology: D3D_PRIMITIVE_TOPOLOGY) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IASetPrimitiveTopology)(This, Topology);
    push_back_payload(D3DPayload::IASetPrimitiveTopology(Topology));
    ret
}
unsafe extern "C" fn VSSetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSSetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::VSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn VSSetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSSetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::VSSetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn Begin_hook(This: *mut c_void, pAsync: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).Begin)(This, pAsync);
    push_back_payload(D3DPayload::Begin(pAsync));
    ret
}
unsafe extern "C" fn End_hook(This: *mut c_void, pAsync: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).End)(This, pAsync);
    push_back_payload(D3DPayload::End(pAsync));
    ret
}
unsafe extern "C" fn GetData_hook(This: *mut c_void, pAsync: *mut c_void, pData: *mut c_void, DataSize: u32, GetDataFlags: u32) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GetData)(This, pAsync, pData, DataSize, GetDataFlags);
    push_back_payload(D3DPayload::GetData(pAsync, pData, DataSize, GetDataFlags));
    ret
}
unsafe extern "C" fn SetPredication_hook(This: *mut c_void, pPredicate: *mut c_void, PredicateValue: BOOL) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SetPredication)(This, pPredicate, PredicateValue);
    push_back_payload(D3DPayload::SetPredication(pPredicate, PredicateValue));
    ret
}
unsafe extern "C" fn GSSetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSSetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::GSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn GSSetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSSetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::GSSetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn OMSetRenderTargets_hook(This: *mut c_void, NumViews: u32, ppRenderTargetViews: *mut *const c_void, pDepthStencilView: *mut c_void) {
    use windows::Win32::Graphics::Direct3D11 as d3d;
    let ret = ((*ORIGINAL_VTABLE.unwrap()).OMSetRenderTargets)(This, NumViews, ppRenderTargetViews, pDepthStencilView);

    let resources = {
        let ppRenderTargetViews = ppRenderTargetViews as *const Option<d3d::ID3D11RenderTargetView>;
        if ppRenderTargetViews.is_null() {
            vec![]
        } else {
            let rtvs = std::slice::from_raw_parts(ppRenderTargetViews, NumViews as usize);
            rtvs.iter().filter_map(|rtv| {
                match rtv {
                    Some(rtv) => {
                        let mut resource = None;
                        rtv.GetResource(&mut resource);
                        resource
                    }
                    None => None,
                }
            }).collect()
        }
    };
    push_back_payload(D3DPayload::OMSetRenderTargets(NumViews, ppRenderTargetViews, pDepthStencilView, resources));
    ret
}
unsafe extern "C" fn OMSetRenderTargetsAndUnorderedAccessViews_hook(This: *mut c_void, NumRTVs: u32, ppRenderTargetViews: *mut *const c_void, pDepthStencilView: *mut c_void, UAVStartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *const c_void, pUAVInitialCounts: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).OMSetRenderTargetsAndUnorderedAccessViews)(This, NumRTVs, ppRenderTargetViews, pDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts);
    push_back_payload(D3DPayload::OMSetRenderTargetsAndUnorderedAccessViews(NumRTVs, ppRenderTargetViews, pDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts));
    ret
}
unsafe extern "C" fn OMSetBlendState_hook(This: *mut c_void, pBlendState: *mut c_void, BlendFactor: *mut f32, SampleMask: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).OMSetBlendState)(This, pBlendState, BlendFactor, SampleMask);
    push_back_payload(D3DPayload::OMSetBlendState(pBlendState, BlendFactor, SampleMask));
    ret
}
unsafe extern "C" fn OMSetDepthStencilState_hook(This: *mut c_void, pDepthStencilState: *mut c_void, StencilRef: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).OMSetDepthStencilState)(This, pDepthStencilState, StencilRef);
    push_back_payload(D3DPayload::OMSetDepthStencilState(pDepthStencilState, StencilRef));
    ret
}
unsafe extern "C" fn SOSetTargets_hook(This: *mut c_void, NumBuffers: u32, ppSOTargets: *mut *const c_void, pOffsets: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SOSetTargets)(This, NumBuffers, ppSOTargets, pOffsets);
    push_back_payload(D3DPayload::SOSetTargets(NumBuffers, ppSOTargets, pOffsets));
    ret
}
unsafe extern "C" fn DrawAuto_hook(This: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DrawAuto)(This, );
    push_back_payload(D3DPayload::DrawAuto());
    ret
}
unsafe extern "C" fn DrawIndexedInstancedIndirect_hook(This: *mut c_void, pBufferForArgs: *mut c_void, AlignedByteOffsetForArgs: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DrawIndexedInstancedIndirect)(This, pBufferForArgs, AlignedByteOffsetForArgs);
    push_back_payload(D3DPayload::DrawIndexedInstancedIndirect(pBufferForArgs, AlignedByteOffsetForArgs));
    ret
}
unsafe extern "C" fn DrawInstancedIndirect_hook(This: *mut c_void, pBufferForArgs: *mut c_void, AlignedByteOffsetForArgs: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DrawInstancedIndirect)(This, pBufferForArgs, AlignedByteOffsetForArgs);
    push_back_payload(D3DPayload::DrawInstancedIndirect(pBufferForArgs, AlignedByteOffsetForArgs));
    ret
}
unsafe extern "C" fn Dispatch_hook(This: *mut c_void, ThreadGroupCountX: u32, ThreadGroupCountY: u32, ThreadGroupCountZ: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).Dispatch)(This, ThreadGroupCountX, ThreadGroupCountY, ThreadGroupCountZ);
    push_back_payload(D3DPayload::Dispatch(ThreadGroupCountX, ThreadGroupCountY, ThreadGroupCountZ));
    ret
}
unsafe extern "C" fn DispatchIndirect_hook(This: *mut c_void, pBufferForArgs: *mut c_void, AlignedByteOffsetForArgs: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DispatchIndirect)(This, pBufferForArgs, AlignedByteOffsetForArgs);
    push_back_payload(D3DPayload::DispatchIndirect(pBufferForArgs, AlignedByteOffsetForArgs));
    ret
}
unsafe extern "C" fn RSSetState_hook(This: *mut c_void, pRasterizerState: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).RSSetState)(This, pRasterizerState);
    push_back_payload(D3DPayload::RSSetState(pRasterizerState));
    ret
}
unsafe extern "C" fn RSSetViewports_hook(This: *mut c_void, NumViewports: u32, pViewports: *mut D3D11_VIEWPORT) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).RSSetViewports)(This, NumViewports, pViewports);
    push_back_payload(D3DPayload::RSSetViewports(NumViewports, pViewports));
    ret
}
unsafe extern "C" fn RSSetScissorRects_hook(This: *mut c_void, NumRects: u32, pRects: *mut RECT) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).RSSetScissorRects)(This, NumRects, pRects);
    push_back_payload(D3DPayload::RSSetScissorRects(NumRects, pRects));
    ret
}
unsafe extern "C" fn CopySubresourceRegion_hook(This: *mut c_void, pDstResource: *mut c_void, DstSubresource: u32, DstX: u32, DstY: u32, DstZ: u32, pSrcResource: *mut c_void, SrcSubresource: u32, pSrcBox: *mut D3D11_BOX) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CopySubresourceRegion)(This, pDstResource, DstSubresource, DstX, DstY, DstZ, pSrcResource, SrcSubresource, pSrcBox);
    push_back_payload(D3DPayload::CopySubresourceRegion(pDstResource, DstSubresource, DstX, DstY, DstZ, pSrcResource, SrcSubresource, pSrcBox));
    ret
}
unsafe extern "C" fn CopyResource_hook(This: *mut c_void, pDstResource: *mut c_void, pSrcResource: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CopyResource)(This, pDstResource, pSrcResource);
    push_back_payload(D3DPayload::CopyResource(pDstResource, pSrcResource));
    ret
}
unsafe extern "C" fn UpdateSubresource_hook(This: *mut c_void, pDstResource: *mut c_void, DstSubresource: u32, pDstBox: *mut D3D11_BOX, pSrcData: *mut c_void, SrcRowPitch: u32, SrcDepthPitch: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).UpdateSubresource)(This, pDstResource, DstSubresource, pDstBox, pSrcData, SrcRowPitch, SrcDepthPitch);
    push_back_payload(D3DPayload::UpdateSubresource(pDstResource, DstSubresource, pDstBox, pSrcData, SrcRowPitch, SrcDepthPitch));
    ret
}
unsafe extern "C" fn CopyStructureCount_hook(This: *mut c_void, pDstBuffer: *mut c_void, DstAlignedByteOffset: u32, pSrcView: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CopyStructureCount)(This, pDstBuffer, DstAlignedByteOffset, pSrcView);
    push_back_payload(D3DPayload::CopyStructureCount(pDstBuffer, DstAlignedByteOffset, pSrcView));
    ret
}
unsafe extern "C" fn ClearRenderTargetView_hook(This: *mut c_void, pRenderTargetView: *mut c_void, ColorRGBA: *mut f32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ClearRenderTargetView)(This, pRenderTargetView, ColorRGBA);
    push_back_payload(D3DPayload::ClearRenderTargetView(pRenderTargetView, ColorRGBA));
    ret
}
unsafe extern "C" fn ClearUnorderedAccessViewUint_hook(This: *mut c_void, pUnorderedAccessView: *mut c_void, Values: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ClearUnorderedAccessViewUint)(This, pUnorderedAccessView, Values);
    push_back_payload(D3DPayload::ClearUnorderedAccessViewUint(pUnorderedAccessView, Values));
    ret
}
unsafe extern "C" fn ClearUnorderedAccessViewFloat_hook(This: *mut c_void, pUnorderedAccessView: *mut c_void, Values: *mut f32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ClearUnorderedAccessViewFloat)(This, pUnorderedAccessView, Values);
    push_back_payload(D3DPayload::ClearUnorderedAccessViewFloat(pUnorderedAccessView, Values));
    ret
}
unsafe extern "C" fn ClearDepthStencilView_hook(This: *mut c_void, pDepthStencilView: *mut c_void, ClearFlags: u32, Depth: f32, Stencil: u8) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ClearDepthStencilView)(This, pDepthStencilView, ClearFlags, Depth, Stencil);
    push_back_payload(D3DPayload::ClearDepthStencilView(pDepthStencilView, ClearFlags, Depth, Stencil));
    ret
}
unsafe extern "C" fn GenerateMips_hook(This: *mut c_void, pShaderResourceView: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GenerateMips)(This, pShaderResourceView);
    push_back_payload(D3DPayload::GenerateMips(pShaderResourceView));
    ret
}
unsafe extern "C" fn SetResourceMinLOD_hook(This: *mut c_void, pResource: *mut c_void, MinLOD: f32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SetResourceMinLOD)(This, pResource, MinLOD);
    push_back_payload(D3DPayload::SetResourceMinLOD(pResource, MinLOD));
    ret
}
unsafe extern "C" fn GetResourceMinLOD_hook(This: *mut c_void, pResource: *mut c_void) -> f32 {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GetResourceMinLOD)(This, pResource);
    push_back_payload(D3DPayload::GetResourceMinLOD(pResource));
    ret
}
unsafe extern "C" fn ResolveSubresource_hook(This: *mut c_void, pDstResource: *mut c_void, DstSubresource: u32, pSrcResource: *mut c_void, SrcSubresource: u32, Format: DXGI_FORMAT) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ResolveSubresource)(This, pDstResource, DstSubresource, pSrcResource, SrcSubresource, Format);
    push_back_payload(D3DPayload::ResolveSubresource(pDstResource, DstSubresource, pSrcResource, SrcSubresource, Format));
    ret
}
unsafe extern "C" fn ExecuteCommandList_hook(This: *mut c_void, pCommandList: *mut c_void, RestoreContextState: BOOL) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ExecuteCommandList)(This, pCommandList, RestoreContextState);
    push_back_payload(D3DPayload::ExecuteCommandList(pCommandList, RestoreContextState));
    ret
}
unsafe extern "C" fn HSSetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSSetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::HSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn HSSetShader_hook(This: *mut c_void, pHullShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSSetShader)(This, pHullShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::HSSetShader(pHullShader, ppClassInstances, NumClassInstances));
    ret
}
unsafe extern "C" fn HSSetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSSetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::HSSetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn HSSetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSSetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::HSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn DSSetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSSetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::DSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn DSSetShader_hook(This: *mut c_void, pDomainShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSSetShader)(This, pDomainShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::DSSetShader(pDomainShader, ppClassInstances, NumClassInstances));
    ret
}
unsafe extern "C" fn DSSetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSSetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::DSSetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn DSSetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSSetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::DSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn CSSetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSSetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::CSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn CSSetUnorderedAccessViews_hook(This: *mut c_void, StartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *const c_void, pUAVInitialCounts: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSSetUnorderedAccessViews)(This, StartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts);
    push_back_payload(D3DPayload::CSSetUnorderedAccessViews(StartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts));
    ret
}
unsafe extern "C" fn CSSetShader_hook(This: *mut c_void, pComputeShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSSetShader)(This, pComputeShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::CSSetShader(pComputeShader, ppClassInstances, NumClassInstances));
    ret
}
unsafe extern "C" fn CSSetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSSetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::CSSetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn CSSetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSSetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::CSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn VSGetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSGetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::VSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn PSGetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSGetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::PSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn PSGetShader_hook(This: *mut c_void, ppPixelShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSGetShader)(This, ppPixelShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::PSGetShader(ppPixelShader, ppClassInstances, pNumClassInstances));
    ret
}
unsafe extern "C" fn PSGetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSGetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::PSGetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn VSGetShader_hook(This: *mut c_void, ppVertexShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSGetShader)(This, ppVertexShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::VSGetShader(ppVertexShader, ppClassInstances, pNumClassInstances));
    ret
}
unsafe extern "C" fn PSGetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSGetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::PSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn IAGetInputLayout_hook(This: *mut c_void, ppInputLayout: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IAGetInputLayout)(This, ppInputLayout);
    push_back_payload(D3DPayload::IAGetInputLayout(ppInputLayout));
    ret
}
unsafe extern "C" fn IAGetVertexBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppVertexBuffers: *mut *mut c_void, pStrides: *mut u32, pOffsets: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IAGetVertexBuffers)(This, StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets);
    push_back_payload(D3DPayload::IAGetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets));
    ret
}
unsafe extern "C" fn IAGetIndexBuffer_hook(This: *mut c_void, pIndexBuffer: *mut *mut c_void, Format: *mut DXGI_FORMAT, Offset: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IAGetIndexBuffer)(This, pIndexBuffer, Format, Offset);
    push_back_payload(D3DPayload::IAGetIndexBuffer(pIndexBuffer, Format, Offset));
    ret
}
unsafe extern "C" fn GSGetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSGetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::GSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn GSGetShader_hook(This: *mut c_void, ppGeometryShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSGetShader)(This, ppGeometryShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::GSGetShader(ppGeometryShader, ppClassInstances, pNumClassInstances));
    ret
}
unsafe extern "C" fn IAGetPrimitiveTopology_hook(This: *mut c_void, pTopology: *mut D3D_PRIMITIVE_TOPOLOGY) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IAGetPrimitiveTopology)(This, pTopology);
    push_back_payload(D3DPayload::IAGetPrimitiveTopology(pTopology));
    ret
}
unsafe extern "C" fn VSGetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSGetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::VSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn VSGetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSGetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::VSGetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn GetPredication_hook(This: *mut c_void, ppPredicate: *mut *mut c_void, pPredicateValue: *mut BOOL) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GetPredication)(This, ppPredicate, pPredicateValue);
    push_back_payload(D3DPayload::GetPredication(ppPredicate, pPredicateValue));
    ret
}
unsafe extern "C" fn GSGetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSGetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::GSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn GSGetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSGetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::GSGetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn OMGetRenderTargets_hook(This: *mut c_void, NumViews: u32, ppRenderTargetViews: *mut *mut c_void, ppDepthStencilView: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).OMGetRenderTargets)(This, NumViews, ppRenderTargetViews, ppDepthStencilView);
    push_back_payload(D3DPayload::OMGetRenderTargets(NumViews, ppRenderTargetViews, ppDepthStencilView));
    ret
}
unsafe extern "C" fn OMGetRenderTargetsAndUnorderedAccessViews_hook(This: *mut c_void, NumRTVs: u32, ppRenderTargetViews: *mut *mut c_void, ppDepthStencilView: *mut *mut c_void, UAVStartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).OMGetRenderTargetsAndUnorderedAccessViews)(This, NumRTVs, ppRenderTargetViews, ppDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews);
    push_back_payload(D3DPayload::OMGetRenderTargetsAndUnorderedAccessViews(NumRTVs, ppRenderTargetViews, ppDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews));
    ret
}
unsafe extern "C" fn OMGetBlendState_hook(This: *mut c_void, ppBlendState: *mut *mut c_void, BlendFactor: *mut f32, pSampleMask: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).OMGetBlendState)(This, ppBlendState, BlendFactor, pSampleMask);
    push_back_payload(D3DPayload::OMGetBlendState(ppBlendState, BlendFactor, pSampleMask));
    ret
}
unsafe extern "C" fn OMGetDepthStencilState_hook(This: *mut c_void, ppDepthStencilState: *mut *mut c_void, pStencilRef: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).OMGetDepthStencilState)(This, ppDepthStencilState, pStencilRef);
    push_back_payload(D3DPayload::OMGetDepthStencilState(ppDepthStencilState, pStencilRef));
    ret
}
unsafe extern "C" fn SOGetTargets_hook(This: *mut c_void, NumBuffers: u32, ppSOTargets: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SOGetTargets)(This, NumBuffers, ppSOTargets);
    push_back_payload(D3DPayload::SOGetTargets(NumBuffers, ppSOTargets));
    ret
}
unsafe extern "C" fn RSGetState_hook(This: *mut c_void, ppRasterizerState: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).RSGetState)(This, ppRasterizerState);
    push_back_payload(D3DPayload::RSGetState(ppRasterizerState));
    ret
}
unsafe extern "C" fn RSGetViewports_hook(This: *mut c_void, pNumViewports: *mut u32, pViewports: *mut D3D11_VIEWPORT) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).RSGetViewports)(This, pNumViewports, pViewports);
    push_back_payload(D3DPayload::RSGetViewports(pNumViewports, pViewports));
    ret
}
unsafe extern "C" fn RSGetScissorRects_hook(This: *mut c_void, pNumRects: *mut u32, pRects: *mut RECT) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).RSGetScissorRects)(This, pNumRects, pRects);
    push_back_payload(D3DPayload::RSGetScissorRects(pNumRects, pRects));
    ret
}
unsafe extern "C" fn HSGetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSGetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::HSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn HSGetShader_hook(This: *mut c_void, ppHullShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSGetShader)(This, ppHullShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::HSGetShader(ppHullShader, ppClassInstances, pNumClassInstances));
    ret
}
unsafe extern "C" fn HSGetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSGetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::HSGetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn HSGetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSGetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::HSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn DSGetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSGetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::DSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn DSGetShader_hook(This: *mut c_void, ppDomainShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSGetShader)(This, ppDomainShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::DSGetShader(ppDomainShader, ppClassInstances, pNumClassInstances));
    ret
}
unsafe extern "C" fn DSGetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSGetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::DSGetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn DSGetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSGetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::DSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn CSGetShaderResources_hook(This: *mut c_void, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSGetShaderResources)(This, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::CSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews));
    ret
}
unsafe extern "C" fn CSGetUnorderedAccessViews_hook(This: *mut c_void, StartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSGetUnorderedAccessViews)(This, StartSlot, NumUAVs, ppUnorderedAccessViews);
    push_back_payload(D3DPayload::CSGetUnorderedAccessViews(StartSlot, NumUAVs, ppUnorderedAccessViews));
    ret
}
unsafe extern "C" fn CSGetShader_hook(This: *mut c_void, ppComputeShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSGetShader)(This, ppComputeShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::CSGetShader(ppComputeShader, ppClassInstances, pNumClassInstances));
    ret
}
unsafe extern "C" fn CSGetSamplers_hook(This: *mut c_void, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSGetSamplers)(This, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::CSGetSamplers(StartSlot, NumSamplers, ppSamplers));
    ret
}
unsafe extern "C" fn CSGetConstantBuffers_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSGetConstantBuffers)(This, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::CSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers));
    ret
}
unsafe extern "C" fn ClearState_hook(This: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ClearState)(This, );
    push_back_payload(D3DPayload::ClearState());
    ret
}
unsafe extern "C" fn Flush_hook(This: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).Flush)(This, );
    push_back_payload(D3DPayload::Flush());
    ret
}
unsafe extern "C" fn GetType_hook(This: *mut c_void) -> D3D11_DEVICE_CONTEXT_TYPE {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GetType)(This, );
    push_back_payload(D3DPayload::GetType());
    ret
}
unsafe extern "C" fn GetContextFlags_hook(This: *mut c_void) -> u32 {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GetContextFlags)(This, );
    push_back_payload(D3DPayload::GetContextFlags());
    ret
}
unsafe extern "C" fn FinishCommandList_hook(This: *mut c_void, RestoreDeferredContextState: BOOL, ppCommandList: *mut *mut c_void) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).FinishCommandList)(This, RestoreDeferredContextState, ppCommandList);
    push_back_payload(D3DPayload::FinishCommandList(RestoreDeferredContextState, ppCommandList));
    ret
}
unsafe extern "C" fn CopySubresourceRegion1_hook(This: *mut c_void, pDstResource: *mut c_void, DstSubresource: u32, DstX: u32, DstY: u32, DstZ: u32, pSrcResource: *mut c_void, SrcSubresource: u32, pSrcBox: *mut D3D11_BOX, CopyFlags: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CopySubresourceRegion1)(This, pDstResource, DstSubresource, DstX, DstY, DstZ, pSrcResource, SrcSubresource, pSrcBox, CopyFlags);
    push_back_payload(D3DPayload::CopySubresourceRegion1(pDstResource, DstSubresource, DstX, DstY, DstZ, pSrcResource, SrcSubresource, pSrcBox, CopyFlags));
    ret
}
unsafe extern "C" fn UpdateSubresource1_hook(This: *mut c_void, pDstResource: *mut c_void, DstSubresource: u32, pDstBox: *mut D3D11_BOX, pSrcData: *mut c_void, SrcRowPitch: u32, SrcDepthPitch: u32, CopyFlags: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).UpdateSubresource1)(This, pDstResource, DstSubresource, pDstBox, pSrcData, SrcRowPitch, SrcDepthPitch, CopyFlags);
    push_back_payload(D3DPayload::UpdateSubresource1(pDstResource, DstSubresource, pDstBox, pSrcData, SrcRowPitch, SrcDepthPitch, CopyFlags));
    ret
}
unsafe extern "C" fn DiscardResource_hook(This: *mut c_void, pResource: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DiscardResource)(This, pResource);
    push_back_payload(D3DPayload::DiscardResource(pResource));
    ret
}
unsafe extern "C" fn DiscardView_hook(This: *mut c_void, pResourceView: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DiscardView)(This, pResourceView);
    push_back_payload(D3DPayload::DiscardView(pResourceView));
    ret
}
unsafe extern "C" fn VSSetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSSetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::VSSetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn HSSetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSSetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::HSSetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn DSSetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSSetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::DSSetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn GSSetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSSetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::GSSetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn PSSetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSSetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::PSSetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn CSSetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSSetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::CSSetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn VSGetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).VSGetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::VSGetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn HSGetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).HSGetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::HSGetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn DSGetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DSGetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::DSGetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn GSGetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GSGetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::GSGetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn PSGetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).PSGetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::PSGetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn CSGetConstantBuffers1_hook(This: *mut c_void, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CSGetConstantBuffers1)(This, StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants);
    push_back_payload(D3DPayload::CSGetConstantBuffers1(StartSlot, NumBuffers, ppConstantBuffers, pFirstConstant, pNumConstants));
    ret
}
unsafe extern "C" fn SwapDeviceContextState_hook(This: *mut c_void, pState: *mut c_void, ppPreviousState: *mut *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SwapDeviceContextState)(This, pState, ppPreviousState);
    push_back_payload(D3DPayload::SwapDeviceContextState(pState, ppPreviousState));
    ret
}
unsafe extern "C" fn ClearView_hook(This: *mut c_void, pView: *mut c_void, Color: *mut f32, pRect: *mut RECT, NumRects: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ClearView)(This, pView, Color, pRect, NumRects);
    push_back_payload(D3DPayload::ClearView(pView, Color, pRect, NumRects));
    ret
}
unsafe extern "C" fn DiscardView1_hook(This: *mut c_void, pResourceView: *mut c_void, pRects: *mut RECT, NumRects: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).DiscardView1)(This, pResourceView, pRects, NumRects);
    push_back_payload(D3DPayload::DiscardView1(pResourceView, pRects, NumRects));
    ret
}
unsafe extern "C" fn UpdateTileMappings_hook(This: *mut c_void, pTiledResource: *mut c_void, NumTiledResourceRegions: u32, pTiledResourceRegionStartCoordinates: *mut D3D11_TILED_RESOURCE_COORDINATE, pTiledResourceRegionSizes: *mut D3D11_TILE_REGION_SIZE, pTilePool: *mut c_void, NumRanges: u32, pRangeFlags: *mut u32, pTilePoolStartOffsets: *mut u32, pRangeTileCounts: *mut u32, Flags: u32) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).UpdateTileMappings)(This, pTiledResource, NumTiledResourceRegions, pTiledResourceRegionStartCoordinates, pTiledResourceRegionSizes, pTilePool, NumRanges, pRangeFlags, pTilePoolStartOffsets, pRangeTileCounts, Flags);
    push_back_payload(D3DPayload::UpdateTileMappings(pTiledResource, NumTiledResourceRegions, pTiledResourceRegionStartCoordinates, pTiledResourceRegionSizes, pTilePool, NumRanges, pRangeFlags, pTilePoolStartOffsets, pRangeTileCounts, Flags));
    ret
}
unsafe extern "C" fn CopyTileMappings_hook(This: *mut c_void, pDestTiledResource: *mut c_void, pDestRegionStartCoordinate: *mut D3D11_TILED_RESOURCE_COORDINATE, pSourceTiledResource: *mut c_void, pSourceRegionStartCoordinate: *mut D3D11_TILED_RESOURCE_COORDINATE, pTileRegionSize: *mut D3D11_TILE_REGION_SIZE, Flags: u32) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CopyTileMappings)(This, pDestTiledResource, pDestRegionStartCoordinate, pSourceTiledResource, pSourceRegionStartCoordinate, pTileRegionSize, Flags);
    push_back_payload(D3DPayload::CopyTileMappings(pDestTiledResource, pDestRegionStartCoordinate, pSourceTiledResource, pSourceRegionStartCoordinate, pTileRegionSize, Flags));
    ret
}
unsafe extern "C" fn CopyTiles_hook(This: *mut c_void, pTiledResource: *mut c_void, pTileRegionStartCoordinate: *mut D3D11_TILED_RESOURCE_COORDINATE, pTileRegionSize: *mut D3D11_TILE_REGION_SIZE, pBuffer: *mut c_void, BufferStartOffsetInBytes: u64, Flags: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).CopyTiles)(This, pTiledResource, pTileRegionStartCoordinate, pTileRegionSize, pBuffer, BufferStartOffsetInBytes, Flags);
    push_back_payload(D3DPayload::CopyTiles(pTiledResource, pTileRegionStartCoordinate, pTileRegionSize, pBuffer, BufferStartOffsetInBytes, Flags));
    ret
}
unsafe extern "C" fn UpdateTiles_hook(This: *mut c_void, pDestTiledResource: *mut c_void, pDestTileRegionStartCoordinate: *mut D3D11_TILED_RESOURCE_COORDINATE, pDestTileRegionSize: *mut D3D11_TILE_REGION_SIZE, pSourceTileData: *mut c_void, Flags: u32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).UpdateTiles)(This, pDestTiledResource, pDestTileRegionStartCoordinate, pDestTileRegionSize, pSourceTileData, Flags);
    push_back_payload(D3DPayload::UpdateTiles(pDestTiledResource, pDestTileRegionStartCoordinate, pDestTileRegionSize, pSourceTileData, Flags));
    ret
}
unsafe extern "C" fn ResizeTilePool_hook(This: *mut c_void, pTilePool: *mut c_void, NewSizeInBytes: u64) -> HRESULT {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).ResizeTilePool)(This, pTilePool, NewSizeInBytes);
    push_back_payload(D3DPayload::ResizeTilePool(pTilePool, NewSizeInBytes));
    ret
}
unsafe extern "C" fn TiledResourceBarrier_hook(This: *mut c_void, pTiledResourceOrViewAccessBeforeBarrier: *mut c_void, pTiledResourceOrViewAccessAfterBarrier: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).TiledResourceBarrier)(This, pTiledResourceOrViewAccessBeforeBarrier, pTiledResourceOrViewAccessAfterBarrier);
    push_back_payload(D3DPayload::TiledResourceBarrier(pTiledResourceOrViewAccessBeforeBarrier, pTiledResourceOrViewAccessAfterBarrier));
    ret
}
unsafe extern "C" fn IsAnnotationEnabled_hook(This: *mut c_void) -> BOOL {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).IsAnnotationEnabled)(This, );
    push_back_payload(D3DPayload::IsAnnotationEnabled());
    ret
}
unsafe extern "C" fn SetMarkerInt_hook(This: *mut c_void, pLabel: *const PWSTR, Data: i32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SetMarkerInt)(This, pLabel, Data);
    push_back_payload(D3DPayload::SetMarkerInt(pLabel, Data));
    ret
}
unsafe extern "C" fn BeginEventInt_hook(This: *mut c_void, pLabel: *const PWSTR, Data: i32) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).BeginEventInt)(This, pLabel, Data);
    push_back_payload(D3DPayload::BeginEventInt(pLabel, Data));
    ret
}
unsafe extern "C" fn EndEvent_hook(This: *mut c_void) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).EndEvent)(This, );
    push_back_payload(D3DPayload::EndEvent());
    ret
}
unsafe extern "C" fn Flush1_hook(This: *mut c_void, ContextType: D3D11_CONTEXT_TYPE, hEvent: HANDLE) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).Flush1)(This, ContextType, hEvent);
    push_back_payload(D3DPayload::Flush1(ContextType, hEvent));
    ret
}
unsafe extern "C" fn SetHardwareProtectionState_hook(This: *mut c_void, HwProtectionEnable: BOOL) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).SetHardwareProtectionState)(This, HwProtectionEnable);
    push_back_payload(D3DPayload::SetHardwareProtectionState(HwProtectionEnable));
    ret
}
unsafe extern "C" fn GetHardwareProtectionState_hook(This: *mut c_void, pHwProtectionEnable: *mut BOOL) {
    let ret = ((*ORIGINAL_VTABLE.unwrap()).GetHardwareProtectionState)(This, pHwProtectionEnable);
    push_back_payload(D3DPayload::GetHardwareProtectionState(pHwProtectionEnable));
    ret
}

pub struct HookState(Vec<*mut u8>);
impl Drop for HookState {
    fn drop(&mut self) {
        if rendering::CAPTURE_D3D_COMMANDS {
            for patch in self.0.iter().rev() {
                unsafe {
                    Patcher::get_mut().unwrap().unpatch(*patch);
                }
            }
        }
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    use crate::game::graphics::kernel::Device;

    if !rendering::CAPTURE_D3D_COMMANDS {
        return Ok(HookState(vec![]));
    }

    let immediate_context: &mut _ = Device::get().immediate_context_mut();
    let device_context = immediate_context.device_context_mut();
    let device_context_ptr: *mut ID3D11DeviceContext = std::mem::transmute(device_context);
    let device_context_vtable_ptr = std::ptr::addr_of_mut!((*device_context_ptr).vtbl);
    ORIGINAL_VTABLE = Some(*device_context_vtable_ptr);
    let device_context_new_vtable_ptr_bytes = (std::ptr::addr_of!(HOOKED_VTABLE) as usize).to_le_bytes();

    let patcher = Patcher::get_mut().ok_or_else(|| anyhow::Error::msg("Failed to retrieve patcher"))?;
    let ptrs = vec![
        patcher.patch(device_context_vtable_ptr as *mut u8, &device_context_new_vtable_ptr_bytes)
    ];

    Ok(HookState(ptrs))
}

