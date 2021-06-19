#pragma once

#include <functional>

namespace xr {
struct Eye
{
    int m_Index{0};

    void Init(int index);
    void Destroy();
    void BeginUpdate();
    void ApplyMatrix();
    void EndUpdate();
    void Render(float x, float y, float w, float h);
    uint32_t GetImage() const;
};

class StereoRender
{
  public:
    void Init();
    void Destroy();
    void Render(const std::function<void()>& fn);
    void RenderEyes();
    uint32_t GetImage(size_t eye) const;

  private:
    size_t m_EyeIndex{0};
    Eye m_Eyes[2];
};
};
