#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::singleton;
use crate::debugger::{Debugger, D3DPayload};
use crate::hooks::Patcher;
use crate::ct_config::*;

use bindings::Windows::Win32::Graphics::Direct3D11::{
    D3D_PRIMITIVE_TOPOLOGY, D3D11_MAPPED_SUBRESOURCE, D3D11_MAP, D3D11_VIEWPORT, D3D11_BOX,
    D3D11_DEVICE_CONTEXT_TYPE
};
use bindings::Windows::Win32::Graphics::Dxgi::{DXGI_FORMAT};
use bindings::Windows::Win32::Foundation::{BOOL, RECT};

use windows::*;
use std::os::raw::c_void;

struct ID3D11DeviceContextUnhooked {
    vtbl: *const ID3D11DeviceContextVtbl<ID3D11DeviceContextUnhooked>,
}

pub struct ID3D11DeviceContextHooked {
    vtbl: *const ID3D11DeviceContextVtbl<ID3D11DeviceContextHooked>,
    original: *mut ID3D11DeviceContextUnhooked,
    vtbl_instance: ID3D11DeviceContextVtbl<ID3D11DeviceContextHooked>,
}
singleton!(ID3D11DeviceContextHooked, original: *mut c_void);
struct ID3D11DeviceContextVtbl<DeviceContextType> {
    QueryInterface: unsafe extern "C" fn(*mut DeviceContextType, *const Guid, *mut *mut c_void) -> HRESULT,
    AddRef: unsafe extern "C" fn(*mut DeviceContextType) -> u32,
    Release: unsafe extern "C" fn(*mut DeviceContextType) -> u32,
    GetDevice: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void),
    GetPrivateData: unsafe extern "C" fn(*mut DeviceContextType, *const Guid, *mut u32, *mut c_void) -> HRESULT,
    SetPrivateData: unsafe extern "C" fn(*mut DeviceContextType, *const Guid, u32, *mut c_void) -> HRESULT,
    SetPrivateDataInterface: unsafe extern "C" fn(*mut DeviceContextType, *const Guid, *mut IUnknown) -> HRESULT,
    VSSetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    PSSetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    PSSetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut *const c_void, u32),
    PSSetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    VSSetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut *const c_void, u32),
    DrawIndexed: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, i32),
    Draw: unsafe extern "C" fn(*mut DeviceContextType, u32, u32),
    Map: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32, D3D11_MAP, u32, *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT,
    Unmap: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32),
    PSSetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    IASetInputLayout: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void),
    IASetVertexBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void, *mut u32, *mut u32),
    IASetIndexBuffer: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, DXGI_FORMAT, u32),
    DrawIndexedInstanced: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, u32, i32, u32),
    DrawInstanced: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, u32, u32),
    GSSetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    GSSetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut *const c_void, u32),
    IASetPrimitiveTopology: unsafe extern "C" fn(*mut DeviceContextType, D3D_PRIMITIVE_TOPOLOGY),
    VSSetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    VSSetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    Begin: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void),
    End: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void),
    GetData: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut c_void, u32, u32) -> HRESULT,
    SetPredication: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, BOOL),
    GSSetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    GSSetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    OMSetRenderTargets: unsafe extern "C" fn(*mut DeviceContextType, u32, *mut *const c_void, *mut c_void),
    OMSetRenderTargetsAndUnorderedAccessViews: unsafe extern "C" fn(*mut DeviceContextType, u32, *mut *const c_void, *mut c_void, u32, u32, *mut *const c_void, *mut u32),
    OMSetBlendState: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut f32, u32),
    OMSetDepthStencilState: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32),
    SOSetTargets: unsafe extern "C" fn(*mut DeviceContextType, u32, *mut *const c_void, *mut u32),
    DrawAuto: unsafe extern "C" fn(*mut DeviceContextType),
    DrawIndexedInstancedIndirect: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32),
    DrawInstancedIndirect: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32),
    Dispatch: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, u32),
    DispatchIndirect: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32),
    RSSetState: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void),
    RSSetViewports: unsafe extern "C" fn(*mut DeviceContextType, u32, *mut D3D11_VIEWPORT),
    RSSetScissorRects: unsafe extern "C" fn(*mut DeviceContextType, u32, *mut RECT),
    CopySubresourceRegion: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32, u32, u32, u32, *mut c_void, u32, *mut D3D11_BOX),
    CopyResource: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut c_void),
    UpdateSubresource: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32, *mut D3D11_BOX, *mut c_void, u32, u32),
    CopyStructureCount: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32, *mut c_void),
    ClearRenderTargetView: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut f32),
    ClearUnorderedAccessViewUint: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut u32),
    ClearUnorderedAccessViewFloat: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut f32),
    ClearDepthStencilView: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32, f32, u8),
    GenerateMips: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void),
    SetResourceMinLOD: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, f32),
    GetResourceMinLOD: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void) -> f32,
    ResolveSubresource: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, u32, *mut c_void, u32, DXGI_FORMAT),
    ExecuteCommandList: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, BOOL),
    HSSetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    HSSetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut *const c_void, u32),
    HSSetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    HSSetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    DSSetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    DSSetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut *const c_void, u32),
    DSSetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    DSSetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    CSSetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    CSSetUnorderedAccessViews: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void, *mut u32),
    CSSetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut c_void, *mut *const c_void, u32),
    CSSetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    CSSetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *const c_void),
    VSGetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    PSGetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    PSGetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut *mut c_void, *mut u32),
    PSGetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    VSGetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut *mut c_void, *mut u32),
    PSGetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    IAGetInputLayout: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void),
    IAGetVertexBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    IAGetIndexBuffer: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut DXGI_FORMAT, *mut u32),
    GSGetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    GSGetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut *mut c_void, *mut u32),
    IAGetPrimitiveTopology: unsafe extern "C" fn(*mut DeviceContextType, *mut D3D_PRIMITIVE_TOPOLOGY),
    VSGetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    VSGetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    GetPredication: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut BOOL),
    GSGetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    GSGetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    OMGetRenderTargets: unsafe extern "C" fn(*mut DeviceContextType, u32, *mut *mut c_void, *mut *mut c_void),
    OMGetRenderTargetsAndUnorderedAccessViews: unsafe extern "C" fn(*mut DeviceContextType, u32, *mut *mut c_void, *mut *mut c_void, u32, u32, *mut *mut c_void),
    OMGetBlendState: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut f32, *mut u32),
    OMGetDepthStencilState: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut u32),
    SOGetTargets: unsafe extern "C" fn(*mut DeviceContextType, u32, *mut *mut c_void),
    RSGetState: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void),
    RSGetViewports: unsafe extern "C" fn(*mut DeviceContextType, *mut u32, *mut D3D11_VIEWPORT),
    RSGetScissorRects: unsafe extern "C" fn(*mut DeviceContextType, *mut u32, *mut RECT),
    HSGetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    HSGetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut *mut c_void, *mut u32),
    HSGetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    HSGetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    DSGetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    DSGetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut *mut c_void, *mut u32),
    DSGetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    DSGetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    CSGetShaderResources: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    CSGetUnorderedAccessViews: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    CSGetShader: unsafe extern "C" fn(*mut DeviceContextType, *mut *mut c_void, *mut *mut c_void, *mut u32),
    CSGetSamplers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    CSGetConstantBuffers: unsafe extern "C" fn(*mut DeviceContextType, u32, u32, *mut *mut c_void),
    ClearState: unsafe extern "C" fn(*mut DeviceContextType),
    Flush: unsafe extern "C" fn(*mut DeviceContextType),
    GetType: unsafe extern "C" fn(*mut DeviceContextType) -> D3D11_DEVICE_CONTEXT_TYPE,
    GetContextFlags: unsafe extern "C" fn(*mut DeviceContextType) -> u32,
    FinishCommandList: unsafe extern "C" fn(*mut DeviceContextType, BOOL, *mut *mut c_void) -> HRESULT,
}

fn push_back_payload(payload: D3DPayload) {
    if let Some(debugger) = Debugger::get_mut() {
        let mut command_stream = debugger.command_stream.lock().unwrap();
        command_stream.add_d3d_command(payload).unwrap();
    }
}
unsafe extern "C" fn QueryInterface_hook(This: *mut ID3D11DeviceContextHooked, riid: *const Guid, ppvObject: *mut *mut c_void) -> HRESULT {
    let ret = ((*(*(*This).original).vtbl).QueryInterface)((*This).original as *mut _, riid, ppvObject);
    push_back_payload(D3DPayload::QueryInterface);
    ret
}
unsafe extern "C" fn AddRef_hook(This: *mut ID3D11DeviceContextHooked) -> u32 {
    let ret = ((*(*(*This).original).vtbl).AddRef)((*This).original as *mut _, );
    push_back_payload(D3DPayload::AddRef);
    ret
}
unsafe extern "C" fn Release_hook(This: *mut ID3D11DeviceContextHooked) -> u32 {
    let ret = ((*(*(*This).original).vtbl).Release)((*This).original as *mut _, );
    push_back_payload(D3DPayload::Release);
    ret
}
unsafe extern "C" fn GetDevice_hook(This: *mut ID3D11DeviceContextHooked, ppDevice: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).GetDevice)((*This).original as *mut _, ppDevice);
    push_back_payload(D3DPayload::GetDevice);
    ret
}
unsafe extern "C" fn GetPrivateData_hook(This: *mut ID3D11DeviceContextHooked, guid: *const Guid, pDataSize: *mut u32, pData: *mut c_void) -> HRESULT {
    let ret = ((*(*(*This).original).vtbl).GetPrivateData)((*This).original as *mut _, guid, pDataSize, pData);
    push_back_payload(D3DPayload::GetPrivateData);
    ret
}
unsafe extern "C" fn SetPrivateData_hook(This: *mut ID3D11DeviceContextHooked, guid: *const Guid, DataSize: u32, pData: *mut c_void) -> HRESULT {
    let ret = ((*(*(*This).original).vtbl).SetPrivateData)((*This).original as *mut _, guid, DataSize, pData);
    push_back_payload(D3DPayload::SetPrivateData);
    ret
}
unsafe extern "C" fn SetPrivateDataInterface_hook(This: *mut ID3D11DeviceContextHooked, guid: *const Guid, pData: *mut IUnknown) -> HRESULT {
    let ret = ((*(*(*This).original).vtbl).SetPrivateDataInterface)((*This).original as *mut _, guid, pData);
    push_back_payload(D3DPayload::SetPrivateDataInterface);
    ret
}
unsafe extern "C" fn VSSetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).VSSetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::VSSetConstantBuffers);
    ret
}
unsafe extern "C" fn PSSetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).PSSetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::PSSetShaderResources);
    ret
}
unsafe extern "C" fn PSSetShader_hook(This: *mut ID3D11DeviceContextHooked, pPixelShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*(*(*This).original).vtbl).PSSetShader)((*This).original as *mut _, pPixelShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::PSSetShader);
    ret
}
unsafe extern "C" fn PSSetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).PSSetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::PSSetSamplers);
    ret
}
unsafe extern "C" fn VSSetShader_hook(This: *mut ID3D11DeviceContextHooked, pVertexShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*(*(*This).original).vtbl).VSSetShader)((*This).original as *mut _, pVertexShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::VSSetShader);
    ret
}
unsafe extern "C" fn DrawIndexed_hook(This: *mut ID3D11DeviceContextHooked, IndexCount: u32, StartIndexLocation: u32, BaseVertexLocation: i32) {
    let ret = ((*(*(*This).original).vtbl).DrawIndexed)((*This).original as *mut _, IndexCount, StartIndexLocation, BaseVertexLocation);
    push_back_payload(D3DPayload::DrawIndexed);
    ret
}
unsafe extern "C" fn Draw_hook(This: *mut ID3D11DeviceContextHooked, VertexCount: u32, StartVertexLocation: u32) {
    let ret = ((*(*(*This).original).vtbl).Draw)((*This).original as *mut _, VertexCount, StartVertexLocation);
    push_back_payload(D3DPayload::Draw);
    ret
}
unsafe extern "C" fn Map_hook(This: *mut ID3D11DeviceContextHooked, pResource: *mut c_void, Subresource: u32, MapType: D3D11_MAP, MapFlags: u32, pMappedResource: *mut D3D11_MAPPED_SUBRESOURCE) -> HRESULT {
    let ret = ((*(*(*This).original).vtbl).Map)((*This).original as *mut _, pResource, Subresource, MapType, MapFlags, pMappedResource);
    push_back_payload(D3DPayload::Map);
    ret
}
unsafe extern "C" fn Unmap_hook(This: *mut ID3D11DeviceContextHooked, pResource: *mut c_void, Subresource: u32) {
    let ret = ((*(*(*This).original).vtbl).Unmap)((*This).original as *mut _, pResource, Subresource);
    push_back_payload(D3DPayload::Unmap);
    ret
}
unsafe extern "C" fn PSSetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).PSSetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::PSSetConstantBuffers);
    ret
}
unsafe extern "C" fn IASetInputLayout_hook(This: *mut ID3D11DeviceContextHooked, pInputLayout: *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).IASetInputLayout)((*This).original as *mut _, pInputLayout);
    push_back_payload(D3DPayload::IASetInputLayout);
    ret
}
unsafe extern "C" fn IASetVertexBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppVertexBuffers: *mut *const c_void, pStrides: *mut u32, pOffsets: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).IASetVertexBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets);
    push_back_payload(D3DPayload::IASetVertexBuffers);
    ret
}
unsafe extern "C" fn IASetIndexBuffer_hook(This: *mut ID3D11DeviceContextHooked, pIndexBuffer: *mut c_void, Format: DXGI_FORMAT, Offset: u32) {
    let ret = ((*(*(*This).original).vtbl).IASetIndexBuffer)((*This).original as *mut _, pIndexBuffer, Format, Offset);
    push_back_payload(D3DPayload::IASetIndexBuffer);
    ret
}
unsafe extern "C" fn DrawIndexedInstanced_hook(This: *mut ID3D11DeviceContextHooked, IndexCountPerInstance: u32, InstanceCount: u32, StartIndexLocation: u32, BaseVertexLocation: i32, StartInstanceLocation: u32) {
    let ret = ((*(*(*This).original).vtbl).DrawIndexedInstanced)((*This).original as *mut _, IndexCountPerInstance, InstanceCount, StartIndexLocation, BaseVertexLocation, StartInstanceLocation);
    push_back_payload(D3DPayload::DrawIndexedInstanced);
    ret
}
unsafe extern "C" fn DrawInstanced_hook(This: *mut ID3D11DeviceContextHooked, VertexCountPerInstance: u32, InstanceCount: u32, StartVertexLocation: u32, StartInstanceLocation: u32) {
    let ret = ((*(*(*This).original).vtbl).DrawInstanced)((*This).original as *mut _, VertexCountPerInstance, InstanceCount, StartVertexLocation, StartInstanceLocation);
    push_back_payload(D3DPayload::DrawInstanced);
    ret
}
unsafe extern "C" fn GSSetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).GSSetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::GSSetConstantBuffers);
    ret
}
unsafe extern "C" fn GSSetShader_hook(This: *mut ID3D11DeviceContextHooked, pShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*(*(*This).original).vtbl).GSSetShader)((*This).original as *mut _, pShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::GSSetShader);
    ret
}
unsafe extern "C" fn IASetPrimitiveTopology_hook(This: *mut ID3D11DeviceContextHooked, Topology: D3D_PRIMITIVE_TOPOLOGY) {
    let ret = ((*(*(*This).original).vtbl).IASetPrimitiveTopology)((*This).original as *mut _, Topology);
    push_back_payload(D3DPayload::IASetPrimitiveTopology);
    ret
}
unsafe extern "C" fn VSSetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).VSSetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::VSSetShaderResources);
    ret
}
unsafe extern "C" fn VSSetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).VSSetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::VSSetSamplers);
    ret
}
unsafe extern "C" fn Begin_hook(This: *mut ID3D11DeviceContextHooked, pAsync: *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).Begin)((*This).original as *mut _, pAsync);
    push_back_payload(D3DPayload::Begin);
    ret
}
unsafe extern "C" fn End_hook(This: *mut ID3D11DeviceContextHooked, pAsync: *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).End)((*This).original as *mut _, pAsync);
    push_back_payload(D3DPayload::End);
    ret
}
unsafe extern "C" fn GetData_hook(This: *mut ID3D11DeviceContextHooked, pAsync: *mut c_void, pData: *mut c_void, DataSize: u32, GetDataFlags: u32) -> HRESULT {
    let ret = ((*(*(*This).original).vtbl).GetData)((*This).original as *mut _, pAsync, pData, DataSize, GetDataFlags);
    push_back_payload(D3DPayload::GetData);
    ret
}
unsafe extern "C" fn SetPredication_hook(This: *mut ID3D11DeviceContextHooked, pPredicate: *mut c_void, PredicateValue: BOOL) {
    let ret = ((*(*(*This).original).vtbl).SetPredication)((*This).original as *mut _, pPredicate, PredicateValue);
    push_back_payload(D3DPayload::SetPredication);
    ret
}
unsafe extern "C" fn GSSetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).GSSetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::GSSetShaderResources);
    ret
}
unsafe extern "C" fn GSSetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).GSSetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::GSSetSamplers);
    ret
}
unsafe extern "C" fn OMSetRenderTargets_hook(This: *mut ID3D11DeviceContextHooked, NumViews: u32, ppRenderTargetViews: *mut *const c_void, pDepthStencilView: *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).OMSetRenderTargets)((*This).original as *mut _, NumViews, ppRenderTargetViews, pDepthStencilView);
    push_back_payload(D3DPayload::OMSetRenderTargets);
    ret
}
unsafe extern "C" fn OMSetRenderTargetsAndUnorderedAccessViews_hook(This: *mut ID3D11DeviceContextHooked, NumRTVs: u32, ppRenderTargetViews: *mut *const c_void, pDepthStencilView: *mut c_void, UAVStartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *const c_void, pUAVInitialCounts: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).OMSetRenderTargetsAndUnorderedAccessViews)((*This).original as *mut _, NumRTVs, ppRenderTargetViews, pDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts);
    push_back_payload(D3DPayload::OMSetRenderTargetsAndUnorderedAccessViews);
    ret
}
unsafe extern "C" fn OMSetBlendState_hook(This: *mut ID3D11DeviceContextHooked, pBlendState: *mut c_void, BlendFactor: *mut f32, SampleMask: u32) {
    let ret = ((*(*(*This).original).vtbl).OMSetBlendState)((*This).original as *mut _, pBlendState, BlendFactor, SampleMask);
    push_back_payload(D3DPayload::OMSetBlendState);
    ret
}
unsafe extern "C" fn OMSetDepthStencilState_hook(This: *mut ID3D11DeviceContextHooked, pDepthStencilState: *mut c_void, StencilRef: u32) {
    let ret = ((*(*(*This).original).vtbl).OMSetDepthStencilState)((*This).original as *mut _, pDepthStencilState, StencilRef);
    push_back_payload(D3DPayload::OMSetDepthStencilState);
    ret
}
unsafe extern "C" fn SOSetTargets_hook(This: *mut ID3D11DeviceContextHooked, NumBuffers: u32, ppSOTargets: *mut *const c_void, pOffsets: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).SOSetTargets)((*This).original as *mut _, NumBuffers, ppSOTargets, pOffsets);
    push_back_payload(D3DPayload::SOSetTargets);
    ret
}
unsafe extern "C" fn DrawAuto_hook(This: *mut ID3D11DeviceContextHooked) {
    let ret = ((*(*(*This).original).vtbl).DrawAuto)((*This).original as *mut _, );
    push_back_payload(D3DPayload::DrawAuto);
    ret
}
unsafe extern "C" fn DrawIndexedInstancedIndirect_hook(This: *mut ID3D11DeviceContextHooked, pBufferForArgs: *mut c_void, AlignedByteOffsetForArgs: u32) {
    let ret = ((*(*(*This).original).vtbl).DrawIndexedInstancedIndirect)((*This).original as *mut _, pBufferForArgs, AlignedByteOffsetForArgs);
    push_back_payload(D3DPayload::DrawIndexedInstancedIndirect);
    ret
}
unsafe extern "C" fn DrawInstancedIndirect_hook(This: *mut ID3D11DeviceContextHooked, pBufferForArgs: *mut c_void, AlignedByteOffsetForArgs: u32) {
    let ret = ((*(*(*This).original).vtbl).DrawInstancedIndirect)((*This).original as *mut _, pBufferForArgs, AlignedByteOffsetForArgs);
    push_back_payload(D3DPayload::DrawInstancedIndirect);
    ret
}
unsafe extern "C" fn Dispatch_hook(This: *mut ID3D11DeviceContextHooked, ThreadGroupCountX: u32, ThreadGroupCountY: u32, ThreadGroupCountZ: u32) {
    let ret = ((*(*(*This).original).vtbl).Dispatch)((*This).original as *mut _, ThreadGroupCountX, ThreadGroupCountY, ThreadGroupCountZ);
    push_back_payload(D3DPayload::Dispatch);
    ret
}
unsafe extern "C" fn DispatchIndirect_hook(This: *mut ID3D11DeviceContextHooked, pBufferForArgs: *mut c_void, AlignedByteOffsetForArgs: u32) {
    let ret = ((*(*(*This).original).vtbl).DispatchIndirect)((*This).original as *mut _, pBufferForArgs, AlignedByteOffsetForArgs);
    push_back_payload(D3DPayload::DispatchIndirect);
    ret
}
unsafe extern "C" fn RSSetState_hook(This: *mut ID3D11DeviceContextHooked, pRasterizerState: *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).RSSetState)((*This).original as *mut _, pRasterizerState);
    push_back_payload(D3DPayload::RSSetState);
    ret
}
unsafe extern "C" fn RSSetViewports_hook(This: *mut ID3D11DeviceContextHooked, NumViewports: u32, pViewports: *mut D3D11_VIEWPORT) {
    let ret = ((*(*(*This).original).vtbl).RSSetViewports)((*This).original as *mut _, NumViewports, pViewports);
    push_back_payload(D3DPayload::RSSetViewports);
    ret
}
unsafe extern "C" fn RSSetScissorRects_hook(This: *mut ID3D11DeviceContextHooked, NumRects: u32, pRects: *mut RECT) {
    let ret = ((*(*(*This).original).vtbl).RSSetScissorRects)((*This).original as *mut _, NumRects, pRects);
    push_back_payload(D3DPayload::RSSetScissorRects);
    ret
}
unsafe extern "C" fn CopySubresourceRegion_hook(This: *mut ID3D11DeviceContextHooked, pDstResource: *mut c_void, DstSubresource: u32, DstX: u32, DstY: u32, DstZ: u32, pSrcResource: *mut c_void, SrcSubresource: u32, pSrcBox: *mut D3D11_BOX) {
    let ret = ((*(*(*This).original).vtbl).CopySubresourceRegion)((*This).original as *mut _, pDstResource, DstSubresource, DstX, DstY, DstZ, pSrcResource, SrcSubresource, pSrcBox);
    push_back_payload(D3DPayload::CopySubresourceRegion);
    ret
}
unsafe extern "C" fn CopyResource_hook(This: *mut ID3D11DeviceContextHooked, pDstResource: *mut c_void, pSrcResource: *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).CopyResource)((*This).original as *mut _, pDstResource, pSrcResource);
    push_back_payload(D3DPayload::CopyResource);
    ret
}
unsafe extern "C" fn UpdateSubresource_hook(This: *mut ID3D11DeviceContextHooked, pDstResource: *mut c_void, DstSubresource: u32, pDstBox: *mut D3D11_BOX, pSrcData: *mut c_void, SrcRowPitch: u32, SrcDepthPitch: u32) {
    let ret = ((*(*(*This).original).vtbl).UpdateSubresource)((*This).original as *mut _, pDstResource, DstSubresource, pDstBox, pSrcData, SrcRowPitch, SrcDepthPitch);
    push_back_payload(D3DPayload::UpdateSubresource);
    ret
}
unsafe extern "C" fn CopyStructureCount_hook(This: *mut ID3D11DeviceContextHooked, pDstBuffer: *mut c_void, DstAlignedByteOffset: u32, pSrcView: *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).CopyStructureCount)((*This).original as *mut _, pDstBuffer, DstAlignedByteOffset, pSrcView);
    push_back_payload(D3DPayload::CopyStructureCount);
    ret
}
unsafe extern "C" fn ClearRenderTargetView_hook(This: *mut ID3D11DeviceContextHooked, pRenderTargetView: *mut c_void, ColorRGBA: *mut f32) {
    let ret = ((*(*(*This).original).vtbl).ClearRenderTargetView)((*This).original as *mut _, pRenderTargetView, ColorRGBA);
    push_back_payload(D3DPayload::ClearRenderTargetView);
    ret
}
unsafe extern "C" fn ClearUnorderedAccessViewUint_hook(This: *mut ID3D11DeviceContextHooked, pUnorderedAccessView: *mut c_void, Values: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).ClearUnorderedAccessViewUint)((*This).original as *mut _, pUnorderedAccessView, Values);
    push_back_payload(D3DPayload::ClearUnorderedAccessViewUint);
    ret
}
unsafe extern "C" fn ClearUnorderedAccessViewFloat_hook(This: *mut ID3D11DeviceContextHooked, pUnorderedAccessView: *mut c_void, Values: *mut f32) {
    let ret = ((*(*(*This).original).vtbl).ClearUnorderedAccessViewFloat)((*This).original as *mut _, pUnorderedAccessView, Values);
    push_back_payload(D3DPayload::ClearUnorderedAccessViewFloat);
    ret
}
unsafe extern "C" fn ClearDepthStencilView_hook(This: *mut ID3D11DeviceContextHooked, pDepthStencilView: *mut c_void, ClearFlags: u32, Depth: f32, Stencil: u8) {
    let ret = ((*(*(*This).original).vtbl).ClearDepthStencilView)((*This).original as *mut _, pDepthStencilView, ClearFlags, Depth, Stencil);
    push_back_payload(D3DPayload::ClearDepthStencilView);
    ret
}
unsafe extern "C" fn GenerateMips_hook(This: *mut ID3D11DeviceContextHooked, pShaderResourceView: *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).GenerateMips)((*This).original as *mut _, pShaderResourceView);
    push_back_payload(D3DPayload::GenerateMips);
    ret
}
unsafe extern "C" fn SetResourceMinLOD_hook(This: *mut ID3D11DeviceContextHooked, pResource: *mut c_void, MinLOD: f32) {
    let ret = ((*(*(*This).original).vtbl).SetResourceMinLOD)((*This).original as *mut _, pResource, MinLOD);
    push_back_payload(D3DPayload::SetResourceMinLOD);
    ret
}
unsafe extern "C" fn GetResourceMinLOD_hook(This: *mut ID3D11DeviceContextHooked, pResource: *mut c_void) -> f32 {
    let ret = ((*(*(*This).original).vtbl).GetResourceMinLOD)((*This).original as *mut _, pResource);
    push_back_payload(D3DPayload::GetResourceMinLOD);
    ret
}
unsafe extern "C" fn ResolveSubresource_hook(This: *mut ID3D11DeviceContextHooked, pDstResource: *mut c_void, DstSubresource: u32, pSrcResource: *mut c_void, SrcSubresource: u32, Format: DXGI_FORMAT) {
    let ret = ((*(*(*This).original).vtbl).ResolveSubresource)((*This).original as *mut _, pDstResource, DstSubresource, pSrcResource, SrcSubresource, Format);
    push_back_payload(D3DPayload::ResolveSubresource);
    ret
}
unsafe extern "C" fn ExecuteCommandList_hook(This: *mut ID3D11DeviceContextHooked, pCommandList: *mut c_void, RestoreContextState: BOOL) {
    let ret = ((*(*(*This).original).vtbl).ExecuteCommandList)((*This).original as *mut _, pCommandList, RestoreContextState);
    push_back_payload(D3DPayload::ExecuteCommandList);
    ret
}
unsafe extern "C" fn HSSetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).HSSetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::HSSetShaderResources);
    ret
}
unsafe extern "C" fn HSSetShader_hook(This: *mut ID3D11DeviceContextHooked, pHullShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*(*(*This).original).vtbl).HSSetShader)((*This).original as *mut _, pHullShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::HSSetShader);
    ret
}
unsafe extern "C" fn HSSetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).HSSetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::HSSetSamplers);
    ret
}
unsafe extern "C" fn HSSetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).HSSetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::HSSetConstantBuffers);
    ret
}
unsafe extern "C" fn DSSetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).DSSetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::DSSetShaderResources);
    ret
}
unsafe extern "C" fn DSSetShader_hook(This: *mut ID3D11DeviceContextHooked, pDomainShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*(*(*This).original).vtbl).DSSetShader)((*This).original as *mut _, pDomainShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::DSSetShader);
    ret
}
unsafe extern "C" fn DSSetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).DSSetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::DSSetSamplers);
    ret
}
unsafe extern "C" fn DSSetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).DSSetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::DSSetConstantBuffers);
    ret
}
unsafe extern "C" fn CSSetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).CSSetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::CSSetShaderResources);
    ret
}
unsafe extern "C" fn CSSetUnorderedAccessViews_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *const c_void, pUAVInitialCounts: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).CSSetUnorderedAccessViews)((*This).original as *mut _, StartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts);
    push_back_payload(D3DPayload::CSSetUnorderedAccessViews);
    ret
}
unsafe extern "C" fn CSSetShader_hook(This: *mut ID3D11DeviceContextHooked, pComputeShader: *mut c_void, ppClassInstances: *mut *const c_void, NumClassInstances: u32) {
    let ret = ((*(*(*This).original).vtbl).CSSetShader)((*This).original as *mut _, pComputeShader, ppClassInstances, NumClassInstances);
    push_back_payload(D3DPayload::CSSetShader);
    ret
}
unsafe extern "C" fn CSSetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).CSSetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::CSSetSamplers);
    ret
}
unsafe extern "C" fn CSSetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *const c_void) {
    let ret = ((*(*(*This).original).vtbl).CSSetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::CSSetConstantBuffers);
    ret
}
unsafe extern "C" fn VSGetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).VSGetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::VSGetConstantBuffers);
    ret
}
unsafe extern "C" fn PSGetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).PSGetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::PSGetShaderResources);
    ret
}
unsafe extern "C" fn PSGetShader_hook(This: *mut ID3D11DeviceContextHooked, ppPixelShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).PSGetShader)((*This).original as *mut _, ppPixelShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::PSGetShader);
    ret
}
unsafe extern "C" fn PSGetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).PSGetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::PSGetSamplers);
    ret
}
unsafe extern "C" fn VSGetShader_hook(This: *mut ID3D11DeviceContextHooked, ppVertexShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).VSGetShader)((*This).original as *mut _, ppVertexShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::VSGetShader);
    ret
}
unsafe extern "C" fn PSGetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).PSGetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::PSGetConstantBuffers);
    ret
}
unsafe extern "C" fn IAGetInputLayout_hook(This: *mut ID3D11DeviceContextHooked, ppInputLayout: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).IAGetInputLayout)((*This).original as *mut _, ppInputLayout);
    push_back_payload(D3DPayload::IAGetInputLayout);
    ret
}
unsafe extern "C" fn IAGetVertexBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppVertexBuffers: *mut *mut c_void, pStrides: *mut u32, pOffsets: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).IAGetVertexBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets);
    push_back_payload(D3DPayload::IAGetVertexBuffers);
    ret
}
unsafe extern "C" fn IAGetIndexBuffer_hook(This: *mut ID3D11DeviceContextHooked, pIndexBuffer: *mut *mut c_void, Format: *mut DXGI_FORMAT, Offset: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).IAGetIndexBuffer)((*This).original as *mut _, pIndexBuffer, Format, Offset);
    push_back_payload(D3DPayload::IAGetIndexBuffer);
    ret
}
unsafe extern "C" fn GSGetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).GSGetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::GSGetConstantBuffers);
    ret
}
unsafe extern "C" fn GSGetShader_hook(This: *mut ID3D11DeviceContextHooked, ppGeometryShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).GSGetShader)((*This).original as *mut _, ppGeometryShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::GSGetShader);
    ret
}
unsafe extern "C" fn IAGetPrimitiveTopology_hook(This: *mut ID3D11DeviceContextHooked, pTopology: *mut D3D_PRIMITIVE_TOPOLOGY) {
    let ret = ((*(*(*This).original).vtbl).IAGetPrimitiveTopology)((*This).original as *mut _, pTopology);
    push_back_payload(D3DPayload::IAGetPrimitiveTopology);
    ret
}
unsafe extern "C" fn VSGetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).VSGetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::VSGetShaderResources);
    ret
}
unsafe extern "C" fn VSGetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).VSGetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::VSGetSamplers);
    ret
}
unsafe extern "C" fn GetPredication_hook(This: *mut ID3D11DeviceContextHooked, ppPredicate: *mut *mut c_void, pPredicateValue: *mut BOOL) {
    let ret = ((*(*(*This).original).vtbl).GetPredication)((*This).original as *mut _, ppPredicate, pPredicateValue);
    push_back_payload(D3DPayload::GetPredication);
    ret
}
unsafe extern "C" fn GSGetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).GSGetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::GSGetShaderResources);
    ret
}
unsafe extern "C" fn GSGetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).GSGetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::GSGetSamplers);
    ret
}
unsafe extern "C" fn OMGetRenderTargets_hook(This: *mut ID3D11DeviceContextHooked, NumViews: u32, ppRenderTargetViews: *mut *mut c_void, ppDepthStencilView: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).OMGetRenderTargets)((*This).original as *mut _, NumViews, ppRenderTargetViews, ppDepthStencilView);
    push_back_payload(D3DPayload::OMGetRenderTargets);
    ret
}
unsafe extern "C" fn OMGetRenderTargetsAndUnorderedAccessViews_hook(This: *mut ID3D11DeviceContextHooked, NumRTVs: u32, ppRenderTargetViews: *mut *mut c_void, ppDepthStencilView: *mut *mut c_void, UAVStartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).OMGetRenderTargetsAndUnorderedAccessViews)((*This).original as *mut _, NumRTVs, ppRenderTargetViews, ppDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews);
    push_back_payload(D3DPayload::OMGetRenderTargetsAndUnorderedAccessViews);
    ret
}
unsafe extern "C" fn OMGetBlendState_hook(This: *mut ID3D11DeviceContextHooked, ppBlendState: *mut *mut c_void, BlendFactor: *mut f32, pSampleMask: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).OMGetBlendState)((*This).original as *mut _, ppBlendState, BlendFactor, pSampleMask);
    push_back_payload(D3DPayload::OMGetBlendState);
    ret
}
unsafe extern "C" fn OMGetDepthStencilState_hook(This: *mut ID3D11DeviceContextHooked, ppDepthStencilState: *mut *mut c_void, pStencilRef: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).OMGetDepthStencilState)((*This).original as *mut _, ppDepthStencilState, pStencilRef);
    push_back_payload(D3DPayload::OMGetDepthStencilState);
    ret
}
unsafe extern "C" fn SOGetTargets_hook(This: *mut ID3D11DeviceContextHooked, NumBuffers: u32, ppSOTargets: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).SOGetTargets)((*This).original as *mut _, NumBuffers, ppSOTargets);
    push_back_payload(D3DPayload::SOGetTargets);
    ret
}
unsafe extern "C" fn RSGetState_hook(This: *mut ID3D11DeviceContextHooked, ppRasterizerState: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).RSGetState)((*This).original as *mut _, ppRasterizerState);
    push_back_payload(D3DPayload::RSGetState);
    ret
}
unsafe extern "C" fn RSGetViewports_hook(This: *mut ID3D11DeviceContextHooked, pNumViewports: *mut u32, pViewports: *mut D3D11_VIEWPORT) {
    let ret = ((*(*(*This).original).vtbl).RSGetViewports)((*This).original as *mut _, pNumViewports, pViewports);
    push_back_payload(D3DPayload::RSGetViewports);
    ret
}
unsafe extern "C" fn RSGetScissorRects_hook(This: *mut ID3D11DeviceContextHooked, pNumRects: *mut u32, pRects: *mut RECT) {
    let ret = ((*(*(*This).original).vtbl).RSGetScissorRects)((*This).original as *mut _, pNumRects, pRects);
    push_back_payload(D3DPayload::RSGetScissorRects);
    ret
}
unsafe extern "C" fn HSGetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).HSGetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::HSGetShaderResources);
    ret
}
unsafe extern "C" fn HSGetShader_hook(This: *mut ID3D11DeviceContextHooked, ppHullShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).HSGetShader)((*This).original as *mut _, ppHullShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::HSGetShader);
    ret
}
unsafe extern "C" fn HSGetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).HSGetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::HSGetSamplers);
    ret
}
unsafe extern "C" fn HSGetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).HSGetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::HSGetConstantBuffers);
    ret
}
unsafe extern "C" fn DSGetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).DSGetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::DSGetShaderResources);
    ret
}
unsafe extern "C" fn DSGetShader_hook(This: *mut ID3D11DeviceContextHooked, ppDomainShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).DSGetShader)((*This).original as *mut _, ppDomainShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::DSGetShader);
    ret
}
unsafe extern "C" fn DSGetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).DSGetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::DSGetSamplers);
    ret
}
unsafe extern "C" fn DSGetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).DSGetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::DSGetConstantBuffers);
    ret
}
unsafe extern "C" fn CSGetShaderResources_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).CSGetShaderResources)((*This).original as *mut _, StartSlot, NumViews, ppShaderResourceViews);
    push_back_payload(D3DPayload::CSGetShaderResources);
    ret
}
unsafe extern "C" fn CSGetUnorderedAccessViews_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).CSGetUnorderedAccessViews)((*This).original as *mut _, StartSlot, NumUAVs, ppUnorderedAccessViews);
    push_back_payload(D3DPayload::CSGetUnorderedAccessViews);
    ret
}
unsafe extern "C" fn CSGetShader_hook(This: *mut ID3D11DeviceContextHooked, ppComputeShader: *mut *mut c_void, ppClassInstances: *mut *mut c_void, pNumClassInstances: *mut u32) {
    let ret = ((*(*(*This).original).vtbl).CSGetShader)((*This).original as *mut _, ppComputeShader, ppClassInstances, pNumClassInstances);
    push_back_payload(D3DPayload::CSGetShader);
    ret
}
unsafe extern "C" fn CSGetSamplers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).CSGetSamplers)((*This).original as *mut _, StartSlot, NumSamplers, ppSamplers);
    push_back_payload(D3DPayload::CSGetSamplers);
    ret
}
unsafe extern "C" fn CSGetConstantBuffers_hook(This: *mut ID3D11DeviceContextHooked, StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut c_void) {
    let ret = ((*(*(*This).original).vtbl).CSGetConstantBuffers)((*This).original as *mut _, StartSlot, NumBuffers, ppConstantBuffers);
    push_back_payload(D3DPayload::CSGetConstantBuffers);
    ret
}
unsafe extern "C" fn ClearState_hook(This: *mut ID3D11DeviceContextHooked) {
    let ret = ((*(*(*This).original).vtbl).ClearState)((*This).original as *mut _, );
    push_back_payload(D3DPayload::ClearState);
    ret
}
unsafe extern "C" fn Flush_hook(This: *mut ID3D11DeviceContextHooked) {
    let ret = ((*(*(*This).original).vtbl).Flush)((*This).original as *mut _, );
    push_back_payload(D3DPayload::Flush);
    ret
}
unsafe extern "C" fn GetType_hook(This: *mut ID3D11DeviceContextHooked) -> D3D11_DEVICE_CONTEXT_TYPE {
    let ret = ((*(*(*This).original).vtbl).GetType)((*This).original as *mut _, );
    push_back_payload(D3DPayload::GetType);
    ret
}
unsafe extern "C" fn GetContextFlags_hook(This: *mut ID3D11DeviceContextHooked) -> u32 {
    let ret = ((*(*(*This).original).vtbl).GetContextFlags)((*This).original as *mut _, );
    push_back_payload(D3DPayload::GetContextFlags);
    ret
}
unsafe extern "C" fn FinishCommandList_hook(This: *mut ID3D11DeviceContextHooked, RestoreDeferredContextState: BOOL, ppCommandList: *mut *mut c_void) -> HRESULT {
    let ret = ((*(*(*This).original).vtbl).FinishCommandList)((*This).original as *mut _, RestoreDeferredContextState, ppCommandList);
    push_back_payload(D3DPayload::FinishCommandList);
    ret
}

impl ID3D11DeviceContextHooked {
    pub fn new(original: *mut c_void) -> anyhow::Result<ID3D11DeviceContextHooked> {
        Ok(ID3D11DeviceContextHooked {
            vtbl: std::ptr::null(),
            original: original as *mut _,
            vtbl_instance: ID3D11DeviceContextVtbl::<ID3D11DeviceContextHooked> {
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
            }
        })
    }

    pub fn initialize(&mut self) {
        self.vtbl = &self.vtbl_instance;
    }
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
            ID3D11DeviceContextHooked::destroy();
        }
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    use crate::game::graphics::kernel::Device;

    if !rendering::CAPTURE_D3D_COMMANDS {
        return Ok(HookState(vec![]));
    }

    let immediate_context: &'static mut _ = *Device::get().immediate_context_ptr_mut();
    let device_context_ptr_ptr = immediate_context.device_context_ptr_mut();
    ID3D11DeviceContextHooked::create((*device_context_ptr_ptr).abi())?;
    let device_context_hooked_bytes = {
        let device_context_hooked = ID3D11DeviceContextHooked::get_mut()
            .ok_or(anyhow::Error::msg("Failed to retrieve hooked instance"))?;
        device_context_hooked.initialize();

        let device_context_hooked_ptr = device_context_hooked as *mut ID3D11DeviceContextHooked;
        (device_context_hooked_ptr as usize).to_le_bytes()
    };

    let patcher = Patcher::get_mut().ok_or(anyhow::Error::msg("Failed to retrieve patcher"))?;
    let mut ptrs = vec![];
    {
        let device_context_ptr_ptr = device_context_ptr_ptr as *mut u8;
        ptrs.push(patcher.patch(device_context_ptr_ptr, &device_context_hooked_bytes));
    }
    {
        let device_context_ptr_ptr = Device::get().device_context_ptr_mut() as *mut u8;
        ptrs.push(patcher.patch(device_context_ptr_ptr, &device_context_hooked_bytes));
    }

    Ok(HookState(ptrs))
}
