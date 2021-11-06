#pragma once

#include <combaseapi.h>
#include <d3d11.h>
#include <minwindef.h>

#include "d3d10/d3d10_texture.h"
#include "d3d10/d3d10_shader.h"

namespace dxvk
{
    class D3D11DeviceChild;
    class D3D11DepthStencilState;
    class D3D11BlendState;
    class D3D11RasterizerState;
    class D3D11Resource;
    class D3D11Buffer;
    class D3D11Texture1D;
    class D3D11Texture2D;
    class D3D11Texture3D;
    class D3D11View;
    class D3D11ShaderResourceView;
    class D3D11RenderTargetView;
    class D3D11DepthStencilView;
    class D3D11UnorderedAccessView;
    class D3D11InputLayout;
    class D3D11SamplerState;
    class D3D11Asynchronous;
    class D3D11Query;
    class D3D11Predicate;
    class D3D11Counter;
    class D3D11ClassInstance;
    class D3D11ClassLinkage;
    class D3D11CommandList;
    class D3D11DeviceContext;
    class D3D11VideoDecoder;
    class D3D11VideoProcessorEnumerator;
    class D3D11VideoProcessor;
    class D3D11AuthenticatedChannel;
    class D3D11CryptoSession;
    class D3D11VideoDecoderOutputView;
    class D3D11VideoProcessorInputView;
    class D3D11VideoProcessorOutputView;
    class D3D11VideoContext;
    class D3D11VideoDevice;
    class D3D11Device;

    class D3D10DepthStencilState;
    class D3D10BlendState;
    class D3D10RasterizerState;
    class D3D10Buffer;
    class D3D10Texture1D;
    class D3D10Texture2D;
    class D3D10Texture3D;
    class D3D10ShaderResourceView;
    class D3D10RenderTargetView;
    class D3D10DepthStencilView;
    class D3D10InputLayout;
    class D3D10SamplerState;
    class D3D10Query;
    class D3D10Device;

    class D3D11DeviceChild : public IUnknown
    {
      public:
        virtual void STDMETHODCALLTYPE GetDevice(ID3D11Device * *ppDevice);
        virtual HRESULT STDMETHODCALLTYPE GetPrivateData(REFGUID guid, UINT * pDataSize, void *pData);
        virtual HRESULT STDMETHODCALLTYPE SetPrivateData(REFGUID guid, UINT DataSize, const void *pData);
        virtual HRESULT STDMETHODCALLTYPE SetPrivateDataInterface(REFGUID guid, const IUnknown *pData);
      private:
        ID3D11DeviceChild* m_Original;
    };

    class D3D11DepthStencilState : public ID3D11DepthStencilState
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_DEPTH_STENCIL_DESC * pDesc);

        virtual D3D10DepthStencilState* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11DepthStencilState* m_Original;
    };

    class D3D11BlendState : public ID3D11BlendState
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_BLEND_DESC * pDesc);

        virtual D3D10BlendState* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11BlendState* m_Original;
    };

    class D3D11RasterizerState : public ID3D11RasterizerState
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_RASTERIZER_DESC * pDesc);

        virtual D3D10RasterizerState* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11RasterizerState* m_Original;
    };

    class D3D11Resource : public ID3D11Resource
    {
      public:
        virtual void STDMETHODCALLTYPE GetType(D3D11_RESOURCE_DIMENSION * pResourceDimension);
        virtual void STDMETHODCALLTYPE SetEvictionPriority(UINT EvictionPriority);
        virtual UINT STDMETHODCALLTYPE GetEvictionPriority(void);
      private:
        ID3D11Resource* m_Original;
    };

    class D3D11Buffer : public ID3D11Buffer
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_BUFFER_DESC * pDesc);

        virtual D3D10Buffer* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11Buffer* m_Original;
    };

    class D3D11Texture1D : public ID3D11Texture1D
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_TEXTURE1D_DESC * pDesc);

        virtual D3D10Texture1D* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11Texture1D* m_Original;
    };

    class D3D11Texture2D : public ID3D11Texture2D
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_TEXTURE2D_DESC * pDesc);

        virtual D3D10Texture2D* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11Texture2D* m_Original;
    };

    class D3D11Texture3D : public ID3D11Texture3D
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_TEXTURE3D_DESC * pDesc);

        virtual D3D10Texture3D* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11Texture3D* m_Original;
    };

    class D3D11View : public ID3D11View
    {
      public:
        virtual void STDMETHODCALLTYPE GetResource(ID3D11Resource * *ppResource);
      private:
        ID3D11View* m_Original;
    };

    class D3D11ShaderResourceView : public ID3D11ShaderResourceView
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_SHADER_RESOURCE_VIEW_DESC * pDesc);

        virtual D3D10ShaderResourceView* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11ShaderResourceView* m_Original;
    };

    class D3D11RenderTargetView : public ID3D11RenderTargetView
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_RENDER_TARGET_VIEW_DESC * pDesc);

        virtual D3D10RenderTargetView* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11RenderTargetView* m_Original;
    };

    class D3D11DepthStencilView : public ID3D11DepthStencilView
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_DEPTH_STENCIL_VIEW_DESC * pDesc);

        virtual D3D10DepthStencilView* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11DepthStencilView* m_Original;
    };

    class D3D11UnorderedAccessView : public ID3D11UnorderedAccessView
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_UNORDERED_ACCESS_VIEW_DESC * pDesc);
      private:
        ID3D11UnorderedAccessView* m_Original;
    };

    template <typename D3D11Interface, typename D3D10Interface>
    class D3D11Shader : public D3D11Interface
    {
    public:
        using D3D10ShaderClass = D3D10Shader<D3D10Interface, D3D11Interface>;

        D3D11Shader(D3D11Device *device)
            : D3D11DeviceChild<D3D11Interface>(device),
              m_shader(shader), m_d3d10(this) {}

        ~D3D11Shader() {}

        HRESULT STDMETHODCALLTYPE QueryInterface(REFIID riid, void **ppvObject) final
        {
            *ppvObject = nullptr;

            if (riid == __uuidof(IUnknown) || riid == __uuidof(ID3D11DeviceChild) || riid == __uuidof(D3D11Interface))
            {
                *ppvObject = ref(this);
                return S_OK;
            }

            if (riid == __uuidof(IUnknown) || riid == __uuidof(ID3D10DeviceChild) || riid == __uuidof(D3D10Interface))
            {
                *ppvObject = ref(&m_d3d10);
                return S_OK;
            }

            Logger::warn("D3D11Shader::QueryInterface: Unknown interface query");
            return E_NOINTERFACE;
        }

        D3D10ShaderClass *GetD3D10Iface()
        {
            return &m_d3d10;
        }

    private:
        D3D10ShaderClass m_d3d10;
    };

    using D3D11VertexShader   = D3D11Shader<ID3D11VertexShader,   ID3D10VertexShader>;
    using D3D11HullShader     = D3D11Shader<ID3D11HullShader,     ID3D10DeviceChild>;
    using D3D11DomainShader   = D3D11Shader<ID3D11DomainShader,   ID3D10DeviceChild>;
    using D3D11GeometryShader = D3D11Shader<ID3D11GeometryShader, ID3D10GeometryShader>;
    using D3D11PixelShader    = D3D11Shader<ID3D11PixelShader,    ID3D10PixelShader>;
    using D3D11ComputeShader  = D3D11Shader<ID3D11ComputeShader,  ID3D10DeviceChild>;

    class D3D11InputLayout : public ID3D11InputLayout
    {
      public:
        virtual D3D10InputLayout* STDMETHODCALLTYPE GetD3D10Iface();
    };

    class D3D11SamplerState : public ID3D11SamplerState
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_SAMPLER_DESC * pDesc);

        virtual D3D10SamplerState* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11SamplerState* m_Original;
    };

    class D3D11Asynchronous : public ID3D11Asynchronous
    {
      public:
        virtual UINT STDMETHODCALLTYPE GetDataSize(void);
      private:
        ID3D11Asynchronous* m_Original;
    };

    class D3D11Query : public ID3D11Query
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_QUERY_DESC * pDesc);

        virtual D3D10Query* STDMETHODCALLTYPE GetD3D10Iface();

        static ID3D11Predicate* AsPredicate(ID3D11Query* pQuery) {
            // ID3D11Predicate and ID3D11Query have the same vtable. This
            // saves us some headache in all query-related functions.
            return static_cast<ID3D11Predicate*>(pQuery);
        }

        static D3D11Query* FromPredicate(ID3D11Predicate* pPredicate) {
            return static_cast<D3D11Query*>(static_cast<ID3D11Query*>(pPredicate));
        }
      private:
        ID3D11Query* m_Original;
    };

    class D3D11Predicate : public ID3D11Predicate{public : };

    class D3D11Counter : public ID3D11Counter
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_COUNTER_DESC * pDesc);
      private:
        ID3D11Counter* m_Original;
    };

    class D3D11ClassInstance : public ID3D11ClassInstance
    {
      public:
        virtual void STDMETHODCALLTYPE GetClassLinkage(ID3D11ClassLinkage * *ppLinkage);
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_CLASS_INSTANCE_DESC * pDesc);
        virtual void STDMETHODCALLTYPE GetInstanceName(LPSTR pInstanceName, SIZE_T * pBufferLength);
        virtual void STDMETHODCALLTYPE GetTypeName(LPSTR pTypeName, SIZE_T * pBufferLength);
      private:
        ID3D11ClassInstance* m_Original;
    };

    class D3D11ClassLinkage : public ID3D11ClassLinkage
    {
      public:
        virtual HRESULT STDMETHODCALLTYPE GetClassInstance(LPCSTR pClassInstanceName, UINT InstanceIndex, ID3D11ClassInstance * *ppInstance);
        virtual HRESULT STDMETHODCALLTYPE CreateClassInstance(LPCSTR pClassTypeName, UINT ConstantBufferOffset, UINT ConstantVectorOffset, UINT TextureOffset, UINT SamplerOffset, ID3D11ClassInstance * *ppInstance);
      private:
        ID3D11ClassLinkage* m_Original;
    };

    class D3D11CommandList : public ID3D11CommandList
    {
      public:
        virtual UINT STDMETHODCALLTYPE GetContextFlags(void);
      private:
        ID3D11CommandList* m_Original;
    };

    class D3D11DeviceContext : public ID3D11DeviceContext
    {
      public:
        virtual void STDMETHODCALLTYPE VSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE PSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE PSSetShader(ID3D11PixelShader * pPixelShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
        virtual void STDMETHODCALLTYPE PSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
        virtual void STDMETHODCALLTYPE VSSetShader(ID3D11VertexShader * pVertexShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
        virtual void STDMETHODCALLTYPE DrawIndexed(UINT IndexCount, UINT StartIndexLocation, INT BaseVertexLocation);
        virtual void STDMETHODCALLTYPE Draw(UINT VertexCount, UINT StartVertexLocation);
        virtual HRESULT STDMETHODCALLTYPE Map(ID3D11Resource * pResource, UINT Subresource, D3D11_MAP MapType, UINT MapFlags, D3D11_MAPPED_SUBRESOURCE * pMappedResource);
        virtual void STDMETHODCALLTYPE Unmap(ID3D11Resource * pResource, UINT Subresource);
        virtual void STDMETHODCALLTYPE PSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE IASetInputLayout(ID3D11InputLayout * pInputLayout);
        virtual void STDMETHODCALLTYPE IASetVertexBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppVertexBuffers, const UINT *pStrides, const UINT *pOffsets);
        virtual void STDMETHODCALLTYPE IASetIndexBuffer(ID3D11Buffer * pIndexBuffer, DXGI_FORMAT Format, UINT Offset);
        virtual void STDMETHODCALLTYPE DrawIndexedInstanced(UINT IndexCountPerInstance, UINT InstanceCount, UINT StartIndexLocation, INT BaseVertexLocation, UINT StartInstanceLocation);
        virtual void STDMETHODCALLTYPE DrawInstanced(UINT VertexCountPerInstance, UINT InstanceCount, UINT StartVertexLocation, UINT StartInstanceLocation);
        virtual void STDMETHODCALLTYPE GSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE GSSetShader(ID3D11GeometryShader * pShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
        virtual void STDMETHODCALLTYPE IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY Topology);
        virtual void STDMETHODCALLTYPE VSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE VSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
        virtual void STDMETHODCALLTYPE Begin(ID3D11Asynchronous * pAsync);
        virtual void STDMETHODCALLTYPE End(ID3D11Asynchronous * pAsync);
        virtual HRESULT STDMETHODCALLTYPE GetData(ID3D11Asynchronous * pAsync, void *pData, UINT DataSize, UINT GetDataFlags);
        virtual void STDMETHODCALLTYPE SetPredication(ID3D11Predicate * pPredicate, BOOL PredicateValue);
        virtual void STDMETHODCALLTYPE GSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE GSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
        virtual void STDMETHODCALLTYPE OMSetRenderTargets(UINT NumViews, ID3D11RenderTargetView *const *ppRenderTargetViews, ID3D11DepthStencilView *pDepthStencilView);
        virtual void STDMETHODCALLTYPE OMSetRenderTargetsAndUnorderedAccessViews(UINT NumRTVs, ID3D11RenderTargetView *const *ppRenderTargetViews, ID3D11DepthStencilView *pDepthStencilView, UINT UAVStartSlot, UINT NumUAVs, ID3D11UnorderedAccessView *const *ppUnorderedAccessViews, const UINT *pUAVInitialCounts);
        virtual void STDMETHODCALLTYPE OMSetBlendState(ID3D11BlendState * pBlendState, const FLOAT BlendFactor[4], UINT SampleMask);
        virtual void STDMETHODCALLTYPE OMSetDepthStencilState(ID3D11DepthStencilState * pDepthStencilState, UINT StencilRef);
        virtual void STDMETHODCALLTYPE SOSetTargets(UINT NumBuffers, ID3D11Buffer *const *ppSOTargets, const UINT *pOffsets);
        virtual void STDMETHODCALLTYPE DrawAuto(void);
        virtual void STDMETHODCALLTYPE DrawIndexedInstancedIndirect(ID3D11Buffer * pBufferForArgs, UINT AlignedByteOffsetForArgs);
        virtual void STDMETHODCALLTYPE DrawInstancedIndirect(ID3D11Buffer * pBufferForArgs, UINT AlignedByteOffsetForArgs);
        virtual void STDMETHODCALLTYPE Dispatch(UINT ThreadGroupCountX, UINT ThreadGroupCountY, UINT ThreadGroupCountZ);
        virtual void STDMETHODCALLTYPE DispatchIndirect(ID3D11Buffer * pBufferForArgs, UINT AlignedByteOffsetForArgs);
        virtual void STDMETHODCALLTYPE RSSetState(ID3D11RasterizerState * pRasterizerState);
        virtual void STDMETHODCALLTYPE RSSetViewports(UINT NumViewports, const D3D11_VIEWPORT *pViewports);
        virtual void STDMETHODCALLTYPE RSSetScissorRects(UINT NumRects, const D3D11_RECT *pRects);
        virtual void STDMETHODCALLTYPE CopySubresourceRegion(ID3D11Resource * pDstResource, UINT DstSubresource, UINT DstX, UINT DstY, UINT DstZ, ID3D11Resource * pSrcResource, UINT SrcSubresource, const D3D11_BOX *pSrcBox);
        virtual void STDMETHODCALLTYPE CopyResource(ID3D11Resource * pDstResource, ID3D11Resource * pSrcResource);
        virtual void STDMETHODCALLTYPE UpdateSubresource(ID3D11Resource * pDstResource, UINT DstSubresource, const D3D11_BOX *pDstBox, const void *pSrcData, UINT SrcRowPitch, UINT SrcDepthPitch);
        virtual void STDMETHODCALLTYPE CopyStructureCount(ID3D11Buffer * pDstBuffer, UINT DstAlignedByteOffset, ID3D11UnorderedAccessView * pSrcView);
        virtual void STDMETHODCALLTYPE ClearRenderTargetView(ID3D11RenderTargetView * pRenderTargetView, const FLOAT ColorRGBA[4]);
        virtual void STDMETHODCALLTYPE ClearUnorderedAccessViewUint(ID3D11UnorderedAccessView * pUnorderedAccessView, const UINT Values[4]);
        virtual void STDMETHODCALLTYPE ClearUnorderedAccessViewFloat(ID3D11UnorderedAccessView * pUnorderedAccessView, const FLOAT Values[4]);
        virtual void STDMETHODCALLTYPE ClearDepthStencilView(ID3D11DepthStencilView * pDepthStencilView, UINT ClearFlags, FLOAT Depth, UINT8 Stencil);
        virtual void STDMETHODCALLTYPE GenerateMips(ID3D11ShaderResourceView * pShaderResourceView);
        virtual void STDMETHODCALLTYPE SetResourceMinLOD(ID3D11Resource * pResource, FLOAT MinLOD);
        virtual FLOAT STDMETHODCALLTYPE GetResourceMinLOD(ID3D11Resource * pResource);
        virtual void STDMETHODCALLTYPE ResolveSubresource(ID3D11Resource * pDstResource, UINT DstSubresource, ID3D11Resource * pSrcResource, UINT SrcSubresource, DXGI_FORMAT Format);
        virtual void STDMETHODCALLTYPE ExecuteCommandList(ID3D11CommandList * pCommandList, BOOL RestoreContextState);
        virtual void STDMETHODCALLTYPE HSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE HSSetShader(ID3D11HullShader * pHullShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
        virtual void STDMETHODCALLTYPE HSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
        virtual void STDMETHODCALLTYPE HSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE DSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE DSSetShader(ID3D11DomainShader * pDomainShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
        virtual void STDMETHODCALLTYPE DSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
        virtual void STDMETHODCALLTYPE DSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE CSSetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView *const *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE CSSetUnorderedAccessViews(UINT StartSlot, UINT NumUAVs, ID3D11UnorderedAccessView *const *ppUnorderedAccessViews, const UINT *pUAVInitialCounts);
        virtual void STDMETHODCALLTYPE CSSetShader(ID3D11ComputeShader * pComputeShader, ID3D11ClassInstance *const *ppClassInstances, UINT NumClassInstances);
        virtual void STDMETHODCALLTYPE CSSetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState *const *ppSamplers);
        virtual void STDMETHODCALLTYPE CSSetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer *const *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE VSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE PSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE PSGetShader(ID3D11PixelShader * *ppPixelShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances);
        virtual void STDMETHODCALLTYPE PSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers);
        virtual void STDMETHODCALLTYPE VSGetShader(ID3D11VertexShader * *ppVertexShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances);
        virtual void STDMETHODCALLTYPE PSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE IAGetInputLayout(ID3D11InputLayout * *ppInputLayout);
        virtual void STDMETHODCALLTYPE IAGetVertexBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppVertexBuffers, UINT * pStrides, UINT * pOffsets);
        virtual void STDMETHODCALLTYPE IAGetIndexBuffer(ID3D11Buffer * *pIndexBuffer, DXGI_FORMAT * Format, UINT * Offset);
        virtual void STDMETHODCALLTYPE GSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE GSGetShader(ID3D11GeometryShader * *ppGeometryShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances);
        virtual void STDMETHODCALLTYPE IAGetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY * pTopology);
        virtual void STDMETHODCALLTYPE VSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE VSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers);
        virtual void STDMETHODCALLTYPE GetPredication(ID3D11Predicate * *ppPredicate, BOOL * pPredicateValue);
        virtual void STDMETHODCALLTYPE GSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE GSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers);
        virtual void STDMETHODCALLTYPE OMGetRenderTargets(UINT NumViews, ID3D11RenderTargetView * *ppRenderTargetViews, ID3D11DepthStencilView * *ppDepthStencilView);
        virtual void STDMETHODCALLTYPE OMGetRenderTargetsAndUnorderedAccessViews(UINT NumRTVs, ID3D11RenderTargetView * *ppRenderTargetViews, ID3D11DepthStencilView * *ppDepthStencilView, UINT UAVStartSlot, UINT NumUAVs, ID3D11UnorderedAccessView * *ppUnorderedAccessViews);
        virtual void STDMETHODCALLTYPE OMGetBlendState(ID3D11BlendState * *ppBlendState, FLOAT BlendFactor[4], UINT * pSampleMask);
        virtual void STDMETHODCALLTYPE OMGetDepthStencilState(ID3D11DepthStencilState * *ppDepthStencilState, UINT * pStencilRef);
        virtual void STDMETHODCALLTYPE SOGetTargets(UINT NumBuffers, ID3D11Buffer * *ppSOTargets);
        virtual void STDMETHODCALLTYPE SOGetTargetsWithOffsets(UINT NumBuffers, ID3D11Buffer** ppSOTargets, UINT* pOffsets);
        virtual void STDMETHODCALLTYPE RSGetState(ID3D11RasterizerState * *ppRasterizerState);
        virtual void STDMETHODCALLTYPE RSGetViewports(UINT * pNumViewports, D3D11_VIEWPORT * pViewports);
        virtual void STDMETHODCALLTYPE RSGetScissorRects(UINT * pNumRects, D3D11_RECT * pRects);
        virtual void STDMETHODCALLTYPE HSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE HSGetShader(ID3D11HullShader * *ppHullShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances);
        virtual void STDMETHODCALLTYPE HSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers);
        virtual void STDMETHODCALLTYPE HSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE DSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE DSGetShader(ID3D11DomainShader * *ppDomainShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances);
        virtual void STDMETHODCALLTYPE DSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers);
        virtual void STDMETHODCALLTYPE DSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE CSGetShaderResources(UINT StartSlot, UINT NumViews, ID3D11ShaderResourceView * *ppShaderResourceViews);
        virtual void STDMETHODCALLTYPE CSGetUnorderedAccessViews(UINT StartSlot, UINT NumUAVs, ID3D11UnorderedAccessView * *ppUnorderedAccessViews);
        virtual void STDMETHODCALLTYPE CSGetShader(ID3D11ComputeShader * *ppComputeShader, ID3D11ClassInstance * *ppClassInstances, UINT * pNumClassInstances);
        virtual void STDMETHODCALLTYPE CSGetSamplers(UINT StartSlot, UINT NumSamplers, ID3D11SamplerState * *ppSamplers);
        virtual void STDMETHODCALLTYPE CSGetConstantBuffers(UINT StartSlot, UINT NumBuffers, ID3D11Buffer * *ppConstantBuffers);
        virtual void STDMETHODCALLTYPE ClearState(void);
        virtual void STDMETHODCALLTYPE Flush(void);
        virtual D3D11_DEVICE_CONTEXT_TYPE STDMETHODCALLTYPE GetType(void);
        virtual UINT STDMETHODCALLTYPE GetContextFlags(void);
        virtual HRESULT STDMETHODCALLTYPE FinishCommandList(BOOL RestoreDeferredContextState, ID3D11CommandList * *ppCommandList);
      private:
        ID3D11DeviceContext* m_Original;
        std::vector<UINT> m_SOOffsets;
    };

    class D3D11VideoDecoder : public ID3D11VideoDecoder
    {
      public:
        virtual HRESULT STDMETHODCALLTYPE GetCreationParameters(D3D11_VIDEO_DECODER_DESC * pVideoDesc, D3D11_VIDEO_DECODER_CONFIG * pConfig);
        virtual HRESULT STDMETHODCALLTYPE GetDriverHandle(HANDLE * pDriverHandle);
      private:
        ID3D11VideoDecoder* m_Original;
    };

    class D3D11VideoProcessorEnumerator : public ID3D11VideoProcessorEnumerator
    {
      public:
        virtual HRESULT STDMETHODCALLTYPE GetVideoProcessorContentDesc(D3D11_VIDEO_PROCESSOR_CONTENT_DESC * pContentDesc);
        virtual HRESULT STDMETHODCALLTYPE CheckVideoProcessorFormat(DXGI_FORMAT Format, UINT * pFlags);
        virtual HRESULT STDMETHODCALLTYPE GetVideoProcessorCaps(D3D11_VIDEO_PROCESSOR_CAPS * pCaps);
        virtual HRESULT STDMETHODCALLTYPE GetVideoProcessorRateConversionCaps(UINT TypeIndex, D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS * pCaps);
        virtual HRESULT STDMETHODCALLTYPE GetVideoProcessorCustomRate(UINT TypeIndex, UINT CustomRateIndex, D3D11_VIDEO_PROCESSOR_CUSTOM_RATE * pRate);
        virtual HRESULT STDMETHODCALLTYPE GetVideoProcessorFilterRange(D3D11_VIDEO_PROCESSOR_FILTER Filter, D3D11_VIDEO_PROCESSOR_FILTER_RANGE * pRange);
      private:
        ID3D11VideoProcessorEnumerator* m_Original;
    };

    class D3D11VideoProcessor : public ID3D11VideoProcessor
    {
      public:
        virtual void STDMETHODCALLTYPE GetContentDesc(D3D11_VIDEO_PROCESSOR_CONTENT_DESC * pDesc);
        virtual void STDMETHODCALLTYPE GetRateConversionCaps(D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS * pCaps);
      private:
        ID3D11VideoProcessor* m_Original;
    };

    class D3D11AuthenticatedChannel : public ID3D11AuthenticatedChannel
    {
      public:
        virtual HRESULT STDMETHODCALLTYPE GetCertificateSize(UINT * pCertificateSize);
        virtual HRESULT STDMETHODCALLTYPE GetCertificate(UINT CertificateSize, BYTE * pCertificate);
        virtual void STDMETHODCALLTYPE GetChannelHandle(HANDLE * pChannelHandle);
      private:
        ID3D11AuthenticatedChannel* m_Original;
    };

    class D3D11CryptoSession : public ID3D11CryptoSession
    {
      public:
        virtual void STDMETHODCALLTYPE GetCryptoType(GUID * pCryptoType);
        virtual void STDMETHODCALLTYPE GetDecoderProfile(GUID * pDecoderProfile);
        virtual HRESULT STDMETHODCALLTYPE GetCertificateSize(UINT * pCertificateSize);
        virtual HRESULT STDMETHODCALLTYPE GetCertificate(UINT CertificateSize, BYTE * pCertificate);
        virtual void STDMETHODCALLTYPE GetCryptoSessionHandle(HANDLE * pCryptoSessionHandle);
      private:
        ID3D11CryptoSession* m_Original;
    };

    class D3D11VideoDecoderOutputView : public ID3D11VideoDecoderOutputView
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC * pDesc);
      private:
        ID3D11VideoDecoderOutputView* m_Original;
    };

    class D3D11VideoProcessorInputView : public ID3D11VideoProcessorInputView
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC * pDesc);
      private:
        ID3D11VideoProcessorInputView* m_Original;
    };

    class D3D11VideoProcessorOutputView : public ID3D11VideoProcessorOutputView
    {
      public:
        virtual void STDMETHODCALLTYPE GetDesc(D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC * pDesc);
      private:
        ID3D11VideoProcessorOutputView* m_Original;
    };

    class D3D11VideoContext : public ID3D11VideoContext
    {
      public:
        virtual HRESULT STDMETHODCALLTYPE GetDecoderBuffer(ID3D11VideoDecoder * pDecoder, D3D11_VIDEO_DECODER_BUFFER_TYPE Type, UINT * pBufferSize, void **ppBuffer);
        virtual HRESULT STDMETHODCALLTYPE ReleaseDecoderBuffer(ID3D11VideoDecoder * pDecoder, D3D11_VIDEO_DECODER_BUFFER_TYPE Type);
        virtual HRESULT STDMETHODCALLTYPE DecoderBeginFrame(ID3D11VideoDecoder * pDecoder, ID3D11VideoDecoderOutputView * pView, UINT ContentKeySize, const void *pContentKey);
        virtual HRESULT STDMETHODCALLTYPE DecoderEndFrame(ID3D11VideoDecoder * pDecoder);
        virtual HRESULT STDMETHODCALLTYPE SubmitDecoderBuffers(ID3D11VideoDecoder * pDecoder, UINT NumBuffers, const D3D11_VIDEO_DECODER_BUFFER_DESC *pBufferDesc);
        virtual APP_DEPRECATED_HRESULT STDMETHODCALLTYPE DecoderExtension(ID3D11VideoDecoder * pDecoder, const D3D11_VIDEO_DECODER_EXTENSION *pExtensionData);
        virtual void STDMETHODCALLTYPE VideoProcessorSetOutputTargetRect(ID3D11VideoProcessor * pVideoProcessor, BOOL Enable, const RECT *pRect);
        virtual void STDMETHODCALLTYPE VideoProcessorSetOutputBackgroundColor(ID3D11VideoProcessor * pVideoProcessor, BOOL YCbCr, const D3D11_VIDEO_COLOR *pColor);
        virtual void STDMETHODCALLTYPE VideoProcessorSetOutputColorSpace(ID3D11VideoProcessor * pVideoProcessor, const D3D11_VIDEO_PROCESSOR_COLOR_SPACE *pColorSpace);
        virtual void STDMETHODCALLTYPE VideoProcessorSetOutputAlphaFillMode(ID3D11VideoProcessor * pVideoProcessor, D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE AlphaFillMode, UINT StreamIndex);
        virtual void STDMETHODCALLTYPE VideoProcessorSetOutputConstriction(ID3D11VideoProcessor * pVideoProcessor, BOOL Enable, SIZE Size);
        virtual void STDMETHODCALLTYPE VideoProcessorSetOutputStereoMode(ID3D11VideoProcessor * pVideoProcessor, BOOL Enable);
        virtual APP_DEPRECATED_HRESULT STDMETHODCALLTYPE VideoProcessorSetOutputExtension(ID3D11VideoProcessor * pVideoProcessor, const GUID *pExtensionGuid, UINT DataSize, void *pData);
        virtual void STDMETHODCALLTYPE VideoProcessorGetOutputTargetRect(ID3D11VideoProcessor * pVideoProcessor, BOOL * Enabled, RECT * pRect);
        virtual void STDMETHODCALLTYPE VideoProcessorGetOutputBackgroundColor(ID3D11VideoProcessor * pVideoProcessor, BOOL * pYCbCr, D3D11_VIDEO_COLOR * pColor);
        virtual void STDMETHODCALLTYPE VideoProcessorGetOutputColorSpace(ID3D11VideoProcessor * pVideoProcessor, D3D11_VIDEO_PROCESSOR_COLOR_SPACE * pColorSpace);
        virtual void STDMETHODCALLTYPE VideoProcessorGetOutputAlphaFillMode(ID3D11VideoProcessor * pVideoProcessor, D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE * pAlphaFillMode, UINT * pStreamIndex);
        virtual void STDMETHODCALLTYPE VideoProcessorGetOutputConstriction(ID3D11VideoProcessor * pVideoProcessor, BOOL * pEnabled, SIZE * pSize);
        virtual void STDMETHODCALLTYPE VideoProcessorGetOutputStereoMode(ID3D11VideoProcessor * pVideoProcessor, BOOL * pEnabled);
        virtual APP_DEPRECATED_HRESULT STDMETHODCALLTYPE VideoProcessorGetOutputExtension(ID3D11VideoProcessor * pVideoProcessor, const GUID *pExtensionGuid, UINT DataSize, void *pData);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamFrameFormat(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_FRAME_FORMAT FrameFormat);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamColorSpace(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, const D3D11_VIDEO_PROCESSOR_COLOR_SPACE *pColorSpace);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamOutputRate(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_OUTPUT_RATE OutputRate, BOOL RepeatFrame, const DXGI_RATIONAL *pCustomRate);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamSourceRect(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, const RECT *pRect);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamDestRect(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, const RECT *pRect);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamAlpha(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, FLOAT Alpha);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamPalette(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, UINT Count, const UINT *pEntries);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamPixelAspectRatio(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, const DXGI_RATIONAL *pSourceAspectRatio, const DXGI_RATIONAL *pDestinationAspectRatio);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamLumaKey(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, FLOAT Lower, FLOAT Upper);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamStereoFormat(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, D3D11_VIDEO_PROCESSOR_STEREO_FORMAT Format, BOOL LeftViewFrame0, BOOL BaseViewFrame0, D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE FlipMode, int MonoOffset);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamAutoProcessingMode(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamFilter(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_FILTER Filter, BOOL Enable, int Level);
        virtual APP_DEPRECATED_HRESULT STDMETHODCALLTYPE VideoProcessorSetStreamExtension(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, const GUID *pExtensionGuid, UINT DataSize, void *pData);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamFrameFormat(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_FRAME_FORMAT * pFrameFormat);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamColorSpace(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_COLOR_SPACE * pColorSpace);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamOutputRate(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_OUTPUT_RATE * pOutputRate, BOOL * pRepeatFrame, DXGI_RATIONAL * pCustomRate);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamSourceRect(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, RECT * pRect);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamDestRect(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, RECT * pRect);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamAlpha(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, FLOAT * pAlpha);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamPalette(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, UINT Count, UINT * pEntries);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamPixelAspectRatio(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, DXGI_RATIONAL * pSourceAspectRatio, DXGI_RATIONAL * pDestinationAspectRatio);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamLumaKey(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled, FLOAT * pLower, FLOAT * pUpper);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamStereoFormat(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnable, D3D11_VIDEO_PROCESSOR_STEREO_FORMAT * pFormat, BOOL * pLeftViewFrame0, BOOL * pBaseViewFrame0, D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE * pFlipMode, int *MonoOffset);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamAutoProcessingMode(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnabled);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamFilter(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, D3D11_VIDEO_PROCESSOR_FILTER Filter, BOOL * pEnabled, int *pLevel);
        virtual APP_DEPRECATED_HRESULT STDMETHODCALLTYPE VideoProcessorGetStreamExtension(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, const GUID *pExtensionGuid, UINT DataSize, void *pData);
        virtual HRESULT STDMETHODCALLTYPE VideoProcessorBlt(ID3D11VideoProcessor * pVideoProcessor, ID3D11VideoProcessorOutputView * pView, UINT OutputFrame, UINT StreamCount, const D3D11_VIDEO_PROCESSOR_STREAM *pStreams);
        virtual HRESULT STDMETHODCALLTYPE NegotiateCryptoSessionKeyExchange(ID3D11CryptoSession * pCryptoSession, UINT DataSize, void *pData);
        virtual void STDMETHODCALLTYPE EncryptionBlt(ID3D11CryptoSession * pCryptoSession, ID3D11Texture2D * pSrcSurface, ID3D11Texture2D * pDstSurface, UINT IVSize, void *pIV);
        virtual void STDMETHODCALLTYPE DecryptionBlt(ID3D11CryptoSession * pCryptoSession, ID3D11Texture2D * pSrcSurface, ID3D11Texture2D * pDstSurface, D3D11_ENCRYPTED_BLOCK_INFO * pEncryptedBlockInfo, UINT ContentKeySize, const void *pContentKey, UINT IVSize, void *pIV);
        virtual void STDMETHODCALLTYPE StartSessionKeyRefresh(ID3D11CryptoSession * pCryptoSession, UINT RandomNumberSize, void *pRandomNumber);
        virtual void STDMETHODCALLTYPE FinishSessionKeyRefresh(ID3D11CryptoSession * pCryptoSession);
        virtual HRESULT STDMETHODCALLTYPE GetEncryptionBltKey(ID3D11CryptoSession * pCryptoSession, UINT KeySize, void *pReadbackKey);
        virtual HRESULT STDMETHODCALLTYPE NegotiateAuthenticatedChannelKeyExchange(ID3D11AuthenticatedChannel * pChannel, UINT DataSize, void *pData);
        virtual HRESULT STDMETHODCALLTYPE QueryAuthenticatedChannel(ID3D11AuthenticatedChannel * pChannel, UINT InputSize, const void *pInput, UINT OutputSize, void *pOutput);
        virtual HRESULT STDMETHODCALLTYPE ConfigureAuthenticatedChannel(ID3D11AuthenticatedChannel * pChannel, UINT InputSize, const void *pInput, D3D11_AUTHENTICATED_CONFIGURE_OUTPUT *pOutput);
        virtual void STDMETHODCALLTYPE VideoProcessorSetStreamRotation(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL Enable, D3D11_VIDEO_PROCESSOR_ROTATION Rotation);
        virtual void STDMETHODCALLTYPE VideoProcessorGetStreamRotation(ID3D11VideoProcessor * pVideoProcessor, UINT StreamIndex, BOOL * pEnable, D3D11_VIDEO_PROCESSOR_ROTATION * pRotation);
      private:
        ID3D11VideoContext* m_Original;
    };

    class D3D11VideoDevice : public ID3D11VideoDevice
    {
      public:
        virtual HRESULT STDMETHODCALLTYPE CreateVideoDecoder(const D3D11_VIDEO_DECODER_DESC *pVideoDesc, const D3D11_VIDEO_DECODER_CONFIG *pConfig, ID3D11VideoDecoder **ppDecoder);
        virtual HRESULT STDMETHODCALLTYPE CreateVideoProcessor(ID3D11VideoProcessorEnumerator * pEnum, UINT RateConversionIndex, ID3D11VideoProcessor * *ppVideoProcessor);
        virtual HRESULT STDMETHODCALLTYPE CreateAuthenticatedChannel(D3D11_AUTHENTICATED_CHANNEL_TYPE ChannelType, ID3D11AuthenticatedChannel * *ppAuthenticatedChannel);
        virtual HRESULT STDMETHODCALLTYPE CreateCryptoSession(const GUID *pCryptoType, const GUID *pDecoderProfile, const GUID *pKeyExchangeType, ID3D11CryptoSession **ppCryptoSession);
        virtual HRESULT STDMETHODCALLTYPE CreateVideoDecoderOutputView(ID3D11Resource * pResource, const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC *pDesc, ID3D11VideoDecoderOutputView **ppVDOVView);
        virtual HRESULT STDMETHODCALLTYPE CreateVideoProcessorInputView(ID3D11Resource * pResource, ID3D11VideoProcessorEnumerator * pEnum, const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC *pDesc, ID3D11VideoProcessorInputView **ppVPIView);
        virtual HRESULT STDMETHODCALLTYPE CreateVideoProcessorOutputView(ID3D11Resource * pResource, ID3D11VideoProcessorEnumerator * pEnum, const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC *pDesc, ID3D11VideoProcessorOutputView **ppVPOView);
        virtual HRESULT STDMETHODCALLTYPE CreateVideoProcessorEnumerator(const D3D11_VIDEO_PROCESSOR_CONTENT_DESC *pDesc, ID3D11VideoProcessorEnumerator **ppEnum);
        virtual UINT STDMETHODCALLTYPE GetVideoDecoderProfileCount(void);
        virtual HRESULT STDMETHODCALLTYPE GetVideoDecoderProfile(UINT Index, GUID * pDecoderProfile);
        virtual HRESULT STDMETHODCALLTYPE CheckVideoDecoderFormat(const GUID *pDecoderProfile, DXGI_FORMAT Format, BOOL *pSupported);
        virtual HRESULT STDMETHODCALLTYPE GetVideoDecoderConfigCount(const D3D11_VIDEO_DECODER_DESC *pDesc, UINT *pCount);
        virtual HRESULT STDMETHODCALLTYPE GetVideoDecoderConfig(const D3D11_VIDEO_DECODER_DESC *pDesc, UINT Index, D3D11_VIDEO_DECODER_CONFIG *pConfig);
        virtual HRESULT STDMETHODCALLTYPE GetContentProtectionCaps(const GUID *pCryptoType, const GUID *pDecoderProfile, D3D11_VIDEO_CONTENT_PROTECTION_CAPS *pCaps);
        virtual HRESULT STDMETHODCALLTYPE CheckCryptoKeyExchange(const GUID *pCryptoType, const GUID *pDecoderProfile, UINT Index, GUID *pKeyExchangeType);
        virtual HRESULT STDMETHODCALLTYPE SetPrivateData(REFGUID guid, UINT DataSize, const void *pData);
        virtual HRESULT STDMETHODCALLTYPE SetPrivateDataInterface(REFGUID guid, const IUnknown *pData);
      private:
        ID3D11VideoDevice* m_Original;
    };

    class D3D11Device : public ID3D11Device
    {
      public:
        virtual HRESULT STDMETHODCALLTYPE CreateBuffer(const D3D11_BUFFER_DESC *pDesc, const D3D11_SUBRESOURCE_DATA *pInitialData, ID3D11Buffer **ppBuffer);
        virtual HRESULT STDMETHODCALLTYPE CreateTexture1D(const D3D11_TEXTURE1D_DESC *pDesc, const D3D11_SUBRESOURCE_DATA *pInitialData, ID3D11Texture1D **ppTexture1D);
        virtual HRESULT STDMETHODCALLTYPE CreateTexture2D(const D3D11_TEXTURE2D_DESC *pDesc, const D3D11_SUBRESOURCE_DATA *pInitialData, ID3D11Texture2D **ppTexture2D);
        virtual HRESULT STDMETHODCALLTYPE CreateTexture3D(const D3D11_TEXTURE3D_DESC *pDesc, const D3D11_SUBRESOURCE_DATA *pInitialData, ID3D11Texture3D **ppTexture3D);
        virtual HRESULT STDMETHODCALLTYPE CreateShaderResourceView(ID3D11Resource * pResource, const D3D11_SHADER_RESOURCE_VIEW_DESC *pDesc, ID3D11ShaderResourceView **ppSRView);
        virtual HRESULT STDMETHODCALLTYPE CreateUnorderedAccessView(ID3D11Resource * pResource, const D3D11_UNORDERED_ACCESS_VIEW_DESC *pDesc, ID3D11UnorderedAccessView **ppUAView);
        virtual HRESULT STDMETHODCALLTYPE CreateRenderTargetView(ID3D11Resource * pResource, const D3D11_RENDER_TARGET_VIEW_DESC *pDesc, ID3D11RenderTargetView **ppRTView);
        virtual HRESULT STDMETHODCALLTYPE CreateDepthStencilView(ID3D11Resource * pResource, const D3D11_DEPTH_STENCIL_VIEW_DESC *pDesc, ID3D11DepthStencilView **ppDepthStencilView);
        virtual HRESULT STDMETHODCALLTYPE CreateInputLayout(const D3D11_INPUT_ELEMENT_DESC *pInputElementDescs, UINT NumElements, const void *pShaderBytecodeWithInputSignature, SIZE_T BytecodeLength, ID3D11InputLayout **ppInputLayout);
        virtual HRESULT STDMETHODCALLTYPE CreateVertexShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11VertexShader **ppVertexShader);
        virtual HRESULT STDMETHODCALLTYPE CreateGeometryShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11GeometryShader **ppGeometryShader);
        virtual HRESULT STDMETHODCALLTYPE CreateGeometryShaderWithStreamOutput(const void *pShaderBytecode, SIZE_T BytecodeLength, const D3D11_SO_DECLARATION_ENTRY *pSODeclaration, UINT NumEntries, const UINT *pBufferStrides, UINT NumStrides, UINT RasterizedStream, ID3D11ClassLinkage *pClassLinkage, ID3D11GeometryShader **ppGeometryShader);
        virtual HRESULT STDMETHODCALLTYPE CreatePixelShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11PixelShader **ppPixelShader);
        virtual HRESULT STDMETHODCALLTYPE CreateHullShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11HullShader **ppHullShader);
        virtual HRESULT STDMETHODCALLTYPE CreateDomainShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11DomainShader **ppDomainShader);
        virtual HRESULT STDMETHODCALLTYPE CreateComputeShader(const void *pShaderBytecode, SIZE_T BytecodeLength, ID3D11ClassLinkage *pClassLinkage, ID3D11ComputeShader **ppComputeShader);
        virtual HRESULT STDMETHODCALLTYPE CreateClassLinkage(ID3D11ClassLinkage * *ppLinkage);
        virtual HRESULT STDMETHODCALLTYPE CreateBlendState(const D3D11_BLEND_DESC *pBlendStateDesc, ID3D11BlendState **ppBlendState);
        virtual HRESULT STDMETHODCALLTYPE CreateDepthStencilState(const D3D11_DEPTH_STENCIL_DESC *pDepthStencilDesc, ID3D11DepthStencilState **ppDepthStencilState);
        virtual HRESULT STDMETHODCALLTYPE CreateRasterizerState(const D3D11_RASTERIZER_DESC *pRasterizerDesc, ID3D11RasterizerState **ppRasterizerState);
        virtual HRESULT STDMETHODCALLTYPE CreateSamplerState(const D3D11_SAMPLER_DESC *pSamplerDesc, ID3D11SamplerState **ppSamplerState);
        virtual HRESULT STDMETHODCALLTYPE CreateQuery(const D3D11_QUERY_DESC *pQueryDesc, ID3D11Query **ppQuery);
        virtual HRESULT STDMETHODCALLTYPE CreatePredicate(const D3D11_QUERY_DESC *pPredicateDesc, ID3D11Predicate **ppPredicate);
        virtual HRESULT STDMETHODCALLTYPE CreateCounter(const D3D11_COUNTER_DESC *pCounterDesc, ID3D11Counter **ppCounter);
        virtual HRESULT STDMETHODCALLTYPE CreateDeferredContext(UINT ContextFlags, ID3D11DeviceContext * *ppDeferredContext);
        virtual HRESULT STDMETHODCALLTYPE OpenSharedResource(HANDLE hResource, REFIID ReturnedInterface, void **ppResource);
        virtual HRESULT STDMETHODCALLTYPE CheckFormatSupport(DXGI_FORMAT Format, UINT * pFormatSupport);
        virtual HRESULT STDMETHODCALLTYPE CheckMultisampleQualityLevels(DXGI_FORMAT Format, UINT SampleCount, UINT * pNumQualityLevels);
        virtual void STDMETHODCALLTYPE CheckCounterInfo(D3D11_COUNTER_INFO * pCounterInfo);
        virtual HRESULT STDMETHODCALLTYPE CheckCounter(const D3D11_COUNTER_DESC *pDesc, D3D11_COUNTER_TYPE *pType, UINT *pActiveCounters, LPSTR szName, UINT *pNameLength, LPSTR szUnits, UINT *pUnitsLength, LPSTR szDescription, UINT *pDescriptionLength);
        virtual HRESULT STDMETHODCALLTYPE CheckFeatureSupport(D3D11_FEATURE Feature, void *pFeatureSupportData, UINT FeatureSupportDataSize);
        virtual HRESULT STDMETHODCALLTYPE GetPrivateData(REFGUID guid, UINT * pDataSize, void *pData);
        virtual HRESULT STDMETHODCALLTYPE SetPrivateData(REFGUID guid, UINT DataSize, const void *pData);
        virtual HRESULT STDMETHODCALLTYPE SetPrivateDataInterface(REFGUID guid, const IUnknown *pData);
        virtual D3D_FEATURE_LEVEL STDMETHODCALLTYPE GetFeatureLevel(void);
        virtual UINT STDMETHODCALLTYPE GetCreationFlags(void);
        virtual HRESULT STDMETHODCALLTYPE GetDeviceRemovedReason(void);
        virtual void STDMETHODCALLTYPE GetImmediateContext(ID3D11DeviceContext * *ppImmediateContext);
        virtual HRESULT STDMETHODCALLTYPE SetExceptionMode(UINT RaiseFlags);
        virtual UINT STDMETHODCALLTYPE GetExceptionMode(void);

        virtual D3D10Device* STDMETHODCALLTYPE GetD3D10Iface();
      private:
        ID3D11Device* m_Original;
    };

    HRESULT WINAPI D3D11CreateDevice(IDXGIAdapter *pAdapter, D3D_DRIVER_TYPE DriverType, HMODULE Software, UINT Flags, CONST D3D_FEATURE_LEVEL *pFeatureLevels, UINT FeatureLevels, UINT SDKVersion, ID3D11Device **ppDevice, D3D_FEATURE_LEVEL *pFeatureLevel, ID3D11DeviceContext **ppImmediateContext);
    HRESULT WINAPI D3D11CreateDeviceAndSwapChain(IDXGIAdapter *pAdapter, D3D_DRIVER_TYPE DriverType, HMODULE Software, UINT Flags, CONST D3D_FEATURE_LEVEL *pFeatureLevels, UINT FeatureLevels, UINT SDKVersion, CONST DXGI_SWAP_CHAIN_DESC *pSwapChainDesc, IDXGISwapChain **ppSwapChain, ID3D11Device **ppDevice, D3D_FEATURE_LEVEL *pFeatureLevel, ID3D11DeviceContext **ppImmediateContext);

} // namespace dxvk