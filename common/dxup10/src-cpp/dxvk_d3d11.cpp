#include "dxvk_d3d11.h"

namespace dxvk
{
    // D3D11DeviceChild
    void STDMETHODCALLTYPE D3D11DeviceChild::GetDevice(ID3D11Device * *ppDevice)
    {
        return m_Original->GetDevice(ppDevice);
    }
    HRESULT STDMETHODCALLTYPE D3D11DeviceChild::GetPrivateData(REFGUID guid, UINT * pDataSize, void *pData)
    {
        return m_Original->GetPrivateData(guid, pDataSize, pData);
    }
    HRESULT STDMETHODCALLTYPE D3D11DeviceChild::SetPrivateData(REFGUID guid, UINT DataSize, const void *pData)
    {
        return m_Original->SetPrivateData(guid, DataSize, pData);
    }
    HRESULT STDMETHODCALLTYPE D3D11DeviceChild::SetPrivateDataInterface(REFGUID guid, const IUnknown *pData)
    {
        return m_Original->SetPrivateDataInterface(guid, pData);
    }

    // D3D11DepthStencilState
    void STDMETHODCALLTYPE D3D11DepthStencilState::GetDesc(D3D11_DEPTH_STENCIL_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11BlendState
    void STDMETHODCALLTYPE D3D11BlendState::GetDesc(D3D11_BLEND_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11RasterizerState
    void STDMETHODCALLTYPE D3D11RasterizerState::GetDesc(D3D11_RASTERIZER_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11Resource
    void STDMETHODCALLTYPE D3D11Resource::GetType(D3D11_RESOURCE_DIMENSION * pResourceDimension)
    {
        return m_Original->GetType(pResourceDimension);
    }
    void STDMETHODCALLTYPE D3D11Resource::SetEvictionPriority(UINT EvictionPriority)
    {
        return m_Original->SetEvictionPriority(EvictionPriority);
    }
    UINT STDMETHODCALLTYPE D3D11Resource::GetEvictionPriority(void)
    {
        return m_Original->GetEvictionPriority();
    }

    // D3D11Buffer
    void STDMETHODCALLTYPE D3D11Buffer::GetDesc(D3D11_BUFFER_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11Texture1D
    void STDMETHODCALLTYPE D3D11Texture1D::GetDesc(D3D11_TEXTURE1D_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11Texture2D
    void STDMETHODCALLTYPE D3D11Texture2D::GetDesc(D3D11_TEXTURE2D_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11Texture3D
    void STDMETHODCALLTYPE D3D11Texture3D::GetDesc(D3D11_TEXTURE3D_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11View
    void STDMETHODCALLTYPE D3D11View::GetResource(ID3D11Resource * *ppResource)
    {
        return m_Original->GetResource(ppResource);
    }

    // D3D11ShaderResourceView
    void STDMETHODCALLTYPE D3D11ShaderResourceView::GetDesc(D3D11_SHADER_RESOURCE_VIEW_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11RenderTargetView
    void STDMETHODCALLTYPE D3D11RenderTargetView::GetDesc(D3D11_RENDER_TARGET_VIEW_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11DepthStencilView
    void STDMETHODCALLTYPE D3D11DepthStencilView::GetDesc(D3D11_DEPTH_STENCIL_VIEW_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11UnorderedAccessView
    void STDMETHODCALLTYPE D3D11UnorderedAccessView::GetDesc(D3D11_UNORDERED_ACCESS_VIEW_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11SamplerState
    void STDMETHODCALLTYPE D3D11SamplerState::GetDesc(D3D11_SAMPLER_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11Asynchronous
    UINT STDMETHODCALLTYPE D3D11Asynchronous::GetDataSize(void)
    {
        return m_Original->GetDataSize();
    }

    // D3D11Query
    void STDMETHODCALLTYPE D3D11Query::GetDesc(D3D11_QUERY_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11Counter
    void STDMETHODCALLTYPE D3D11Counter::GetDesc(D3D11_COUNTER_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11ClassInstance
    void STDMETHODCALLTYPE D3D11ClassInstance::GetClassLinkage(ID3D11ClassLinkage * *ppLinkage)
    {
        return m_Original->GetClassLinkage(ppLinkage);
    }
    void STDMETHODCALLTYPE D3D11ClassInstance::GetDesc(D3D11_CLASS_INSTANCE_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }
    void STDMETHODCALLTYPE D3D11ClassInstance::GetInstanceName(LPSTR pInstanceName, SIZE_T * pBufferLength)
    {
        return m_Original->GetInstanceName(pInstanceName, pBufferLength);
    }
    void STDMETHODCALLTYPE D3D11ClassInstance::GetTypeName(LPSTR pTypeName, SIZE_T * pBufferLength)
    {
        return m_Original->GetTypeName(pTypeName, pBufferLength);
    }

    // D3D11ClassLinkage
    HRESULT STDMETHODCALLTYPE D3D11ClassLinkage::GetClassInstance(LPCSTR pClassInstanceName, UINT InstanceIndex, ID3D11ClassInstance * *ppInstance)
    {
        return m_Original->GetClassInstance(pClassInstanceName, InstanceIndex, ppInstance);
    }
    HRESULT STDMETHODCALLTYPE D3D11ClassLinkage::CreateClassInstance(LPCSTR pClassTypeName, UINT ConstantBufferOffset, UINT ConstantVectorOffset, UINT TextureOffset, UINT SamplerOffset, ID3D11ClassInstance * *ppInstance)
    {
        return m_Original->CreateClassInstance(pClassTypeName, ConstantBufferOffset, ConstantVectorOffset, TextureOffset, SamplerOffset, ppInstance);
    }

    // D3D11CommandList
    UINT STDMETHODCALLTYPE D3D11CommandList::GetContextFlags(void)
    {
        return m_Original->GetContextFlags();
    }

    // D3D11DeviceContext
    void STDMETHODCALLTYPE D3D11DeviceContext::VSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers)
    {
        return m_Original->VSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::PSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews)
    {
        return m_Original->PSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::PSSetShader(ID3D11PixelShader * pPixelShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances)
    {
        return m_Original->PSSetShader(pPixelShader, ppClassInstances, NumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::PSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers)
    {
        return m_Original->PSSetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::VSSetShader(ID3D11VertexShader * pVertexShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances)
    {
        return m_Original->VSSetShader(pVertexShader, ppClassInstances, NumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DrawIndexed(UINT IndexCount, UINT StartIndexLocation, INT BaseVertexLocation)
    {
        return m_Original->DrawIndexed(IndexCount, StartIndexLocation, BaseVertexLocation);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::Draw(UINT VertexCount, UINT StartVertexLocation)
    {
        return m_Original->Draw(VertexCount, StartVertexLocation);
    }
    HRESULT STDMETHODCALLTYPE D3D11DeviceContext::Map(ID3D11Resource * pResource, UINT Subresource, D3D11_MAP MapType, UINT MapFlags, D3D11_MAPPED_SUBRESOURCE * pMappedResource)
    {
        return m_Original->Map(pResource, Subresource, MapType, MapFlags, pMappedResource);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::Unmap(ID3D11Resource * pResource, UINT Subresource)
    {
        return m_Original->Unmap(pResource, Subresource);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::PSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers)
    {
        return m_Original->PSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::IASetInputLayout(ID3D11InputLayout * pInputLayout)
    {
        return m_Original->IASetInputLayout(pInputLayout);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::IASetVertexBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppVertexBuffers, const UINT *pStrides, const UINT *pOffsets)
    {
        return m_Original->IASetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::IASetIndexBuffer(ID3D11Buffer * pIndexBuffer, DXGI_FORMAT Format, UINT Offset)
    {
        return m_Original->IASetIndexBuffer(pIndexBuffer, Format, Offset);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DrawIndexedInstanced(UINT IndexCountPerInstance, UINT InstanceCount, UINT StartIndexLocation, INT BaseVertexLocation, UINT StartInstanceLocation)
    {
        return m_Original->DrawIndexedInstanced(IndexCountPerInstance, InstanceCount, StartIndexLocation, BaseVertexLocation, StartInstanceLocation);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DrawInstanced(UINT VertexCountPerInstance, UINT InstanceCount, UINT StartVertexLocation, UINT StartInstanceLocation)
    {
        return m_Original->DrawInstanced(VertexCountPerInstance, InstanceCount, StartVertexLocation, StartInstanceLocation);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers)
    {
        return m_Original->GSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GSSetShader(ID3D11GeometryShader * pShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances)
    {
        return m_Original->GSSetShader(pShader, ppClassInstances, NumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY Topology)
    {
        return m_Original->IASetPrimitiveTopology(Topology);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::VSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews)
    {
        return m_Original->VSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::VSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers)
    {
        return m_Original->VSSetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::Begin(ID3D11Asynchronous * pAsync)
    {
        return m_Original->Begin(pAsync);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::End(ID3D11Asynchronous * pAsync)
    {
        return m_Original->End(pAsync);
    }
    HRESULT STDMETHODCALLTYPE D3D11DeviceContext::GetData(ID3D11Asynchronous * pAsync, void *pData, UINT DataSize, UINT GetDataFlags)
    {
        return m_Original->GetData(pAsync, pData, DataSize, GetDataFlags);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::SetPredication(ID3D11Predicate * pPredicate, BOOL PredicateValue)
    {
        return m_Original->SetPredication(pPredicate, PredicateValue);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews)
    {
        return m_Original->GSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers)
    {
        return m_Original->GSSetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::OMSetRenderTargets(UINT NumViews, ID3D11RenderTargetView *const *ppRenderTargetViews, ID3D11DepthStencilView *pDepthStencilView)
    {
        return m_Original->OMSetRenderTargets(NumViews, ppRenderTargetViews, pDepthStencilView);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::OMSetRenderTargetsAndUnorderedAccessViews(UINT NumRTVs, ID3D11RenderTargetView *const *ppRenderTargetViews, ID3D11DepthStencilView *pDepthStencilView, UINT UAVStartSlot, UINT NumUAVs, ID3D11UnorderedAccessView *const *ppUnorderedAccessViews, const UINT *pUAVInitialCounts)
    {
        return m_Original->OMSetRenderTargetsAndUnorderedAccessViews(NumRTVs, ppRenderTargetViews, pDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::OMSetBlendState(ID3D11BlendState * pBlendState, const FLOAT BlendFactor[4], UINT SampleMask)
    {
        return m_Original->OMSetBlendState(pBlendState, BlendFactor, SampleMask);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::OMSetDepthStencilState(ID3D11DepthStencilState * pDepthStencilState, UINT StencilRef)
    {
        return m_Original->OMSetDepthStencilState(pDepthStencilState, StencilRef);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::SOSetTargets(UINT NumBuffers, ID3D11Buffer *const *ppSOTargets, const UINT *pOffsets)
    {
        m_SOOffsets.assign(pOffsets, pOffsets + NumBuffers);
        return m_Original->SOSetTargets(NumBuffers, ppSOTargets, pOffsets);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DrawAuto(void)
    {
        return m_Original->DrawAuto();
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DrawIndexedInstancedIndirect(ID3D11Buffer * pBufferForArgs, UINT AlignedByteOffsetForArgs)
    {
        return m_Original->DrawIndexedInstancedIndirect(pBufferForArgs, AlignedByteOffsetForArgs);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DrawInstancedIndirect(ID3D11Buffer * pBufferForArgs, UINT AlignedByteOffsetForArgs)
    {
        return m_Original->DrawInstancedIndirect(pBufferForArgs, AlignedByteOffsetForArgs);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::Dispatch(UINT ThreadGroupCountX, UINT ThreadGroupCountY, UINT ThreadGroupCountZ)
    {
        return m_Original->Dispatch(ThreadGroupCountX, ThreadGroupCountY, ThreadGroupCountZ);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DispatchIndirect(ID3D11Buffer * pBufferForArgs, UINT AlignedByteOffsetForArgs)
    {
        return m_Original->DispatchIndirect(pBufferForArgs, AlignedByteOffsetForArgs);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::RSSetState(ID3D11RasterizerState * pRasterizerState)
    {
        return m_Original->RSSetState(pRasterizerState);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::RSSetViewports(UINT NumViewports, const D3D11_VIEWPORT *pViewports)
    {
        return m_Original->RSSetViewports(NumViewports, pViewports);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::RSSetScissorRects(UINT NumRects, const D3D11_RECT *pRects)
    {
        return m_Original->RSSetScissorRects(NumRects, pRects);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CopySubresourceRegion(ID3D11Resource * pDstResource, UINT DstSubresource, UINT DstX, UINT DstY, UINT DstZ, ID3D11Resource * pSrcResource, UINT SrcSubresource, const D3D11_BOX *pSrcBox)
    {
        return m_Original->CopySubresourceRegion(pDstResource, DstSubresource, DstX, DstY, DstZ, pSrcResource, SrcSubresource, pSrcBox);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CopyResource(ID3D11Resource * pDstResource, ID3D11Resource * pSrcResource)
    {
        return m_Original->CopyResource(pDstResource, pSrcResource);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::UpdateSubresource(ID3D11Resource * pDstResource, UINT DstSubresource, const D3D11_BOX *pDstBox, const void *pSrcData, UINT SrcRowPitch, UINT SrcDepthPitch)
    {
        return m_Original->UpdateSubresource(pDstResource, DstSubresource, pDstBox, pSrcData, SrcRowPitch, SrcDepthPitch);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CopyStructureCount(ID3D11Buffer * pDstBuffer, UINT DstAlignedByteOffset, ID3D11UnorderedAccessView * pSrcView)
    {
        return m_Original->CopyStructureCount(pDstBuffer, DstAlignedByteOffset, pSrcView);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::ClearRenderTargetView(ID3D11RenderTargetView * pRenderTargetView, const FLOAT ColorRGBA[4])
    {
        return m_Original->ClearRenderTargetView(pRenderTargetView, ColorRGBA);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::ClearUnorderedAccessViewUint(ID3D11UnorderedAccessView * pUnorderedAccessView, const UINT Values[4])
    {
        return m_Original->ClearUnorderedAccessViewUint(pUnorderedAccessView, Values);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::ClearUnorderedAccessViewFloat(ID3D11UnorderedAccessView * pUnorderedAccessView, const FLOAT Values[4])
    {
        return m_Original->ClearUnorderedAccessViewFloat(pUnorderedAccessView, Values);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::ClearDepthStencilView(ID3D11DepthStencilView * pDepthStencilView, UINT ClearFlags, FLOAT Depth, UINT8 Stencil)
    {
        return m_Original->ClearDepthStencilView(pDepthStencilView, ClearFlags, Depth, Stencil);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GenerateMips(ID3D11ShaderResourceView * pShaderResourceView)
    {
        return m_Original->GenerateMips(pShaderResourceView);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::SetResourceMinLOD(ID3D11Resource * pResource, FLOAT MinLOD)
    {
        return m_Original->SetResourceMinLOD(pResource, MinLOD);
    }
    FLOAT STDMETHODCALLTYPE D3D11DeviceContext::GetResourceMinLOD(ID3D11Resource * pResource)
    {
        return m_Original->GetResourceMinLOD(pResource);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::ResolveSubresource(ID3D11Resource * pDstResource, UINT DstSubresource, ID3D11Resource * pSrcResource, UINT SrcSubresource, DXGI_FORMAT Format)
    {
        return m_Original->ResolveSubresource(pDstResource, DstSubresource, pSrcResource, SrcSubresource, Format);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::ExecuteCommandList(ID3D11CommandList * pCommandList, BOOL RestoreContextState)
    {
        return m_Original->ExecuteCommandList(pCommandList, RestoreContextState);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::HSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews)
    {
        return m_Original->HSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::HSSetShader(ID3D11HullShader * pHullShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances)
    {
        return m_Original->HSSetShader(pHullShader, ppClassInstances, NumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::HSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers)
    {
        return m_Original->HSSetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::HSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers)
    {
        return m_Original->HSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews)
    {
        return m_Original->DSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DSSetShader(ID3D11DomainShader * pDomainShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances)
    {
        return m_Original->DSSetShader(pDomainShader, ppClassInstances, NumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers)
    {
        return m_Original->DSSetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers)
    {
        return m_Original->DSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews)
    {
        return m_Original->CSSetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSSetUnorderedAccessViews(UINT StartSlot, UINT NumUAVs, ID3D11UnorderedAccessView *const *ppUnorderedAccessViews, const UINT *pUAVInitialCounts)
    {
        return m_Original->CSSetUnorderedAccessViews(StartSlot, NumUAVs, ppUnorderedAccessViews, pUAVInitialCounts);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSSetShader(ID3D11ComputeShader * pComputeShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances)
    {
        return m_Original->CSSetShader(pComputeShader, ppClassInstances, NumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers)
    {
        return m_Original->CSSetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers)
    {
        return m_Original->CSSetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::VSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers)
    {
        return m_Original->VSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::PSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews)
    {
        return m_Original->PSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::PSGetShader(ID3D11PixelShader * *ppPixelShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances)
    {
        return m_Original->PSGetShader(ppPixelShader, ppClassInstances, pNumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::PSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers)
    {
        return m_Original->PSGetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::VSGetShader(ID3D11VertexShader * *ppVertexShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances)
    {
        return m_Original->VSGetShader(ppVertexShader, ppClassInstances, pNumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::PSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers)
    {
        return m_Original->PSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::IAGetInputLayout(ID3D11InputLayout * *ppInputLayout)
    {
        return m_Original->IAGetInputLayout(ppInputLayout);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::IAGetVertexBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppVertexBuffers, UINT * pStrides, UINT * pOffsets)
    {
        return m_Original->IAGetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::IAGetIndexBuffer(ID3D11Buffer * *pIndexBuffer, DXGI_FORMAT * Format, UINT * Offset)
    {
        return m_Original->IAGetIndexBuffer(pIndexBuffer, Format, Offset);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers)
    {
        return m_Original->GSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GSGetShader(ID3D11GeometryShader * *ppGeometryShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances)
    {
        return m_Original->GSGetShader(ppGeometryShader, ppClassInstances, pNumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::IAGetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY * pTopology)
    {
        return m_Original->IAGetPrimitiveTopology(pTopology);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::VSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews)
    {
        return m_Original->VSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::VSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers)
    {
        return m_Original->VSGetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GetPredication(ID3D11Predicate * *ppPredicate, BOOL * pPredicateValue)
    {
        return m_Original->GetPredication(ppPredicate, pPredicateValue);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews)
    {
        return m_Original->GSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::GSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers)
    {
        return m_Original->GSGetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::OMGetRenderTargets(UINT NumViews, ID3D11RenderTargetView * *ppRenderTargetViews, ID3D11DepthStencilView * *ppDepthStencilView)
    {
        return m_Original->OMGetRenderTargets(NumViews, ppRenderTargetViews, ppDepthStencilView);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::OMGetRenderTargetsAndUnorderedAccessViews(UINT NumRTVs, ID3D11RenderTargetView * *ppRenderTargetViews, ID3D11DepthStencilView * *ppDepthStencilView, UINT UAVStartSlot, UINT NumUAVs, ID3D11UnorderedAccessView * *ppUnorderedAccessViews)
    {
        return m_Original->OMGetRenderTargetsAndUnorderedAccessViews(NumRTVs, ppRenderTargetViews, ppDepthStencilView, UAVStartSlot, NumUAVs, ppUnorderedAccessViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::OMGetBlendState(ID3D11BlendState * *ppBlendState, FLOAT BlendFactor[4], UINT * pSampleMask)
    {
        return m_Original->OMGetBlendState(ppBlendState, BlendFactor, pSampleMask);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::OMGetDepthStencilState(ID3D11DepthStencilState * *ppDepthStencilState, UINT * pStencilRef)
    {
        return m_Original->OMGetDepthStencilState(ppDepthStencilState, pStencilRef);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::SOGetTargets(UINT NumBuffers, ID3D11Buffer * *ppSOTargets)
    {
        return m_Original->SOGetTargets(NumBuffers, ppSOTargets);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::SOGetTargetsWithOffsets(UINT NumBuffers, ID3D11Buffer * *ppSOTargets, UINT* pOffsets)
    {
        // TODO: fix?
        if (pOffsets)
        {
            std::copy(m_SOOffsets.begin(), m_SOOffsets.end(), pOffsets);
        }
        return m_Original->SOGetTargets(NumBuffers, ppSOTargets);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::RSGetState(ID3D11RasterizerState * *ppRasterizerState)
    {
        return m_Original->RSGetState(ppRasterizerState);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::RSGetViewports(UINT * pNumViewports, D3D11_VIEWPORT * pViewports)
    {
        return m_Original->RSGetViewports(pNumViewports, pViewports);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::RSGetScissorRects(UINT * pNumRects, D3D11_RECT * pRects)
    {
        return m_Original->RSGetScissorRects(pNumRects, pRects);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::HSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews)
    {
        return m_Original->HSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::HSGetShader(ID3D11HullShader * *ppHullShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances)
    {
        return m_Original->HSGetShader(ppHullShader, ppClassInstances, pNumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::HSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers)
    {
        return m_Original->HSGetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::HSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers)
    {
        return m_Original->HSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews)
    {
        return m_Original->DSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DSGetShader(ID3D11DomainShader * *ppDomainShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances)
    {
        return m_Original->DSGetShader(ppDomainShader, ppClassInstances, pNumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers)
    {
        return m_Original->DSGetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::DSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers)
    {
        return m_Original->DSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews)
    {
        return m_Original->CSGetShaderResources(StartSlot, NumViews, ppShaderResourceViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSGetUnorderedAccessViews(UINT StartSlot, UINT NumUAVs, ID3D11UnorderedAccessView * *ppUnorderedAccessViews)
    {
        return m_Original->CSGetUnorderedAccessViews(StartSlot, NumUAVs, ppUnorderedAccessViews);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSGetShader(ID3D11ComputeShader * *ppComputeShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances)
    {
        return m_Original->CSGetShader(ppComputeShader, ppClassInstances, pNumClassInstances);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers)
    {
        return m_Original->CSGetSamplers(StartSlot, NumSamplers, ppSamplers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::CSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers)
    {
        return m_Original->CSGetConstantBuffers(StartSlot, NumBuffers, ppConstantBuffers);
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::ClearState(void)
    {
        return m_Original->ClearState();
    }
    void STDMETHODCALLTYPE D3D11DeviceContext::Flush(void)
    {
        return m_Original->Flush();
    }
    D3D11_DEVICE_CONTEXT_TYPE STDMETHODCALLTYPE D3D11DeviceContext::GetType(void)
    {
        return m_Original->GetType();
    }
    UINT STDMETHODCALLTYPE D3D11DeviceContext::GetContextFlags(void)
    {
        return m_Original->GetContextFlags();
    }
    HRESULT STDMETHODCALLTYPE D3D11DeviceContext::FinishCommandList(BOOL RestoreDeferredContextState, ID3D11CommandList * *ppCommandList)
    {
        return m_Original->FinishCommandList(RestoreDeferredContextState, ppCommandList);
    }

    // D3D11VideoDecoder
    HRESULT STDMETHODCALLTYPE D3D11VideoDecoder::GetCreationParameters(D3D11_VIDEO_DECODER_DESC * pVideoDesc, D3D11_VIDEO_DECODER_CONFIG * pConfig)
    {
        return m_Original->GetCreationParameters(pVideoDesc, pConfig);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDecoder::GetDriverHandle(HANDLE * pDriverHandle)
    {
        return m_Original->GetDriverHandle(pDriverHandle);
    }

    // D3D11VideoProcessorEnumerator
    HRESULT STDMETHODCALLTYPE D3D11VideoProcessorEnumerator::GetVideoProcessorContentDesc(D3D11_VIDEO_PROCESSOR_CONTENT_DESC * pContentDesc)
    {
        return m_Original->GetVideoProcessorContentDesc(pContentDesc);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoProcessorEnumerator::CheckVideoProcessorFormat(DXGI_FORMAT Format, UINT * pFlags)
    {
        return m_Original->CheckVideoProcessorFormat(Format, pFlags);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoProcessorEnumerator::GetVideoProcessorCaps(D3D11_VIDEO_PROCESSOR_CAPS * pCaps)
    {
        return m_Original->GetVideoProcessorCaps(pCaps);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoProcessorEnumerator::GetVideoProcessorRateConversionCaps(UINT TypeIndex, D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS * pCaps)
    {
        return m_Original->GetVideoProcessorRateConversionCaps(TypeIndex, pCaps);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoProcessorEnumerator::GetVideoProcessorCustomRate(UINT TypeIndex, UINT CustomRateIndex, D3D11_VIDEO_PROCESSOR_CUSTOM_RATE * pRate)
    {
        return m_Original->GetVideoProcessorCustomRate(TypeIndex, CustomRateIndex, pRate);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoProcessorEnumerator::GetVideoProcessorFilterRange(D3D11_VIDEO_PROCESSOR_FILTER Filter, D3D11_VIDEO_PROCESSOR_FILTER_RANGE * pRange)
    {
        return m_Original->GetVideoProcessorFilterRange(Filter, pRange);
    }

    // D3D11VideoProcessor
    void STDMETHODCALLTYPE D3D11VideoProcessor::GetContentDesc(D3D11_VIDEO_PROCESSOR_CONTENT_DESC * pDesc)
    {
        return m_Original->GetContentDesc(pDesc);
    }
    void STDMETHODCALLTYPE D3D11VideoProcessor::GetRateConversionCaps(D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS * pCaps)
    {
        return m_Original->GetRateConversionCaps(pCaps);
    }

    // D3D11AuthenticatedChannel
    HRESULT STDMETHODCALLTYPE D3D11AuthenticatedChannel::GetCertificateSize(UINT * pCertificateSize)
    {
        return m_Original->GetCertificateSize(pCertificateSize);
    }
    HRESULT STDMETHODCALLTYPE D3D11AuthenticatedChannel::GetCertificate(UINT CertificateSize, BYTE * pCertificate)
    {
        return m_Original->GetCertificate(CertificateSize, pCertificate);
    }
    void STDMETHODCALLTYPE D3D11AuthenticatedChannel::GetChannelHandle(HANDLE * pChannelHandle)
    {
        return m_Original->GetChannelHandle(pChannelHandle);
    }

    // D3D11CryptoSession
    void STDMETHODCALLTYPE D3D11CryptoSession::GetCryptoType(GUID * pCryptoType)
    {
        return m_Original->GetCryptoType(pCryptoType);
    }
    void STDMETHODCALLTYPE D3D11CryptoSession::GetDecoderProfile(GUID * pDecoderProfile)
    {
        return m_Original->GetDecoderProfile(pDecoderProfile);
    }
    HRESULT STDMETHODCALLTYPE D3D11CryptoSession::GetCertificateSize(UINT * pCertificateSize)
    {
        return m_Original->GetCertificateSize(pCertificateSize);
    }
    HRESULT STDMETHODCALLTYPE D3D11CryptoSession::GetCertificate(UINT CertificateSize, BYTE * pCertificate)
    {
        return m_Original->GetCertificate(CertificateSize, pCertificate);
    }
    void STDMETHODCALLTYPE D3D11CryptoSession::GetCryptoSessionHandle(HANDLE * pCryptoSessionHandle)
    {
        return m_Original->GetCryptoSessionHandle(pCryptoSessionHandle);
    }

    // D3D11VideoDecoderOutputView
    void STDMETHODCALLTYPE D3D11VideoDecoderOutputView::GetDesc(D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11VideoProcessorInputView
    void STDMETHODCALLTYPE D3D11VideoProcessorInputView::GetDesc(D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11VideoProcessorOutputView
    void STDMETHODCALLTYPE D3D11VideoProcessorOutputView::GetDesc(D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC * pDesc)
    {
        return m_Original->GetDesc(pDesc);
    }

    // D3D11VideoContext
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::GetDecoderBuffer(ID3D11VideoDecoder * pDecoder, D3D11_VIDEO_DECODER_BUFFER_TYPE Type, UINT * pBufferSize, void **ppBuffer)
    {
        return m_Original->GetDecoderBuffer(pDecoder, Type, pBufferSize, ppBuffer);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::ReleaseDecoderBuffer(ID3D11VideoDecoder * pDecoder, D3D11_VIDEO_DECODER_BUFFER_TYPE Type)
    {
        return m_Original->ReleaseDecoderBuffer(pDecoder, Type);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::DecoderBeginFrame(ID3D11VideoDecoder * pDecoder, ID3D11VideoDecoderOutputView * pView, UINT ContentKeySize, const void *pContentKey)
    {
        return m_Original->DecoderBeginFrame(pDecoder, pView, ContentKeySize, pContentKey);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::DecoderEndFrame(ID3D11VideoDecoder * pDecoder)
    {
        return m_Original->DecoderEndFrame(pDecoder);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::SubmitDecoderBuffers(ID3D11VideoDecoder * pDecoder, UINT NumBuffers, const D3D11_VIDEO_DECODER_BUFFER_DESC *pBufferDesc)
    {
        return m_Original->SubmitDecoderBuffers(pDecoder, NumBuffers, pBufferDesc);
    }
    APP_DEPRECATED_HRESULT STDMETHODCALLTYPE D3D11VideoContext::DecoderExtension(ID3D11VideoDecoder * pDecoder, const D3D11_VIDEO_DECODER_EXTENSION *pExtensionData)
    {
        return m_Original->DecoderExtension(pDecoder, pExtensionData);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetOutputTargetRect(ID3D11VideoProcessor * pVideoProcessor, BOOL Enable, const RECT *pRect)
    {
        return m_Original->VideoProcessorSetOutputTargetRect(pVideoProcessor, Enable, pRect);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetOutputBackgroundColor(ID3D11VideoProcessor * pVideoProcessor, BOOL YCbCr, const D3D11_VIDEO_COLOR *pColor)
    {
        return m_Original->VideoProcessorSetOutputBackgroundColor(pVideoProcessor, YCbCr, pColor);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetOutputColorSpace(ID3D11VideoProcessor * pVideoProcessor, const D3D11_VIDEO_PROCESSOR_COLOR_SPACE *pColorSpace)
    {
        return m_Original->VideoProcessorSetOutputColorSpace(pVideoProcessor, pColorSpace);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetOutputAlphaFillMode(ID3D11VideoProcessor * pVideoProcessor, D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE AlphaFillMode, UINT StreamIndex)
    {
        return m_Original->VideoProcessorSetOutputAlphaFillMode(pVideoProcessor, AlphaFillMode, StreamIndex);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetOutputConstriction(ID3D11VideoProcessor * pVideoProcessor, BOOL Enable, SIZE Size)
    {
        return m_Original->VideoProcessorSetOutputConstriction(pVideoProcessor, Enable, Size);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetOutputStereoMode(ID3D11VideoProcessor * pVideoProcessor, BOOL Enable)
    {
        return m_Original->VideoProcessorSetOutputStereoMode(pVideoProcessor, Enable);
    }
    APP_DEPRECATED_HRESULT STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetOutputExtension(ID3D11VideoProcessor * pVideoProcessor, const GUID *pExtensionGuid, UINT DataSize, void *pData)
    {
        return m_Original->VideoProcessorSetOutputExtension(pVideoProcessor, pExtensionGuid, DataSize, pData);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetOutputTargetRect(ID3D11VideoProcessor * pVideoProcessor, BOOL * Enabled, RECT * pRect)
    {
        return m_Original->VideoProcessorGetOutputTargetRect(pVideoProcessor, Enabled, pRect);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetOutputBackgroundColor(ID3D11VideoProcessor * pVideoProcessor, BOOL * pYCbCr, D3D11_VIDEO_COLOR * pColor)
    {
        return m_Original->VideoProcessorGetOutputBackgroundColor(pVideoProcessor, pYCbCr, pColor);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetOutputColorSpace(ID3D11VideoProcessor * pVideoProcessor, D3D11_VIDEO_PROCESSOR_COLOR_SPACE * pColorSpace)
    {
        return m_Original->VideoProcessorGetOutputColorSpace(pVideoProcessor, pColorSpace);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetOutputAlphaFillMode(ID3D11VideoProcessor * pVideoProcessor, D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE * pAlphaFillMode, UINT * pStreamIndex)
    {
        return m_Original->VideoProcessorGetOutputAlphaFillMode(pVideoProcessor, pAlphaFillMode, pStreamIndex);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetOutputConstriction(ID3D11VideoProcessor * pVideoProcessor, BOOL * pEnabled, SIZE * pSize)
    {
        return m_Original->VideoProcessorGetOutputConstriction(pVideoProcessor, pEnabled, pSize);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetOutputStereoMode(ID3D11VideoProcessor * pVideoProcessor, BOOL * pEnabled)
    {
        return m_Original->VideoProcessorGetOutputStereoMode(pVideoProcessor, pEnabled);
    }
    APP_DEPRECATED_HRESULT STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetOutputExtension(ID3D11VideoProcessor * pVideoProcessor, const GUID *pExtensionGuid, UINT DataSize, void *pData)
    {
        return m_Original->VideoProcessorGetOutputExtension(pVideoProcessor, pExtensionGuid, DataSize, pData);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamFrameFormat(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_FRAME_FORMAT FrameFormat)
    {
        return m_Original->VideoProcessorSetStreamFrameFormat(pVideoProcessor, StreamIndex, FrameFormat);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamColorSpace(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, const D3D11_VIDEO_PROCESSOR_COLOR_SPACE *pColorSpace)
    {
        return m_Original->VideoProcessorSetStreamColorSpace(pVideoProcessor, StreamIndex, pColorSpace);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamOutputRate(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_OUTPUT_RATE OutputRate, BOOL RepeatFrame, const DXGI_RATIONAL *pCustomRate)
    {
        return m_Original->VideoProcessorSetStreamOutputRate(pVideoProcessor, StreamIndex, OutputRate, RepeatFrame, pCustomRate);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamSourceRect(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, const RECT *pRect)
    {
        return m_Original->VideoProcessorSetStreamSourceRect(pVideoProcessor, StreamIndex, Enable, pRect);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamDestRect(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, const RECT *pRect)
    {
        return m_Original->VideoProcessorSetStreamDestRect(pVideoProcessor, StreamIndex, Enable, pRect);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamAlpha(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, FLOAT Alpha)
    {
        return m_Original->VideoProcessorSetStreamAlpha(pVideoProcessor, StreamIndex, Enable, Alpha);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamPalette(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, UINT Count, const UINT *pEntries)
    {
        return m_Original->VideoProcessorSetStreamPalette(pVideoProcessor, StreamIndex, Count, pEntries);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamPixelAspectRatio(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, const DXGI_RATIONAL *pSourceAspectRatio, const DXGI_RATIONAL *pDestinationAspectRatio)
    {
        return m_Original->VideoProcessorSetStreamPixelAspectRatio(pVideoProcessor, StreamIndex, Enable, pSourceAspectRatio, pDestinationAspectRatio);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamLumaKey(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, FLOAT Lower, FLOAT Upper)
    {
        return m_Original->VideoProcessorSetStreamLumaKey(pVideoProcessor, StreamIndex, Enable, Lower, Upper);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamStereoFormat(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, D3D11_VIDEO_PROCESSOR_STEREO_FORMAT Format, BOOL LeftViewFrame0, BOOL BaseViewFrame0, D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE FlipMode, int MonoOffset)
    {
        return m_Original->VideoProcessorSetStreamStereoFormat(pVideoProcessor, StreamIndex, Enable, Format, LeftViewFrame0, BaseViewFrame0, FlipMode, MonoOffset);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamAutoProcessingMode(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable)
    {
        return m_Original->VideoProcessorSetStreamAutoProcessingMode(pVideoProcessor, StreamIndex, Enable);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamFilter(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_FILTER Filter, BOOL Enable, int Level)
    {
        return m_Original->VideoProcessorSetStreamFilter(pVideoProcessor, StreamIndex, Filter, Enable, Level);
    }
    APP_DEPRECATED_HRESULT STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamExtension(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, const GUID *pExtensionGuid, UINT DataSize, void *pData)
    {
        return m_Original->VideoProcessorSetStreamExtension(pVideoProcessor, StreamIndex, pExtensionGuid, DataSize, pData);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamFrameFormat(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_FRAME_FORMAT * pFrameFormat)
    {
        return m_Original->VideoProcessorGetStreamFrameFormat(pVideoProcessor, StreamIndex, pFrameFormat);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamColorSpace(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_COLOR_SPACE * pColorSpace)
    {
        return m_Original->VideoProcessorGetStreamColorSpace(pVideoProcessor, StreamIndex, pColorSpace);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamOutputRate(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_OUTPUT_RATE * pOutputRate, BOOL * pRepeatFrame, DXGI_RATIONAL * pCustomRate)
    {
        return m_Original->VideoProcessorGetStreamOutputRate(pVideoProcessor, StreamIndex, pOutputRate, pRepeatFrame, pCustomRate);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamSourceRect(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, RECT * pRect)
    {
        return m_Original->VideoProcessorGetStreamSourceRect(pVideoProcessor, StreamIndex, pEnabled, pRect);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamDestRect(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, RECT * pRect)
    {
        return m_Original->VideoProcessorGetStreamDestRect(pVideoProcessor, StreamIndex, pEnabled, pRect);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamAlpha(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, FLOAT * pAlpha)
    {
        return m_Original->VideoProcessorGetStreamAlpha(pVideoProcessor, StreamIndex, pEnabled, pAlpha);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamPalette(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, UINT Count, UINT * pEntries)
    {
        return m_Original->VideoProcessorGetStreamPalette(pVideoProcessor, StreamIndex, Count, pEntries);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamPixelAspectRatio(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, DXGI_RATIONAL * pSourceAspectRatio, DXGI_RATIONAL * pDestinationAspectRatio)
    {
        return m_Original->VideoProcessorGetStreamPixelAspectRatio(pVideoProcessor, StreamIndex, pEnabled, pSourceAspectRatio, pDestinationAspectRatio);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamLumaKey(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, FLOAT * pLower, FLOAT * pUpper)
    {
        return m_Original->VideoProcessorGetStreamLumaKey(pVideoProcessor, StreamIndex, pEnabled, pLower, pUpper);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamStereoFormat(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnable, D3D11_VIDEO_PROCESSOR_STEREO_FORMAT * pFormat, BOOL * pLeftViewFrame0, BOOL * pBaseViewFrame0, D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE * pFlipMode, int *MonoOffset)
    {
        return m_Original->VideoProcessorGetStreamStereoFormat(pVideoProcessor, StreamIndex, pEnable, pFormat, pLeftViewFrame0, pBaseViewFrame0, pFlipMode, MonoOffset);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamAutoProcessingMode(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled)
    {
        return m_Original->VideoProcessorGetStreamAutoProcessingMode(pVideoProcessor, StreamIndex, pEnabled);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamFilter(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_FILTER Filter, BOOL * pEnabled, int *pLevel)
    {
        return m_Original->VideoProcessorGetStreamFilter(pVideoProcessor, StreamIndex, Filter, pEnabled, pLevel);
    }
    APP_DEPRECATED_HRESULT STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamExtension(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, const GUID *pExtensionGuid, UINT DataSize, void *pData)
    {
        return m_Original->VideoProcessorGetStreamExtension(pVideoProcessor, StreamIndex, pExtensionGuid, DataSize, pData);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorBlt(ID3D11VideoProcessor * pVideoProcessor, ID3D11VideoProcessorOutputView * pView, UINT OutputFrame, UINT StreamCount, const D3D11_VIDEO_PROCESSOR_STREAM *pStreams)
    {
        return m_Original->VideoProcessorBlt(pVideoProcessor, pView, OutputFrame, StreamCount, pStreams);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::NegotiateCryptoSessionKeyExchange(ID3D11CryptoSession * pCryptoSession, UINT DataSize, void *pData)
    {
        return m_Original->NegotiateCryptoSessionKeyExchange(pCryptoSession, DataSize, pData);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::EncryptionBlt(ID3D11CryptoSession * pCryptoSession, ID3D11Texture2D * pSrcSurface, ID3D11Texture2D * pDstSurface, UINT IVSize, void *pIV)
    {
        return m_Original->EncryptionBlt(pCryptoSession, pSrcSurface, pDstSurface, IVSize, pIV);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::DecryptionBlt(ID3D11CryptoSession * pCryptoSession, ID3D11Texture2D * pSrcSurface, ID3D11Texture2D * pDstSurface, D3D11_ENCRYPTED_BLOCK_INFO * pEncryptedBlockInfo, UINT ContentKeySize, const void *pContentKey, UINT IVSize, void *pIV)
    {
        return m_Original->DecryptionBlt(pCryptoSession, pSrcSurface, pDstSurface, pEncryptedBlockInfo, ContentKeySize, pContentKey, IVSize, pIV);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::StartSessionKeyRefresh(ID3D11CryptoSession * pCryptoSession, UINT RandomNumberSize, void *pRandomNumber)
    {
        return m_Original->StartSessionKeyRefresh(pCryptoSession, RandomNumberSize, pRandomNumber);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::FinishSessionKeyRefresh(ID3D11CryptoSession * pCryptoSession)
    {
        return m_Original->FinishSessionKeyRefresh(pCryptoSession);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::GetEncryptionBltKey(ID3D11CryptoSession * pCryptoSession, UINT KeySize, void *pReadbackKey)
    {
        return m_Original->GetEncryptionBltKey(pCryptoSession, KeySize, pReadbackKey);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::NegotiateAuthenticatedChannelKeyExchange(ID3D11AuthenticatedChannel * pChannel, UINT DataSize, void *pData)
    {
        return m_Original->NegotiateAuthenticatedChannelKeyExchange(pChannel, DataSize, pData);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::QueryAuthenticatedChannel(ID3D11AuthenticatedChannel * pChannel, UINT InputSize, const void *pInput, UINT OutputSize, void *pOutput)
    {
        return m_Original->QueryAuthenticatedChannel(pChannel, InputSize, pInput, OutputSize, pOutput);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoContext::ConfigureAuthenticatedChannel(ID3D11AuthenticatedChannel * pChannel, UINT InputSize, const void *pInput, D3D11_AUTHENTICATED_CONFIGURE_OUTPUT *pOutput)
    {
        return m_Original->ConfigureAuthenticatedChannel(pChannel, InputSize, pInput, pOutput);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorSetStreamRotation(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, D3D11_VIDEO_PROCESSOR_ROTATION Rotation)
    {
        return m_Original->VideoProcessorSetStreamRotation(pVideoProcessor, StreamIndex, Enable, Rotation);
    }
    void STDMETHODCALLTYPE D3D11VideoContext::VideoProcessorGetStreamRotation(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnable, D3D11_VIDEO_PROCESSOR_ROTATION * pRotation)
    {
        return m_Original->VideoProcessorGetStreamRotation(pVideoProcessor, StreamIndex, pEnable, pRotation);
    }

    // D3D11VideoDevice
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CreateVideoDecoder(const D3D11_VIDEO_DECODER_DESC *pVideoDesc, const D3D11_VIDEO_DECODER_CONFIG *pConfig, ID3D11VideoDecoder **ppDecoder)
    {
        return m_Original->CreateVideoDecoder(pVideoDesc, pConfig, ppDecoder);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CreateVideoProcessor(ID3D11VideoProcessorEnumerator * pEnum, UINT RateConversionIndex, ID3D11VideoProcessor * *ppVideoProcessor)
    {
        return m_Original->CreateVideoProcessor(pEnum, RateConversionIndex, ppVideoProcessor);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CreateAuthenticatedChannel(D3D11_AUTHENTICATED_CHANNEL_TYPE ChannelType, ID3D11AuthenticatedChannel * *ppAuthenticatedChannel)
    {
        return m_Original->CreateAuthenticatedChannel(ChannelType, ppAuthenticatedChannel);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CreateCryptoSession(const GUID *pCryptoType, const GUID *pDecoderProfile, const GUID *pKeyExchangeType, ID3D11CryptoSession **ppCryptoSession)
    {
        return m_Original->CreateCryptoSession(pCryptoType, pDecoderProfile, pKeyExchangeType, ppCryptoSession);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CreateVideoDecoderOutputView(ID3D11Resource * pResource, const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC *pDesc, ID3D11VideoDecoderOutputView **ppVDOVView)
    {
        return m_Original->CreateVideoDecoderOutputView(pResource, pDesc, ppVDOVView);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CreateVideoProcessorInputView(ID3D11Resource * pResource, ID3D11VideoProcessorEnumerator * pEnum, const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC *pDesc, ID3D11VideoProcessorInputView **ppVPIView)
    {
        return m_Original->CreateVideoProcessorInputView(pResource, pEnum, pDesc, ppVPIView);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CreateVideoProcessorOutputView(ID3D11Resource * pResource, ID3D11VideoProcessorEnumerator * pEnum, const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC *pDesc, ID3D11VideoProcessorOutputView **ppVPOView)
    {
        return m_Original->CreateVideoProcessorOutputView(pResource, pEnum, pDesc, ppVPOView);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CreateVideoProcessorEnumerator(const D3D11_VIDEO_PROCESSOR_CONTENT_DESC *pDesc, ID3D11VideoProcessorEnumerator **ppEnum)
    {
        return m_Original->CreateVideoProcessorEnumerator(pDesc, ppEnum);
    }
    UINT STDMETHODCALLTYPE D3D11VideoDevice::GetVideoDecoderProfileCount(void)
    {
        return m_Original->GetVideoDecoderProfileCount();
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::GetVideoDecoderProfile(UINT Index, GUID * pDecoderProfile)
    {
        return m_Original->GetVideoDecoderProfile(Index, pDecoderProfile);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CheckVideoDecoderFormat(const GUID *pDecoderProfile, DXGI_FORMAT Format, BOOL *pSupported)
    {
        return m_Original->CheckVideoDecoderFormat(pDecoderProfile, Format, pSupported);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::GetVideoDecoderConfigCount(const D3D11_VIDEO_DECODER_DESC *pDesc, UINT *pCount)
    {
        return m_Original->GetVideoDecoderConfigCount(pDesc, pCount);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::GetVideoDecoderConfig(const D3D11_VIDEO_DECODER_DESC *pDesc, UINT Index, D3D11_VIDEO_DECODER_CONFIG *pConfig)
    {
        return m_Original->GetVideoDecoderConfig(pDesc, Index, pConfig);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::GetContentProtectionCaps(const GUID *pCryptoType, const GUID *pDecoderProfile, D3D11_VIDEO_CONTENT_PROTECTION_CAPS *pCaps)
    {
        return m_Original->GetContentProtectionCaps(pCryptoType, pDecoderProfile, pCaps);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::CheckCryptoKeyExchange(const GUID *pCryptoType, const GUID *pDecoderProfile, UINT Index, GUID *pKeyExchangeType)
    {
        return m_Original->CheckCryptoKeyExchange(pCryptoType, pDecoderProfile, Index, pKeyExchangeType);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::SetPrivateData(REFGUID guid, UINT DataSize, const void *pData)
    {
        return m_Original->SetPrivateData(guid, DataSize, pData);
    }
    HRESULT STDMETHODCALLTYPE D3D11VideoDevice::SetPrivateDataInterface(REFGUID guid, const IUnknown *pData)
    {
        return m_Original->SetPrivateDataInterface(guid, pData);
    }

    // D3D11Device
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateBuffer(const D3D11_BUFFER_DESC *pDesc, const D3D11_SUBRESOURCE_DATA *pInitialData, ID3D11Buffer **ppBuffer)
    {
        return m_Original->CreateBuffer(pDesc, pInitialData, ppBuffer);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateTexture1D(const D3D11_TEXTURE1D_DESC *pDesc, const D3D11_SUBRESOURCE_DATA *pInitialData, ID3D11Texture1D **ppTexture1D)
    {
        return m_Original->CreateTexture1D(pDesc, pInitialData, ppTexture1D);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateTexture2D(const D3D11_TEXTURE2D_DESC *pDesc, const D3D11_SUBRESOURCE_DATA *pInitialData, ID3D11Texture2D **ppTexture2D)
    {
        return m_Original->CreateTexture2D(pDesc, pInitialData, ppTexture2D);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateTexture3D(const D3D11_TEXTURE3D_DESC *pDesc, const D3D11_SUBRESOURCE_DATA *pInitialData, ID3D11Texture3D **ppTexture3D)
    {
        return m_Original->CreateTexture3D(pDesc, pInitialData, ppTexture3D);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateShaderResourceView(ID3D11Resource * pResource, const D3D11_SHADER_RESOURCE_VIEW_DESC *pDesc, ID3D11ShaderResourceView **ppSRView)
    {
        return m_Original->CreateShaderResourceView(pResource, pDesc, ppSRView);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateUnorderedAccessView(ID3D11Resource * pResource, const D3D11_UNORDERED_ACCESS_VIEW_DESC *pDesc, ID3D11UnorderedAccessView **ppUAView)
    {
        return m_Original->CreateUnorderedAccessView(pResource, pDesc, ppUAView);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateRenderTargetView(ID3D11Resource * pResource, const D3D11_RENDER_TARGET_VIEW_DESC *pDesc, ID3D11RenderTargetView **ppRTView)
    {
        return m_Original->CreateRenderTargetView(pResource, pDesc, ppRTView);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateDepthStencilView(ID3D11Resource * pResource, const D3D11_DEPTH_STENCIL_VIEW_DESC *pDesc, ID3D11DepthStencilView **ppDepthStencilView)
    {
        return m_Original->CreateDepthStencilView(pResource, pDesc, ppDepthStencilView);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateInputLayout(const D3D11_INPUT_ELEMENT_DESC *pInputElementDescs, UINT NumElements, const void *pShaderBytecodeWithInputSignature, SIZE_T BytecodeLength, ID3D11InputLayout **ppInputLayout)
    {
        return m_Original->CreateInputLayout(pInputElementDescs, NumElements, pShaderBytecodeWithInputSignature, BytecodeLength, ppInputLayout);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateVertexShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11VertexShader **ppVertexShader)
    {
        return m_Original->CreateVertexShader(pShaderBytecode, BytecodeLength, pClassLinkage, ppVertexShader);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateGeometryShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11GeometryShader **ppGeometryShader)
    {
        return m_Original->CreateGeometryShader(pShaderBytecode, BytecodeLength, pClassLinkage, ppGeometryShader);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateGeometryShaderWithStreamOutput(const void *pShaderBytecode, SIZE_T BytecodeLength, const D3D11_SO_DECLARATION_ENTRY *pSODeclaration, UINT NumEntries, const UINT *pBufferStrides, UINT NumStrides, UINT RasterizedStream, ID3D11ClassLinkage *pClassLinkage, ID3D11GeometryShader **ppGeometryShader)
    {
        return m_Original->CreateGeometryShaderWithStreamOutput(pShaderBytecode, BytecodeLength, pSODeclaration, NumEntries, pBufferStrides, NumStrides, RasterizedStream, pClassLinkage, ppGeometryShader);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreatePixelShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11PixelShader **ppPixelShader)
    {
        return m_Original->CreatePixelShader(pShaderBytecode, BytecodeLength, pClassLinkage, ppPixelShader);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateHullShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11HullShader **ppHullShader)
    {
        return m_Original->CreateHullShader(pShaderBytecode, BytecodeLength, pClassLinkage, ppHullShader);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateDomainShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11DomainShader **ppDomainShader)
    {
        return m_Original->CreateDomainShader(pShaderBytecode, BytecodeLength, pClassLinkage, ppDomainShader);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateComputeShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11ComputeShader **ppComputeShader)
    {
        return m_Original->CreateComputeShader(pShaderBytecode, BytecodeLength, pClassLinkage, ppComputeShader);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateClassLinkage(ID3D11ClassLinkage * *ppLinkage)
    {
        return m_Original->CreateClassLinkage(ppLinkage);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateBlendState(const D3D11_BLEND_DESC *pBlendStateDesc, ID3D11BlendState **ppBlendState)
    {
        return m_Original->CreateBlendState(pBlendStateDesc, ppBlendState);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateDepthStencilState(const D3D11_DEPTH_STENCIL_DESC *pDepthStencilDesc, ID3D11DepthStencilState **ppDepthStencilState)
    {
        return m_Original->CreateDepthStencilState(pDepthStencilDesc, ppDepthStencilState);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateRasterizerState(const D3D11_RASTERIZER_DESC *pRasterizerDesc, ID3D11RasterizerState **ppRasterizerState)
    {
        return m_Original->CreateRasterizerState(pRasterizerDesc, ppRasterizerState);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateSamplerState(const D3D11_SAMPLER_DESC *pSamplerDesc, ID3D11SamplerState **ppSamplerState)
    {
        return m_Original->CreateSamplerState(pSamplerDesc, ppSamplerState);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateQuery(const D3D11_QUERY_DESC *pQueryDesc, ID3D11Query **ppQuery)
    {
        return m_Original->CreateQuery(pQueryDesc, ppQuery);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreatePredicate(const D3D11_QUERY_DESC *pPredicateDesc, ID3D11Predicate **ppPredicate)
    {
        return m_Original->CreatePredicate(pPredicateDesc, ppPredicate);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateCounter(const D3D11_COUNTER_DESC *pCounterDesc, ID3D11Counter **ppCounter)
    {
        return m_Original->CreateCounter(pCounterDesc, ppCounter);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CreateDeferredContext(UINT ContextFlags, ID3D11DeviceContext * *ppDeferredContext)
    {
        return m_Original->CreateDeferredContext(ContextFlags, ppDeferredContext);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::OpenSharedResource(HANDLE hResource, REFIID ReturnedInterface, void **ppResource)
    {
        return m_Original->OpenSharedResource(hResource, ReturnedInterface, ppResource);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CheckFormatSupport(DXGI_FORMAT Format, UINT * pFormatSupport)
    {
        return m_Original->CheckFormatSupport(Format, pFormatSupport);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CheckMultisampleQualityLevels(DXGI_FORMAT Format, UINT SampleCount, UINT * pNumQualityLevels)
    {
        return m_Original->CheckMultisampleQualityLevels(Format, SampleCount, pNumQualityLevels);
    }
    void STDMETHODCALLTYPE D3D11Device::CheckCounterInfo(D3D11_COUNTER_INFO * pCounterInfo)
    {
        return m_Original->CheckCounterInfo(pCounterInfo);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CheckCounter(const D3D11_COUNTER_DESC *pDesc, D3D11_COUNTER_TYPE *pType, UINT *pActiveCounters, LPSTR szName, UINT *pNameLength, LPSTR szUnits, UINT *pUnitsLength, LPSTR szDescription, UINT *pDescriptionLength)
    {
        return m_Original->CheckCounter(pDesc, pType, pActiveCounters, szName, pNameLength, szUnits, pUnitsLength, szDescription, pDescriptionLength);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::CheckFeatureSupport(D3D11_FEATURE Feature, void *pFeatureSupportData, UINT FeatureSupportDataSize)
    {
        return m_Original->CheckFeatureSupport(Feature, pFeatureSupportData, FeatureSupportDataSize);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::GetPrivateData(REFGUID guid, UINT * pDataSize, void *pData)
    {
        return m_Original->GetPrivateData(guid, pDataSize, pData);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::SetPrivateData(REFGUID guid, UINT DataSize, const void *pData)
    {
        return m_Original->SetPrivateData(guid, DataSize, pData);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::SetPrivateDataInterface(REFGUID guid, const IUnknown *pData)
    {
        return m_Original->SetPrivateDataInterface(guid, pData);
    }
    D3D_FEATURE_LEVEL STDMETHODCALLTYPE D3D11Device::GetFeatureLevel(void)
    {
        return m_Original->GetFeatureLevel();
    }
    UINT STDMETHODCALLTYPE D3D11Device::GetCreationFlags(void)
    {
        return m_Original->GetCreationFlags();
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::GetDeviceRemovedReason(void)
    {
        return m_Original->GetDeviceRemovedReason();
    }
    void STDMETHODCALLTYPE D3D11Device::GetImmediateContext(ID3D11DeviceContext * *ppImmediateContext)
    {
        return m_Original->GetImmediateContext(ppImmediateContext);
    }
    HRESULT STDMETHODCALLTYPE D3D11Device::SetExceptionMode(UINT RaiseFlags)
    {
        return m_Original->SetExceptionMode(RaiseFlags);
    }
    UINT STDMETHODCALLTYPE D3D11Device::GetExceptionMode(void)
    {
        return m_Original->GetExceptionMode();
    }

    HRESULT WINAPI D3D11CreateDevice(IDXGIAdapter *pAdapter, D3D_DRIVER_TYPE DriverType, HMODULE Software, UINT Flags, CONST D3D_FEATURE_LEVEL *pFeatureLevels, UINT FeatureLevels, UINT SDKVersion, ID3D11Device **ppDevice, D3D_FEATURE_LEVEL *pFeatureLevel, ID3D11DeviceContext **ppImmediateContext)
    {

    }
    HRESULT WINAPI D3D11CreateDeviceAndSwapChain(IDXGIAdapter *pAdapter, D3D_DRIVER_TYPE DriverType, HMODULE Software, UINT Flags, CONST D3D_FEATURE_LEVEL *pFeatureLevels, UINT FeatureLevels, UINT SDKVersion, CONST DXGI_SWAP_CHAIN_DESC *pSwapChainDesc, IDXGISwapChain **ppSwapChain, ID3D11Device **ppDevice, D3D_FEATURE_LEVEL *pFeatureLevel, ID3D11DeviceContext **ppImmediateContext)
    {

    }

} // namespace dxvk