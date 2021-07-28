use crate::debugger::payload::*;

use bindings::Windows::Win32::Foundation::{BOOL, RECT};
use bindings::Windows::Win32::Graphics::Direct3D11::{
    D3D11_BOX, D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, D3D11_VIEWPORT, D3D_PRIMITIVE_TOPOLOGY,
};
use bindings::Windows::Win32::Graphics::Dxgi::DXGI_FORMAT;
use windows::{Guid, IUnknown};

use std::os::raw::c_void;

use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumDiscriminants};

use cimgui as ig;

#[derive(Display, EnumDiscriminants, EnumCount, Clone)]
#[allow(dead_code)]
#[rustfmt::skip]
pub enum D3DPayload {
    QueryInterface(*const Guid, *mut *mut c_void),
    AddRef(),
    Release(),
    GetDevice(*mut *mut c_void),
    GetPrivateData(*const Guid, *mut u32, *mut c_void),
    SetPrivateData(*const Guid, u32, *mut c_void),
    SetPrivateDataInterface(*const Guid, *mut IUnknown),
    VSSetConstantBuffers(u32, u32, *mut *const c_void),
    PSSetShaderResources(u32, u32, *mut *const c_void),
    PSSetShader(*mut c_void, *mut *const c_void, u32),
    PSSetSamplers(u32, u32, *mut *const c_void),
    VSSetShader(*mut c_void, *mut *const c_void, u32),
    DrawIndexed(u32, u32, i32),
    Draw(u32, u32),
    Map(*mut c_void, u32, D3D11_MAP, u32, *mut D3D11_MAPPED_SUBRESOURCE),
    Unmap(*mut c_void, u32),
    PSSetConstantBuffers(u32, u32, *mut *const c_void),
    IASetInputLayout(*mut c_void),
    IASetVertexBuffers(u32, u32, *mut *const c_void, *mut u32, *mut u32),
    IASetIndexBuffer(*mut c_void, DXGI_FORMAT, u32),
    DrawIndexedInstanced(u32, u32, u32, i32, u32),
    DrawInstanced(u32, u32, u32, u32),
    GSSetConstantBuffers(u32, u32, *mut *const c_void),
    GSSetShader(*mut c_void, *mut *const c_void, u32),
    IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY),
    VSSetShaderResources(u32, u32, *mut *const c_void),
    VSSetSamplers(u32, u32, *mut *const c_void),
    Begin(*mut c_void),
    End(*mut c_void),
    GetData(*mut c_void, *mut c_void, u32, u32),
    SetPredication(*mut c_void, BOOL),
    GSSetShaderResources(u32, u32, *mut *const c_void),
    GSSetSamplers(u32, u32, *mut *const c_void),
    OMSetRenderTargets(u32, *mut *const c_void, *mut c_void),
    OMSetRenderTargetsAndUnorderedAccessViews(u32, *mut *const c_void, *mut c_void, u32, u32, *mut *const c_void, *mut u32),
    OMSetBlendState(*mut c_void, *mut f32, u32),
    OMSetDepthStencilState(*mut c_void, u32),
    SOSetTargets(u32, *mut *const c_void, *mut u32),
    DrawAuto(),
    DrawIndexedInstancedIndirect(*mut c_void, u32),
    DrawInstancedIndirect(*mut c_void, u32),
    Dispatch(u32, u32, u32),
    DispatchIndirect(*mut c_void, u32),
    RSSetState(*mut c_void),
    RSSetViewports(u32, *mut D3D11_VIEWPORT),
    RSSetScissorRects(u32, *mut RECT),
    CopySubresourceRegion(*mut c_void, u32, u32, u32, u32, *mut c_void, u32, *mut D3D11_BOX),
    CopyResource(*mut c_void, *mut c_void),
    UpdateSubresource(*mut c_void, u32, *mut D3D11_BOX, *mut c_void, u32, u32),
    CopyStructureCount(*mut c_void, u32, *mut c_void),
    ClearRenderTargetView(*mut c_void, *mut f32),
    ClearUnorderedAccessViewUint(*mut c_void, *mut u32),
    ClearUnorderedAccessViewFloat(*mut c_void, *mut f32),
    ClearDepthStencilView(*mut c_void, u32, f32, u8),
    GenerateMips(*mut c_void),
    SetResourceMinLOD(*mut c_void, f32),
    GetResourceMinLOD(*mut c_void),
    ResolveSubresource(*mut c_void, u32, *mut c_void, u32, DXGI_FORMAT),
    ExecuteCommandList(*mut c_void, BOOL),
    HSSetShaderResources(u32, u32, *mut *const c_void),
    HSSetShader(*mut c_void, *mut *const c_void, u32),
    HSSetSamplers(u32, u32, *mut *const c_void),
    HSSetConstantBuffers(u32, u32, *mut *const c_void),
    DSSetShaderResources(u32, u32, *mut *const c_void),
    DSSetShader(*mut c_void, *mut *const c_void, u32),
    DSSetSamplers(u32, u32, *mut *const c_void),
    DSSetConstantBuffers(u32, u32, *mut *const c_void),
    CSSetShaderResources(u32, u32, *mut *const c_void),
    CSSetUnorderedAccessViews(u32, u32, *mut *const c_void, *mut u32),
    CSSetShader(*mut c_void, *mut *const c_void, u32),
    CSSetSamplers(u32, u32, *mut *const c_void),
    CSSetConstantBuffers(u32, u32, *mut *const c_void),
    VSGetConstantBuffers(u32, u32, *mut *mut c_void),
    PSGetShaderResources(u32, u32, *mut *mut c_void),
    PSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    PSGetSamplers(u32, u32, *mut *mut c_void),
    VSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    PSGetConstantBuffers(u32, u32, *mut *mut c_void),
    IAGetInputLayout(*mut *mut c_void),
    IAGetVertexBuffers(u32, u32, *mut *mut c_void, *mut u32, *mut u32),
    IAGetIndexBuffer(*mut *mut c_void, *mut DXGI_FORMAT, *mut u32),
    GSGetConstantBuffers(u32, u32, *mut *mut c_void),
    GSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    IAGetPrimitiveTopology(*mut D3D_PRIMITIVE_TOPOLOGY),
    VSGetShaderResources(u32, u32, *mut *mut c_void),
    VSGetSamplers(u32, u32, *mut *mut c_void),
    GetPredication(*mut *mut c_void, *mut BOOL),
    GSGetShaderResources(u32, u32, *mut *mut c_void),
    GSGetSamplers(u32, u32, *mut *mut c_void),
    OMGetRenderTargets(u32, *mut *mut c_void, *mut *mut c_void),
    OMGetRenderTargetsAndUnorderedAccessViews(u32, *mut *mut c_void, *mut *mut c_void, u32, u32, *mut *mut c_void),
    OMGetBlendState(*mut *mut c_void, *mut f32, *mut u32),
    OMGetDepthStencilState(*mut *mut c_void, *mut u32),
    SOGetTargets(u32, *mut *mut c_void),
    RSGetState(*mut *mut c_void),
    RSGetViewports(*mut u32, *mut D3D11_VIEWPORT),
    RSGetScissorRects(*mut u32, *mut RECT),
    HSGetShaderResources(u32, u32, *mut *mut c_void),
    HSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    HSGetSamplers(u32, u32, *mut *mut c_void),
    HSGetConstantBuffers(u32, u32, *mut *mut c_void),
    DSGetShaderResources(u32, u32, *mut *mut c_void),
    DSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    DSGetSamplers(u32, u32, *mut *mut c_void),
    DSGetConstantBuffers(u32, u32, *mut *mut c_void),
    CSGetShaderResources(u32, u32, *mut *mut c_void),
    CSGetUnorderedAccessViews(u32, u32, *mut *mut c_void),
    CSGetShader(*mut *mut c_void, *mut *mut c_void, *mut u32),
    CSGetSamplers(u32, u32, *mut *mut c_void),
    CSGetConstantBuffers(u32, u32, *mut *mut c_void),
    ClearState(),
    Flush(),
    GetType(),
    GetContextFlags(),
    FinishCommandList(BOOL, *mut *mut c_void),
}
impl Payload for D3DPayload {
    fn title(&self) -> String {
        match self {
            _ => self.to_string(),
        }
    }

    fn colour(&self) -> ig::Color {
        let type_index = D3DPayloadDiscriminants::from(self) as u32;
        let hue = type_index as f32 / D3DPayload::COUNT as f32;
        ig::Color::from_hsv(hue, 0.6, 0.8)
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    fn draw(&self) -> anyhow::Result<()> {
        match self {
            Self::QueryInterface(riid, ppvObject) => {
                ig::bullet(); ig::textf!("riid: {:?}", riid);
                ig::bullet(); ig::textf!("ppvObject: {:?}", ppvObject);
            }
            Self::GetDevice(ppDevice) => {
                ig::bullet(); ig::textf!("ppDevice: {:?}", ppDevice);
            }
            Self::GetPrivateData(guid, pDataSize, pData) => {
                ig::bullet(); ig::textf!("guid: {:?}", guid);
                ig::bullet(); ig::textf!("pDataSize: {:?}", pDataSize);
                ig::bullet(); ig::textf!("pData: {:?}", pData);
            }
            Self::SetPrivateData(guid, DataSize, pData) => {
                ig::bullet(); ig::textf!("guid: {:?}", guid);
                ig::bullet(); ig::textf!("DataSize: {:?}", DataSize);
                ig::bullet(); ig::textf!("pData: {:?}", pData);
            }
            Self::SetPrivateDataInterface(guid, pData) => {
                ig::bullet(); ig::textf!("guid: {:?}", guid);
                ig::bullet(); ig::textf!("pData: {:?}", pData);
            }
            Self::VSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::PSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::PSSetShader(pPixelShader, ppClassInstances, NumClassInstances) => {
                ig::bullet(); ig::textf!("pPixelShader: {:?}", pPixelShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::PSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::VSSetShader(pVertexShader, ppClassInstances, NumClassInstances) => {
                ig::bullet(); ig::textf!("pVertexShader: {:?}", pVertexShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::DrawIndexed(IndexCount, StartIndexLocation, BaseVertexLocation) => {
                ig::bullet(); ig::textf!("IndexCount: {:?}", IndexCount);
                ig::bullet(); ig::textf!("StartIndexLocation: {:?}", StartIndexLocation);
                ig::bullet(); ig::textf!("BaseVertexLocation: {:?}", BaseVertexLocation);
            }
            Self::Draw(VertexCount, StartVertexLocation) => {
                ig::bullet(); ig::textf!("VertexCount: {:?}", VertexCount);
                ig::bullet(); ig::textf!("StartVertexLocation: {:?}", StartVertexLocation);
            }
            Self::Map(pResource, Subresource, MapType, MapFlags, pMappedResource) => {
                ig::bullet(); ig::textf!("pResource: {:?}", pResource);
                ig::bullet(); ig::textf!("Subresource: {:?}", Subresource);
                ig::bullet(); ig::textf!("MapType: {:?}", MapType);
                ig::bullet(); ig::textf!("MapFlags: {:?}", MapFlags);
                ig::bullet(); ig::textf!("pMappedResource: {:?}", pMappedResource);
            }
            Self::Unmap(pResource, Subresource) => {
                ig::bullet(); ig::textf!("pResource: {:?}", pResource);
                ig::bullet(); ig::textf!("Subresource: {:?}", Subresource);
            }
            Self::PSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::IASetInputLayout(pInputLayout) => {
                ig::bullet(); ig::textf!("pInputLayout: {:?}", pInputLayout);
            }
            Self::IASetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppVertexBuffers: {:?}", ppVertexBuffers);
                ig::bullet(); ig::textf!("pStrides: {:?}", pStrides);
                ig::bullet(); ig::textf!("pOffsets: {:?}", pOffsets);
            }
            Self::IASetIndexBuffer(pIndexBuffer, Format, Offset) => {
                ig::bullet(); ig::textf!("pIndexBuffer: {:?}", pIndexBuffer);
                ig::bullet(); ig::textf!("Format: {:?}", Format);
                ig::bullet(); ig::textf!("Offset: {:?}", Offset);
            }
            Self::DrawIndexedInstanced(IndexCountPerInstance, InstanceCount, StartIndexLocation, BaseVertexLocation, StartInstanceLocation) => {
                ig::bullet(); ig::textf!("IndexCountPerInstance: {:?}", IndexCountPerInstance);
                ig::bullet(); ig::textf!("InstanceCount: {:?}", InstanceCount);
                ig::bullet(); ig::textf!("StartIndexLocation: {:?}", StartIndexLocation);
                ig::bullet(); ig::textf!("BaseVertexLocation: {:?}", BaseVertexLocation);
                ig::bullet(); ig::textf!("StartInstanceLocation: {:?}", StartInstanceLocation);
            }
            Self::DrawInstanced(VertexCountPerInstance, InstanceCount, StartVertexLocation, StartInstanceLocation) => {
                ig::bullet(); ig::textf!("VertexCountPerInstance: {:?}", VertexCountPerInstance);
                ig::bullet(); ig::textf!("InstanceCount: {:?}", InstanceCount);
                ig::bullet(); ig::textf!("StartVertexLocation: {:?}", StartVertexLocation);
                ig::bullet(); ig::textf!("StartInstanceLocation: {:?}", StartInstanceLocation);
            }
            Self::GSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::GSSetShader(pShader, ppClassInstances, NumClassInstances) => {
                ig::bullet(); ig::textf!("pShader: {:?}", pShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::IASetPrimitiveTopology(Topology) => {
                ig::bullet(); ig::textf!("Topology: {:?}", Topology);
            }
            Self::VSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::VSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::Begin(pAsync) => {
                ig::bullet(); ig::textf!("pAsync: {:?}", pAsync);
            }
            Self::End(pAsync) => {
                ig::bullet(); ig::textf!("pAsync: {:?}", pAsync);
            }
            Self::GetData(pAsync, pData, DataSize, GetDataFlags) => {
                ig::bullet(); ig::textf!("pAsync: {:?}", pAsync);
                ig::bullet(); ig::textf!("pData: {:?}", pData);
                ig::bullet(); ig::textf!("DataSize: {:?}", DataSize);
                ig::bullet(); ig::textf!("GetDataFlags: {:?}", GetDataFlags);
            }
            Self::SetPredication(pPredicate, PredicateValue) => {
                ig::bullet(); ig::textf!("pPredicate: {:?}", pPredicate);
                ig::bullet(); ig::textf!("PredicateValue: {:?}", PredicateValue);
            }
            Self::GSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::GSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::OMSetRenderTargets(NumViews, ppRenderTargetViews, pDepthStencilView) => {
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppRenderTargetViews: {:?}", ppRenderTargetViews);
                ig::bullet(); ig::textf!("pDepthStencilView: {:?}", pDepthStencilView);
            }
            Self::OMSetRenderTargetsAndUnorderedAccessViews(NumRTVs, ppRenderTargetViews, pDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts) => {
                ig::bullet(); ig::textf!("NumRTVs: {:?}", NumRTVs);
                ig::bullet(); ig::textf!("ppRenderTargetViews: {:?}", ppRenderTargetViews);
                ig::bullet(); ig::textf!("pDepthStencilView: {:?}", pDepthStencilView);
                ig::bullet(); ig::textf!("UAVStartSlot: {:?}", UAVStartSlot);
                ig::bullet(); ig::textf!("NumUAVs: {:?}", NumUAVs);
                ig::bullet(); ig::textf!("ppUnorderedAccessViews: {:?}", ppUnorderedAccessViews);
                ig::bullet(); ig::textf!("pUAVInitialCounts: {:?}", pUAVInitialCounts);
            }
            Self::OMSetBlendState(pBlendState, BlendFactor, SampleMask) => {
                ig::bullet(); ig::textf!("pBlendState: {:?}", pBlendState);
                ig::bullet(); ig::textf!("BlendFactor: {:?}", BlendFactor);
                ig::bullet(); ig::textf!("SampleMask: {:?}", SampleMask);
            }
            Self::OMSetDepthStencilState(pDepthStencilState, StencilRef) => {
                ig::bullet(); ig::textf!("pDepthStencilState: {:?}", pDepthStencilState);
                ig::bullet(); ig::textf!("StencilRef: {:?}", StencilRef);
            }
            Self::SOSetTargets(NumBuffers, ppSOTargets, pOffsets) => {
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppSOTargets: {:?}", ppSOTargets);
                ig::bullet(); ig::textf!("pOffsets: {:?}", pOffsets);
            }
            Self::DrawIndexedInstancedIndirect(pBufferForArgs, AlignedByteOffsetForArgs) => {
                ig::bullet(); ig::textf!("pBufferForArgs: {:?}", pBufferForArgs);
                ig::bullet(); ig::textf!("AlignedByteOffsetForArgs: {:?}", AlignedByteOffsetForArgs);
            }
            Self::DrawInstancedIndirect(pBufferForArgs, AlignedByteOffsetForArgs) => {
                ig::bullet(); ig::textf!("pBufferForArgs: {:?}", pBufferForArgs);
                ig::bullet(); ig::textf!("AlignedByteOffsetForArgs: {:?}", AlignedByteOffsetForArgs);
            }
            Self::Dispatch(ThreadGroupCountX, ThreadGroupCountY, ThreadGroupCountZ) => {
                ig::bullet(); ig::textf!("ThreadGroupCountX: {:?}", ThreadGroupCountX);
                ig::bullet(); ig::textf!("ThreadGroupCountY: {:?}", ThreadGroupCountY);
                ig::bullet(); ig::textf!("ThreadGroupCountZ: {:?}", ThreadGroupCountZ);
            }
            Self::DispatchIndirect(pBufferForArgs, AlignedByteOffsetForArgs) => {
                ig::bullet(); ig::textf!("pBufferForArgs: {:?}", pBufferForArgs);
                ig::bullet(); ig::textf!("AlignedByteOffsetForArgs: {:?}", AlignedByteOffsetForArgs);
            }
            Self::RSSetState(pRasterizerState) => {
                ig::bullet(); ig::textf!("pRasterizerState: {:?}", pRasterizerState);
            }
            Self::RSSetViewports(NumViewports, pViewports) => {
                ig::bullet(); ig::textf!("NumViewports: {:?}", NumViewports);
                ig::bullet(); ig::textf!("pViewports: {:?}", pViewports);
            }
            Self::RSSetScissorRects(NumRects, pRects) => {
                ig::bullet(); ig::textf!("NumRects: {:?}", NumRects);
                ig::bullet(); ig::textf!("pRects: {:?}", pRects);
            }
            Self::CopySubresourceRegion(pDstResource, DstSubresource, DstX, DstY, DstZ, pSrcResource, SrcSubresource, pSrcBox) => {
                ig::bullet(); ig::textf!("pDstResource: {:?}", pDstResource);
                ig::bullet(); ig::textf!("DstSubresource: {:?}", DstSubresource);
                ig::bullet(); ig::textf!("DstX: {:?}", DstX);
                ig::bullet(); ig::textf!("DstY: {:?}", DstY);
                ig::bullet(); ig::textf!("DstZ: {:?}", DstZ);
                ig::bullet(); ig::textf!("pSrcResource: {:?}", pSrcResource);
                ig::bullet(); ig::textf!("SrcSubresource: {:?}", SrcSubresource);
                ig::bullet(); ig::textf!("pSrcBox: {:?}", pSrcBox);
            }
            Self::CopyResource(pDstResource, pSrcResource) => {
                ig::bullet(); ig::textf!("pDstResource: {:?}", pDstResource);
                ig::bullet(); ig::textf!("pSrcResource: {:?}", pSrcResource);
            }
            Self::UpdateSubresource(pDstResource, DstSubresource, pDstBox, pSrcData, SrcRowPitch, SrcDepthPitch) => {
                ig::bullet(); ig::textf!("pDstResource: {:?}", pDstResource);
                ig::bullet(); ig::textf!("DstSubresource: {:?}", DstSubresource);
                ig::bullet(); ig::textf!("pDstBox: {:?}", pDstBox);
                ig::bullet(); ig::textf!("pSrcData: {:?}", pSrcData);
                ig::bullet(); ig::textf!("SrcRowPitch: {:?}", SrcRowPitch);
                ig::bullet(); ig::textf!("SrcDepthPitch: {:?}", SrcDepthPitch);
            }
            Self::CopyStructureCount(pDstBuffer, DstAlignedByteOffset, pSrcView) => {
                ig::bullet(); ig::textf!("pDstBuffer: {:?}", pDstBuffer);
                ig::bullet(); ig::textf!("DstAlignedByteOffset: {:?}", DstAlignedByteOffset);
                ig::bullet(); ig::textf!("pSrcView: {:?}", pSrcView);
            }
            Self::ClearRenderTargetView(pRenderTargetView, ColorRGBA) => {
                ig::bullet(); ig::textf!("pRenderTargetView: {:?}", pRenderTargetView);
                ig::bullet(); ig::textf!("ColorRGBA: {:?}", ColorRGBA);
            }
            Self::ClearUnorderedAccessViewUint(pUnorderedAccessView, Values) => {
                ig::bullet(); ig::textf!("pUnorderedAccessView: {:?}", pUnorderedAccessView);
                ig::bullet(); ig::textf!("Values: {:?}", Values);
            }
            Self::ClearUnorderedAccessViewFloat(pUnorderedAccessView, Values) => {
                ig::bullet(); ig::textf!("pUnorderedAccessView: {:?}", pUnorderedAccessView);
                ig::bullet(); ig::textf!("Values: {:?}", Values);
            }
            Self::ClearDepthStencilView(pDepthStencilView, ClearFlags, Depth, Stencil) => {
                ig::bullet(); ig::textf!("pDepthStencilView: {:?}", pDepthStencilView);
                ig::bullet(); ig::textf!("ClearFlags: {:?}", ClearFlags);
                ig::bullet(); ig::textf!("Depth: {:?}", Depth);
                ig::bullet(); ig::textf!("Stencil: {:?}", Stencil);
            }
            Self::GenerateMips(pShaderResourceView) => {
                ig::bullet(); ig::textf!("pShaderResourceView: {:?}", pShaderResourceView);
            }
            Self::SetResourceMinLOD(pResource, MinLOD) => {
                ig::bullet(); ig::textf!("pResource: {:?}", pResource);
                ig::bullet(); ig::textf!("MinLOD: {:?}", MinLOD);
            }
            Self::GetResourceMinLOD(pResource) => {
                ig::bullet(); ig::textf!("pResource: {:?}", pResource);
            }
            Self::ResolveSubresource(pDstResource, DstSubresource, pSrcResource, SrcSubresource, Format) => {
                ig::bullet(); ig::textf!("pDstResource: {:?}", pDstResource);
                ig::bullet(); ig::textf!("DstSubresource: {:?}", DstSubresource);
                ig::bullet(); ig::textf!("pSrcResource: {:?}", pSrcResource);
                ig::bullet(); ig::textf!("SrcSubresource: {:?}", SrcSubresource);
                ig::bullet(); ig::textf!("Format: {:?}", Format);
            }
            Self::ExecuteCommandList(pCommandList, RestoreContextState) => {
                ig::bullet(); ig::textf!("pCommandList: {:?}", pCommandList);
                ig::bullet(); ig::textf!("RestoreContextState: {:?}", RestoreContextState);
            }
            Self::HSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::HSSetShader(pHullShader, ppClassInstances, NumClassInstances) => {
                ig::bullet(); ig::textf!("pHullShader: {:?}", pHullShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::HSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::HSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::DSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::DSSetShader(pDomainShader, ppClassInstances, NumClassInstances) => {
                ig::bullet(); ig::textf!("pDomainShader: {:?}", pDomainShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::DSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::DSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::CSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::CSSetUnorderedAccessViews(StartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumUAVs: {:?}", NumUAVs);
                ig::bullet(); ig::textf!("ppUnorderedAccessViews: {:?}", ppUnorderedAccessViews);
                ig::bullet(); ig::textf!("pUAVInitialCounts: {:?}", pUAVInitialCounts);
            }
            Self::CSSetShader(pComputeShader, ppClassInstances, NumClassInstances) => {
                ig::bullet(); ig::textf!("pComputeShader: {:?}", pComputeShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("NumClassInstances: {:?}", NumClassInstances);
            }
            Self::CSSetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::CSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::VSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::PSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::PSGetShader(ppPixelShader, ppClassInstances, pNumClassInstances) => {
                ig::bullet(); ig::textf!("ppPixelShader: {:?}", ppPixelShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::PSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::VSGetShader(ppVertexShader, ppClassInstances, pNumClassInstances) => {
                ig::bullet(); ig::textf!("ppVertexShader: {:?}", ppVertexShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::PSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::IAGetInputLayout(ppInputLayout) => {
                ig::bullet(); ig::textf!("ppInputLayout: {:?}", ppInputLayout);
            }
            Self::IAGetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppVertexBuffers: {:?}", ppVertexBuffers);
                ig::bullet(); ig::textf!("pStrides: {:?}", pStrides);
                ig::bullet(); ig::textf!("pOffsets: {:?}", pOffsets);
            }
            Self::IAGetIndexBuffer(pIndexBuffer, Format, Offset) => {
                ig::bullet(); ig::textf!("pIndexBuffer: {:?}", pIndexBuffer);
                ig::bullet(); ig::textf!("Format: {:?}", Format);
                ig::bullet(); ig::textf!("Offset: {:?}", Offset);
            }
            Self::GSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::GSGetShader(ppGeometryShader, ppClassInstances, pNumClassInstances) => {
                ig::bullet(); ig::textf!("ppGeometryShader: {:?}", ppGeometryShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::IAGetPrimitiveTopology(pTopology) => {
                ig::bullet(); ig::textf!("pTopology: {:?}", pTopology);
            }
            Self::VSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::VSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::GetPredication(ppPredicate, pPredicateValue) => {
                ig::bullet(); ig::textf!("ppPredicate: {:?}", ppPredicate);
                ig::bullet(); ig::textf!("pPredicateValue: {:?}", pPredicateValue);
            }
            Self::GSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::GSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::OMGetRenderTargets(NumViews, ppRenderTargetViews, ppDepthStencilView) => {
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppRenderTargetViews: {:?}", ppRenderTargetViews);
                ig::bullet(); ig::textf!("ppDepthStencilView: {:?}", ppDepthStencilView);
            }
            Self::OMGetRenderTargetsAndUnorderedAccessViews(NumRTVs, ppRenderTargetViews, ppDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews) => {
                ig::bullet(); ig::textf!("NumRTVs: {:?}", NumRTVs);
                ig::bullet(); ig::textf!("ppRenderTargetViews: {:?}", ppRenderTargetViews);
                ig::bullet(); ig::textf!("ppDepthStencilView: {:?}", ppDepthStencilView);
                ig::bullet(); ig::textf!("UAVStartSlot: {:?}", UAVStartSlot);
                ig::bullet(); ig::textf!("NumUAVs: {:?}", NumUAVs);
                ig::bullet(); ig::textf!("ppUnorderedAccessViews: {:?}", ppUnorderedAccessViews);
            }
            Self::OMGetBlendState(ppBlendState, BlendFactor, pSampleMask) => {
                ig::bullet(); ig::textf!("ppBlendState: {:?}", ppBlendState);
                ig::bullet(); ig::textf!("BlendFactor: {:?}", BlendFactor);
                ig::bullet(); ig::textf!("pSampleMask: {:?}", pSampleMask);
            }
            Self::OMGetDepthStencilState(ppDepthStencilState, pStencilRef) => {
                ig::bullet(); ig::textf!("ppDepthStencilState: {:?}", ppDepthStencilState);
                ig::bullet(); ig::textf!("pStencilRef: {:?}", pStencilRef);
            }
            Self::SOGetTargets(NumBuffers, ppSOTargets) => {
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppSOTargets: {:?}", ppSOTargets);
            }
            Self::RSGetState(ppRasterizerState) => {
                ig::bullet(); ig::textf!("ppRasterizerState: {:?}", ppRasterizerState);
            }
            Self::RSGetViewports(pNumViewports, pViewports) => {
                ig::bullet(); ig::textf!("pNumViewports: {:?}", pNumViewports);
                ig::bullet(); ig::textf!("pViewports: {:?}", pViewports);
            }
            Self::RSGetScissorRects(pNumRects, pRects) => {
                ig::bullet(); ig::textf!("pNumRects: {:?}", pNumRects);
                ig::bullet(); ig::textf!("pRects: {:?}", pRects);
            }
            Self::HSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::HSGetShader(ppHullShader, ppClassInstances, pNumClassInstances) => {
                ig::bullet(); ig::textf!("ppHullShader: {:?}", ppHullShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::HSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::HSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::DSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::DSGetShader(ppDomainShader, ppClassInstances, pNumClassInstances) => {
                ig::bullet(); ig::textf!("ppDomainShader: {:?}", ppDomainShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::DSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::DSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::CSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumViews: {:?}", NumViews);
                ig::bullet(); ig::textf!("ppShaderResourceViews: {:?}", ppShaderResourceViews);
            }
            Self::CSGetUnorderedAccessViews(StartSlot, NumUAVs, ppUnorderedAccessViews) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumUAVs: {:?}", NumUAVs);
                ig::bullet(); ig::textf!("ppUnorderedAccessViews: {:?}", ppUnorderedAccessViews);
            }
            Self::CSGetShader(ppComputeShader, ppClassInstances, pNumClassInstances) => {
                ig::bullet(); ig::textf!("ppComputeShader: {:?}", ppComputeShader);
                ig::bullet(); ig::textf!("ppClassInstances: {:?}", ppClassInstances);
                ig::bullet(); ig::textf!("pNumClassInstances: {:?}", pNumClassInstances);
            }
            Self::CSGetSamplers(StartSlot, NumSamplers, ppSamplers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumSamplers: {:?}", NumSamplers);
                ig::bullet(); ig::textf!("ppSamplers: {:?}", ppSamplers);
            }
            Self::CSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers) => {
                ig::bullet(); ig::textf!("StartSlot: {:?}", StartSlot);
                ig::bullet(); ig::textf!("NumBuffers: {:?}", NumBuffers);
                ig::bullet(); ig::textf!("ppConstantBuffers: {:?}", ppConstantBuffers);
            }
            Self::FinishCommandList(RestoreDeferredContextState, ppCommandList) => {
                ig::bullet(); ig::textf!("RestoreDeferredContextState: {:?}", RestoreDeferredContextState);
                ig::bullet(); ig::textf!("ppCommandList: {:?}", ppCommandList);
            }
            _ => {}
        }

        Ok(())
    }
}

pub type D3DCommand = Command<D3DPayload>;
