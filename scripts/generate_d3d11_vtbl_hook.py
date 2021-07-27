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

def sanitize_fragment(fragment, dc_type_replacement):
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
    elif fragment == 'REFGUID':
        return '*const Guid'
    elif fragment == 'REFIID':
        return '*const Guid'
    elif fragment == 'FLOAT':
        return 'f32'
    elif fragment == 'void':
        return 'c_void'
    elif fragment == 'ID3D11DeviceContext':
        return dc_type_replacement
    elif fragment == 'D3D11_PRIMITIVE_TOPOLOGY':
        return 'D3D_PRIMITIVE_TOPOLOGY'
    elif fragment == 'D3D11_RECT':
        return 'RECT'
    elif fragment.startswith("ID3D11"):
        return 'c_void'
    else:
        return fragment

def cpp_type_to_rust_type(type, dc_type_replacement = 'DeviceContextType'):
    fragments = ' '.join([sanitize_fragment(fragment, dc_type_replacement) for fragment in type.split(' ')[::-1]]).split(' ')
    # if fragments[-1].startswith("ID3D11") and fragments[-1] != "ID3D11DeviceContextHooked":
    #     del fragments[-2]

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
"""

output += "struct ID3D11DeviceContextVtbl<DeviceContextType> {\n"
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
fn push_back_payload(payload: D3DPayload) {
    if let Some(debugger) = Debugger::get_mut() {
        let mut command_stream = debugger.command_stream.lock().unwrap();
        command_stream.add_d3d_command(payload).unwrap();
    }
}
"""

for function in functions:
    signature = f"unsafe extern \"C\" fn {function['name']}_hook("
    signature += ', '.join([f"{arg['name']}: {cpp_type_to_rust_type(arg['type'], 'ID3D11DeviceContextHooked')}" for arg in function['args']])

    signature += ")"
    if function['return_type'] != 'void':
        signature += f" -> {cpp_type_to_rust_type(function['return_type'])}"

    args_string = ', '.join([arg['name'] for arg in function['args'][1:]])
    output += signature + " {\n"
    output += f"    let ret = ((*(*(*This).original).vtbl).{function['name']})((*This).original as *mut _, {args_string});\n"
    output += f"    push_back_payload(D3DPayload::{function['name']});\n"
    output += f"    ret\n";
    output += "}\n"
output += """
impl ID3D11DeviceContextHooked {
    pub fn new(original: *mut c_void) -> anyhow::Result<ID3D11DeviceContextHooked> {
        Ok(ID3D11DeviceContextHooked {
            vtbl: std::ptr::null(),
            original: original as *mut _,
            vtbl_instance: ID3D11DeviceContextVtbl::<ID3D11DeviceContextHooked> {
"""
for function in functions:
    output += f"                {function['name']}: {function['name']}_hook,\n"
output += """            }
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
"""

pathlib.Path("native/src/hooks/graphics/d3d/device_context.rs").write_text(output)