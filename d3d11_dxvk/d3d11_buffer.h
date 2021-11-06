#pragma once


#include "../d3d10/d3d10_buffer.h"

#include "d3d11_device_child.h"
#include "d3d11_resource.h"

namespace dxvk {

  class D3D11Device;
  class D3D11DeviceContext;


  class D3D11Buffer : public D3D11DeviceChild<ID3D11Buffer> {
  public:

    D3D11Buffer(
            D3D11Device*                pDevice,
      const D3D11_BUFFER_DESC*          pDesc);
    ~D3D11Buffer();

    HRESULT STDMETHODCALLTYPE QueryInterface(
            REFIID  riid,
            void**  ppvObject) final;

    void STDMETHODCALLTYPE GetDesc(
            D3D11_BUFFER_DESC *pDesc) final;

    const D3D11_BUFFER_DESC* Desc() const {
      return &m_desc;
    }

    D3D10Buffer* GetD3D10Iface() {
      return &m_d3d10;
    }

    /**
     * \brief Normalizes buffer description
     *
     * \param [in] pDesc Buffer description
     * \returns \c S_OK if the parameters are valid
     */
    static HRESULT NormalizeBufferProperties(
            D3D11_BUFFER_DESC*      pDesc);

  private:

    const D3D11_BUFFER_DESC     m_desc;

    D3D11DXGIResource           m_resource;
    D3D10Buffer                 m_d3d10;

  };


  /**
   * \brief Retrieves buffer from resource pointer
   *
   * \param [in] pResource The resource to query
   * \returns Pointer to buffer, or \c nullptr
   */
  D3D11Buffer* GetCommonBuffer(
          ID3D11Resource*       pResource);

}
