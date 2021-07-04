fn main() {
    windows::build! {
        Windows::Win32::Foundation::*,
        Windows::Win32::System::Threading::*,
        Windows::Win32::System::ProcessStatus::*,
        Windows::Win32::System::LibraryLoader::*,
        Windows::Win32::System::Memory::*,
        Windows::Win32::Graphics::Direct3D11::*,
        Windows::Win32::Graphics::Dxgi::*,
        Windows::Win32::UI::WindowsAndMessaging::*,
    };
}