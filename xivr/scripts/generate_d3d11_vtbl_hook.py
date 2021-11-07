import re
import pathlib

device_context_vtbl = """
HRESULT(*QueryInterface)(ID3D11DeviceContext *This, REFIID riid, void **ppvObject);
ULONG(*AddRef)(ID3D11DeviceContext *This);
ULONG(*Release)(ID3D11DeviceContext *This);
void(*GetDevice)(ID3D11DeviceContext *This, ID3D11Device **ppDevice);
HRESULT(*GetPrivateData)(ID3D11DeviceContext *This, REFGUID guid, UINT *pDataSize, void *pData);
HRESULT(*SetPrivateData)(ID3D11DeviceContext *This, REFGUID guid, UINT DataSize, const void *pData);
HRESULT(*SetPrivateDataInterface)(ID3D11DeviceContext *This, REFGUID guid, const IUnknown *pData);
void(*VSSetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
void(*PSSetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
void(*PSSetShader)(ID3D11DeviceContext *This, ID3D11PixelShader *pPixelShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
void(*PSSetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
void(*VSSetShader)(ID3D11DeviceContext *This, ID3D11VertexShader *pVertexShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
void(*DrawIndexed)(ID3D11DeviceContext *This, UINT IndexCount, UINT StartIndexLocation, INT BaseVertexLocation);
void(*Draw)(ID3D11DeviceContext *This, UINT VertexCount, UINT StartVertexLocation);
HRESULT(*Map)(ID3D11DeviceContext *This, ID3D11Resource *pResource, UINT Subresource, D3D11_MAP MapType, UINT MapFlags, D3D11_MAPPED_SUBRESOURCE *pMappedResource);
void(*Unmap)(ID3D11DeviceContext *This, ID3D11Resource *pResource, UINT Subresource);
void(*PSSetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
void(*IASetInputLayout)(ID3D11DeviceContext *This, ID3D11InputLayout *pInputLayout);
void(*IASetVertexBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppVertexBuffers, const UINT *pStrides, const UINT *pOffsets);
void(*IASetIndexBuffer)(ID3D11DeviceContext *This, ID3D11Buffer *pIndexBuffer, DXGI_FORMAT Format, UINT Offset);
void(*DrawIndexedInstanced)(ID3D11DeviceContext *This, UINT IndexCountPerInstance, UINT InstanceCount, UINT StartIndexLocation, INT BaseVertexLocation, UINT StartInstanceLocation);
void(*DrawInstanced)(ID3D11DeviceContext *This, UINT VertexCountPerInstance, UINT InstanceCount, UINT StartVertexLocation, UINT StartInstanceLocation);
void(*GSSetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
void(*GSSetShader)(ID3D11DeviceContext *This, ID3D11GeometryShader *pShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
void(*IASetPrimitiveTopology)(ID3D11DeviceContext *This, D3D11_PRIMITIVE_TOPOLOGY Topology);
void(*VSSetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
void(*VSSetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
void(*Begin)(ID3D11DeviceContext *This, ID3D11Asynchronous *pAsync);
void(*End)(ID3D11DeviceContext *This, ID3D11Asynchronous *pAsync);
HRESULT(*GetData)(ID3D11DeviceContext *This, ID3D11Asynchronous *pAsync, void *pData, UINT DataSize, UINT GetDataFlags);
void(*SetPredication)(ID3D11DeviceContext *This, ID3D11Predicate *pPredicate, BOOL PredicateValue);
void(*GSSetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
void(*GSSetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
void(*OMSetRenderTargets)(ID3D11DeviceContext *This, UINT NumViews, ID3D11RenderTargetView *const *ppRenderTargetViews, ID3D11DepthStencilView *pDepthStencilView);
void(*OMSetRenderTargetsAndUnorderedAccessViews)(ID3D11DeviceContext *This, UINT NumRTVs, ID3D11RenderTargetView *const *ppRenderTargetViews, ID3D11DepthStencilView *pDepthStencilView, UINT UAVStartSlot, UINT NumUAVs, ID3D11UnorderedAccessView *const *ppUnorderedAccessViews, const UINT *pUAVInitialCounts);
void(*OMSetBlendState)(ID3D11DeviceContext *This, ID3D11BlendState *pBlendState, const FLOAT BlendFactor[4], UINT SampleMask);
void(*OMSetDepthStencilState)(ID3D11DeviceContext *This, ID3D11DepthStencilState *pDepthStencilState, UINT StencilRef);
void(*SOSetTargets)(ID3D11DeviceContext *This, UINT NumBuffers, ID3D11Buffer *const *ppSOTargets, const UINT *pOffsets);
void(*DrawAuto)(ID3D11DeviceContext *This);
void(*DrawIndexedInstancedIndirect)(ID3D11DeviceContext *This, ID3D11Buffer *pBufferForArgs, UINT AlignedByteOffsetForArgs);
void(*DrawInstancedIndirect)(ID3D11DeviceContext *This, ID3D11Buffer *pBufferForArgs, UINT AlignedByteOffsetForArgs);
void(*Dispatch)(ID3D11DeviceContext *This, UINT ThreadGroupCountX, UINT ThreadGroupCountY, UINT ThreadGroupCountZ);
void(*DispatchIndirect)(ID3D11DeviceContext *This, ID3D11Buffer *pBufferForArgs, UINT AlignedByteOffsetForArgs);
void(*RSSetState)(ID3D11DeviceContext *This, ID3D11RasterizerState *pRasterizerState);
void(*RSSetViewports)(ID3D11DeviceContext *This, UINT NumViewports, const D3D11_VIEWPORT *pViewports);
void(*RSSetScissorRects)(ID3D11DeviceContext *This, UINT NumRects, const D3D11_RECT *pRects);
void(*CopySubresourceRegion)(ID3D11DeviceContext *This, ID3D11Resource *pDstResource, UINT DstSubresource, UINT DstX, UINT DstY, UINT DstZ, ID3D11Resource *pSrcResource, UINT SrcSubresource, const D3D11_BOX *pSrcBox);
void(*CopyResource)(ID3D11DeviceContext *This, ID3D11Resource *pDstResource, ID3D11Resource *pSrcResource);
void(*UpdateSubresource)(ID3D11DeviceContext *This, ID3D11Resource *pDstResource, UINT DstSubresource, const D3D11_BOX *pDstBox, const void *pSrcData, UINT SrcRowPitch, UINT SrcDepthPitch);
void(*CopyStructureCount)(ID3D11DeviceContext *This, ID3D11Buffer *pDstBuffer, UINT DstAlignedByteOffset, ID3D11UnorderedAccessView *pSrcView);
void(*ClearRenderTargetView)(ID3D11DeviceContext *This, ID3D11RenderTargetView *pRenderTargetView, const FLOAT ColorRGBA[4]);
void(*ClearUnorderedAccessViewUint)(ID3D11DeviceContext *This, ID3D11UnorderedAccessView *pUnorderedAccessView, const UINT Values[4]);
void(*ClearUnorderedAccessViewFloat)(ID3D11DeviceContext *This, ID3D11UnorderedAccessView *pUnorderedAccessView, const FLOAT Values[4]);
void(*ClearDepthStencilView)(ID3D11DeviceContext *This, ID3D11DepthStencilView *pDepthStencilView, UINT ClearFlags, FLOAT Depth, UINT8 Stencil);
void(*GenerateMips)(ID3D11DeviceContext *This, ID3D11ShaderResourceView *pShaderResourceView);
void(*SetResourceMinLOD)(ID3D11DeviceContext *This, ID3D11Resource *pResource, FLOAT MinLOD);
FLOAT(*GetResourceMinLOD)(ID3D11DeviceContext *This, ID3D11Resource *pResource);
void(*ResolveSubresource)(ID3D11DeviceContext *This, ID3D11Resource *pDstResource, UINT DstSubresource, ID3D11Resource *pSrcResource, UINT SrcSubresource, DXGI_FORMAT Format);
void(*ExecuteCommandList)(ID3D11DeviceContext *This, ID3D11CommandList *pCommandList, BOOL RestoreContextState);
void(*HSSetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
void(*HSSetShader)(ID3D11DeviceContext *This, ID3D11HullShader *pHullShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
void(*HSSetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
void(*HSSetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
void(*DSSetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
void(*DSSetShader)(ID3D11DeviceContext *This, ID3D11DomainShader *pDomainShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
void(*DSSetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
void(*DSSetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
void(*CSSetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
void(*CSSetUnorderedAccessViews)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumUAVs, ID3D11UnorderedAccessView *const *ppUnorderedAccessViews, const UINT *pUAVInitialCounts);
void(*CSSetShader)(ID3D11DeviceContext *This, ID3D11ComputeShader *pComputeShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
void(*CSSetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
void(*CSSetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
void(*VSGetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers);
void(*PSGetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView **ppShaderResourceViews);
void(*PSGetShader)(ID3D11DeviceContext *This, ID3D11PixelShader **ppPixelShader, ID3D11ClassInstance **ppClassInstances, UINT *pNumClassInstances);
void(*PSGetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState **ppSamplers);
void(*VSGetShader)(ID3D11DeviceContext *This, ID3D11VertexShader **ppVertexShader, ID3D11ClassInstance **ppClassInstances, UINT *pNumClassInstances);
void(*PSGetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers);
void(*IAGetInputLayout)(ID3D11DeviceContext *This, ID3D11InputLayout **ppInputLayout);
void(*IAGetVertexBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppVertexBuffers, UINT *pStrides, UINT *pOffsets);
void(*IAGetIndexBuffer)(ID3D11DeviceContext *This, ID3D11Buffer **pIndexBuffer, DXGI_FORMAT *Format, UINT *Offset);
void(*GSGetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers);
void(*GSGetShader)(ID3D11DeviceContext *This, ID3D11GeometryShader **ppGeometryShader, ID3D11ClassInstance **ppClassInstances, UINT *pNumClassInstances);
void(*IAGetPrimitiveTopology)(ID3D11DeviceContext *This, D3D11_PRIMITIVE_TOPOLOGY *pTopology);
void(*VSGetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView **ppShaderResourceViews);
void(*VSGetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState **ppSamplers);
void(*GetPredication)(ID3D11DeviceContext *This, ID3D11Predicate **ppPredicate, BOOL *pPredicateValue);
void(*GSGetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView **ppShaderResourceViews);
void(*GSGetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState **ppSamplers);
void(*OMGetRenderTargets)(ID3D11DeviceContext *This, UINT NumViews, ID3D11RenderTargetView **ppRenderTargetViews, ID3D11DepthStencilView **ppDepthStencilView);
void(*OMGetRenderTargetsAndUnorderedAccessViews)(ID3D11DeviceContext *This, UINT NumRTVs, ID3D11RenderTargetView **ppRenderTargetViews, ID3D11DepthStencilView **ppDepthStencilView, UINT UAVStartSlot, UINT NumUAVs, ID3D11UnorderedAccessView **ppUnorderedAccessViews);
void(*OMGetBlendState)(ID3D11DeviceContext *This, ID3D11BlendState **ppBlendState, FLOAT BlendFactor[4], UINT *pSampleMask);
void(*OMGetDepthStencilState)(ID3D11DeviceContext *This, ID3D11DepthStencilState **ppDepthStencilState, UINT *pStencilRef);
void(*SOGetTargets)(ID3D11DeviceContext *This, UINT NumBuffers, ID3D11Buffer **ppSOTargets);
void(*RSGetState)(ID3D11DeviceContext *This, ID3D11RasterizerState **ppRasterizerState);
void(*RSGetViewports)(ID3D11DeviceContext *This, UINT *pNumViewports, D3D11_VIEWPORT *pViewports);
void(*RSGetScissorRects)(ID3D11DeviceContext *This, UINT *pNumRects, D3D11_RECT *pRects);
void(*HSGetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView **ppShaderResourceViews);
void(*HSGetShader)(ID3D11DeviceContext *This, ID3D11HullShader **ppHullShader, ID3D11ClassInstance **ppClassInstances, UINT *pNumClassInstances);
void(*HSGetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState **ppSamplers);
void(*HSGetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers);
void(*DSGetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView **ppShaderResourceViews);
void(*DSGetShader)(ID3D11DeviceContext *This, ID3D11DomainShader **ppDomainShader, ID3D11ClassInstance **ppClassInstances, UINT *pNumClassInstances);
void(*DSGetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState **ppSamplers);
void(*DSGetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers);
void(*CSGetShaderResources)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView **ppShaderResourceViews);
void(*CSGetUnorderedAccessViews)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumUAVs, ID3D11UnorderedAccessView **ppUnorderedAccessViews);
void(*CSGetShader)(ID3D11DeviceContext *This, ID3D11ComputeShader **ppComputeShader, ID3D11ClassInstance **ppClassInstances, UINT *pNumClassInstances);
void(*CSGetSamplers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumSamplers, ID3D11SamplerState **ppSamplers);
void(*CSGetConstantBuffers)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers);
void(*ClearState)(ID3D11DeviceContext *This);
void(*Flush)(ID3D11DeviceContext *This);
D3D11_DEVICE_CONTEXT_TYPE(*GetType)(ID3D11DeviceContext *This);
UINT(*GetContextFlags)(ID3D11DeviceContext *This);
HRESULT(*FinishCommandList)(ID3D11DeviceContext *This, BOOL RestoreDeferredContextState, ID3D11CommandList **ppCommandList);
void(*CopySubresourceRegion1)(ID3D11DeviceContext *This, ID3D11Resource *pDstResource, UINT DstSubresource, UINT DstX, UINT DstY, UINT DstZ, ID3D11Resource *pSrcResource, UINT SrcSubresource, const D3D11_BOX *pSrcBox, UINT CopyFlags);
void(*UpdateSubresource1)(ID3D11DeviceContext *This, ID3D11Resource *pDstResource, UINT DstSubresource, const D3D11_BOX *pDstBox, const void *pSrcData, UINT SrcRowPitch, UINT SrcDepthPitch, UINT CopyFlags);
void(*DiscardResource)(ID3D11DeviceContext *This, ID3D11Resource *pResource);
void(*DiscardView)(ID3D11DeviceContext *This, ID3D11View *pResourceView);
void(*VSSetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers, const UINT *pFirstConstant, const UINT *pNumConstants);
void(*HSSetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers, const UINT *pFirstConstant, const UINT *pNumConstants);
void(*DSSetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers, const UINT *pFirstConstant, const UINT *pNumConstants);
void(*GSSetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers, const UINT *pFirstConstant, const UINT *pNumConstants);
void(*PSSetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers, const UINT *pFirstConstant, const UINT *pNumConstants);
void(*CSSetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers, const UINT *pFirstConstant, const UINT *pNumConstants);
void(*VSGetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers, UINT *pFirstConstant, UINT *pNumConstants);
void(*HSGetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers, UINT *pFirstConstant, UINT *pNumConstants);
void(*DSGetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers, UINT *pFirstConstant, UINT *pNumConstants);
void(*GSGetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers, UINT *pFirstConstant, UINT *pNumConstants);
void(*PSGetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers, UINT *pFirstConstant, UINT *pNumConstants);
void(*CSGetConstantBuffers1)(ID3D11DeviceContext *This, UINT StartSlot, UINT NumBuffers, ID3D11Buffer **ppConstantBuffers, UINT *pFirstConstant, UINT *pNumConstants);
void(*SwapDeviceContextState)(ID3D11DeviceContext *This, ID3DDeviceContextState *pState, ID3DDeviceContextState **ppPreviousState);
void(*ClearView)(ID3D11DeviceContext *This, ID3D11View *pView, const FLOAT Color[4], const D3D11_RECT *pRect, UINT NumRects);
void(*DiscardView1)(ID3D11DeviceContext *This, ID3D11View *pResourceView, const D3D11_RECT *pRects, UINT NumRects);
HRESULT(*UpdateTileMappings)(ID3D11DeviceContext *This, ID3D11Resource *pTiledResource, UINT NumTiledResourceRegions, const D3D11_TILED_RESOURCE_COORDINATE *pTiledResourceRegionStartCoordinates, const D3D11_TILE_REGION_SIZE *pTiledResourceRegionSizes, ID3D11Buffer *pTilePool, UINT NumRanges, const UINT *pRangeFlags, const UINT *pTilePoolStartOffsets, const UINT *pRangeTileCounts, UINT Flags);
HRESULT(*CopyTileMappings)(ID3D11DeviceContext *This, ID3D11Resource *pDestTiledResource, const D3D11_TILED_RESOURCE_COORDINATE *pDestRegionStartCoordinate, ID3D11Resource *pSourceTiledResource, const D3D11_TILED_RESOURCE_COORDINATE *pSourceRegionStartCoordinate, const D3D11_TILE_REGION_SIZE *pTileRegionSize, UINT Flags);
void(*CopyTiles)(ID3D11DeviceContext *This, ID3D11Resource *pTiledResource, const D3D11_TILED_RESOURCE_COORDINATE *pTileRegionStartCoordinate, const D3D11_TILE_REGION_SIZE *pTileRegionSize, ID3D11Buffer *pBuffer, UINT64 BufferStartOffsetInBytes, UINT Flags);
void(*UpdateTiles)(ID3D11DeviceContext *This, ID3D11Resource *pDestTiledResource, const D3D11_TILED_RESOURCE_COORDINATE *pDestTileRegionStartCoordinate, const D3D11_TILE_REGION_SIZE *pDestTileRegionSize, const void *pSourceTileData, UINT Flags);
HRESULT(*ResizeTilePool)(ID3D11DeviceContext *This, ID3D11Buffer *pTilePool, UINT64 NewSizeInBytes);
void(*TiledResourceBarrier)(ID3D11DeviceContext *This, ID3D11DeviceChild *pTiledResourceOrViewAccessBeforeBarrier, ID3D11DeviceChild *pTiledResourceOrViewAccessAfterBarrier);
BOOL(*IsAnnotationEnabled)(ID3D11DeviceContext *This);
void(*SetMarkerInt)(ID3D11DeviceContext *This, LPCWSTR pLabel, INT Data);
void(*BeginEventInt)(ID3D11DeviceContext *This, LPCWSTR pLabel, INT Data);
void(*EndEvent)(ID3D11DeviceContext *This);
void(*Flush1)(ID3D11DeviceContext *This, D3D11_CONTEXT_TYPE ContextType, HANDLE hEvent);
void(*SetHardwareProtectionState)(ID3D11DeviceContext *This, BOOL HwProtectionEnable);
void(*GetHardwareProtectionState)(ID3D11DeviceContext *This, BOOL *pHwProtectionEnable);
"""

signature_regex = re.compile('(.*?)' + '\(\*(.*?)\)' + '\((.*?)\)')
arg_regex = re.compile('(.*?)([A-z0-9]+)$')
array_regex = re.compile('([A-z0-9]+)\[[0-9]+\]')

def process_arg(arg):
    (type, name) = [s.strip() for s in arg_regex.findall(arg)[0]]
    array_match = array_regex.findall(name)
    if len(array_match) > 0:
        name = array_match[0]
        type += " *"

    return {'type': type, 'name': name}

def sanitize_fragment(fragment):
    if fragment == '*':
        return '*mut'
    if fragment == '**':
        return '*mut *mut'
    elif fragment == 'UINT8':
        return 'u8'
    elif fragment == 'INT':
        return 'i32'
    elif fragment == 'UINT':
        return 'u32'
    elif fragment == 'ULONG':
        return 'u32'
    elif fragment == 'UINT64':
        return 'u64'
    elif fragment == 'REFGUID':
        return '*const GUID'
    elif fragment == 'REFIID':
        return '*const GUID'
    elif fragment == 'LPCWSTR':
        return '*const PWSTR'
    elif fragment == 'FLOAT':
        return 'f32'
    elif fragment == 'void':
        return 'c_void'
    elif fragment == 'D3D11_PRIMITIVE_TOPOLOGY':
        return 'D3D_PRIMITIVE_TOPOLOGY'
    elif fragment == 'D3D11_RECT':
        return 'RECT'
    elif fragment.startswith("ID3D"):
        return 'c_void'
    else:
        return fragment

def cpp_type_to_rust_type(type):
    fragments = ' '.join([sanitize_fragment(fragment) for fragment in type.split(' ')[::-1]]).split(' ')

    if fragments[-1] == 'const':
        fragments.pop()

    return ' '.join(fragments)

functions = []
for function in device_context_vtbl.splitlines():
    matches = signature_regex.findall(function)
    if len(matches) == 0:
        continue
    matches = matches[0]

    [return_type, name, args_str] = matches
    args = [process_arg(arg) for arg in args_str.split(', ')]
    functions.append({
        'return_type': return_type,
        'name': name,
        'args': args
    })

output = """#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::debugger::Debugger;
use crate::debugger::d3d_payload::D3DPayload;
use crate::hooks::Patcher;
use crate::ct_config::*;

use windows::Win32::Graphics::Direct3D11::{
    D3D_PRIMITIVE_TOPOLOGY, D3D11_MAPPED_SUBRESOURCE, D3D11_MAP, D3D11_VIEWPORT, D3D11_BOX,
    D3D11_DEVICE_CONTEXT_TYPE, D3D11_TILED_RESOURCE_COORDINATE, D3D11_TILE_REGION_SIZE,
    D3D11_CONTEXT_TYPE,
};
use windows::Win32::Graphics::Dxgi::{DXGI_FORMAT};
use windows::Win32::Foundation::{BOOL, RECT, HANDLE, PWSTR};

use windows::*;
use std::os::raw::c_void;

struct ID3D11DeviceContext {
    vtbl: *const ID3D11DeviceContextVtbl,
}

"""

output += "struct ID3D11DeviceContextVtbl {\n"
for function in functions:
    function_decl = f"    {function['name']}: unsafe extern \"C\" fn("
    function_decl += ', '.join([f"{cpp_type_to_rust_type(arg['type'])}" for arg in function['args']])

    function_decl += ")"
    if function['return_type'] != 'void':
        function_decl += f" -> {cpp_type_to_rust_type(function['return_type'])}"

    output += function_decl
    output += ",\n"
output += "}\n"

output += """
static mut ORIGINAL_VTABLE: Option<*const ID3D11DeviceContextVtbl> = None;
static HOOKED_VTABLE: ID3D11DeviceContextVtbl = ID3D11DeviceContextVtbl {
"""
for function in functions:
    output += f"    {function['name']}: {function['name']}_hook,\n"
output += "};\n"

output += """
fn push_back_payload(payload: D3DPayload) {
    if let Some(debugger) = Debugger::get_mut() {
        let mut command_stream = debugger.command_stream.lock().unwrap();
        command_stream.add_d3d_command(payload).unwrap();
    }
}
"""

for function in functions:
    signature = f"unsafe extern \"C\" fn {function['name']}_hook("
    signature += ', '.join([f"{arg['name']}: {cpp_type_to_rust_type(arg['type'])}" for arg in function['args']])

    signature += ")"
    if function['return_type'] != 'void':
        signature += f" -> {cpp_type_to_rust_type(function['return_type'])}"

    args_string = ', '.join([arg['name'] for arg in function['args'][1:]])
    output += signature + " {\n"
    output += f"    let ret = ((*ORIGINAL_VTABLE.unwrap()).{function['name']})(This, {args_string});\n"
    output += f"    push_back_payload(D3DPayload::{function['name']}({args_string}));\n"
    output += f"    ret\n";
    output += "}\n"
output += """
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
    let device_context_ptr = unsafe { std::mem::transmute(device_context) }, as *mut ID3D11DeviceContext;
    let device_context_vtable_ptr = std::ptr::addr_of_mut!((*device_context_ptr).vtbl);
    ORIGINAL_VTABLE = Some(*device_context_vtable_ptr);
    let device_context_new_vtable_ptr_bytes = (std::ptr::addr_of!(HOOKED_VTABLE) as usize).to_le_bytes();

    let patcher = Patcher::get_mut().ok_or(anyhow::Error::msg("Failed to retrieve patcher"))?;
    let ptrs = vec![
        patcher.patch(device_context_vtable_ptr as *mut u8, &device_context_new_vtable_ptr_bytes)
    ];

    Ok(HookState(ptrs))
}
"""


PRINT_ENUM = False
PRINT_MATCH_CASES = False
if PRINT_ENUM:
    print("""#[derive(Display, EnumDiscriminants, EnumCount, Clone)]
#[allow(dead_code)]
#[rustfmt::skip]
pub enum D3DPayload {""")
    for function in functions:
        args_string = ', '.join([cpp_type_to_rust_type(arg['type']) for arg in function['args'][1:]])
        print(f"\t{function['name']}({args_string}),")
    print("}")

if PRINT_MATCH_CASES:
    print("match self {")
    for function in functions:
        if len(function['args']) == 1:
            continue

        args_string = ', '.join([arg['name'] for arg in function['args'][1:]])
        print(f"\tSelf::{function['name']}({args_string}) => {{")
        for arg in function['args'][1:]:
            print(f"\t\tig::bulletf!(\"{arg['name']}: {{:?}}\", {arg['name']});")
        print("\t}")
    print("\t_ => {}")
    print("}")

pathlib.Path("native/src/hooks/graphics/d3d/device_context.rs").write_text(output)