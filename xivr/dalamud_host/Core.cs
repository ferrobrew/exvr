using Dalamud.Plugin;
using Dalamud.Logging;
using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using ImGuiNET;

namespace XIVR
{
    [UnmanagedFunctionPointer(CallingConvention.Winapi)]
    public delegate void LogDelegate(string s);

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public unsafe struct LoadParameters
    {
        public LogDelegate logger;
        public IntPtr imguiContext;
        public IntPtr imguiAllocatorAlloc;
        public IntPtr imguiAllocatorFree;
        public void* imguiAllocatorUserData;
    }

    public class Core : IDalamudPlugin
    {
        public string Name => "XIVR Core";

        private DalamudPluginInterface pi;

        // When loaded by LivePluginLoader, the executing assembly will be wrong.
        // Supplying this property allows LivePluginLoader to supply the correct location, so that
        // you have full compatibility when loaded normally and through LPL.
        public string AssemblyLocation { get => assemblyLocation; set => assemblyLocation = value; }
        private string assemblyLocation = System.Reflection.Assembly.GetExecutingAssembly().Location;

        private string DirPath { get => Path.GetFullPath(Path.GetDirectoryName(assemblyLocation)!); }
        private string ModuleName(string ext) => "xivr_native" + "." + ext;
        private string ModulePath(string ext) => Path.Combine(DirPath, ModuleName(ext));
        private string ModuleLoadedName(string ext) => "xivr_native_loaded" + "." + ext;
        private string ModuleLoadedPath(string ext) => Path.Combine(DirPath, ModuleLoadedName(ext));
        private IntPtr module = IntPtr.Zero;
        private bool visible = true;

        private LogDelegate logDelegate = (s) => PluginLog.Information("native: {0:l}", s);

        public Core(DalamudPluginInterface pluginInterface)
        {
            this.pi = pluginInterface;

            this.pi.UiBuilder.Draw += this.OnDraw;
        }

        public void Dispose()
        {
            Unload(() => { });

            this.pi.Dispose();
        }

        private unsafe IntPtr ModuleFunction(string name)
        {
            return NativeLibrary.GetExport(this.module, name);
        }

        private void Reload()
        {
            PluginLog.Information("Reloading...");

            if (this.module != IntPtr.Zero)
            {
                // On unload, we resize the window. This causes the D3D device to be invalidated,
                // and we don't want to start up OpenXR with an invalid device.
                // Instead, let's use more jank to delay the startup until we can be sure we're good to go.
                this.Unload(() => Task.Delay(2500).ContinueWith(_ => this.Load()));
            }
            else
            {
                this.Load();
            }
        }

        private void Unload(Action onUnload)
        {
            if (this.module == IntPtr.Zero) return;

            unsafe
            {
                ((delegate* unmanaged<void>)ModuleFunction("xivr_unload"))();
            }
            NativeLibrary.Free(this.module);
            this.module = IntPtr.Zero;

            PluginLog.Information("Destroyed module");
            onUnload();
        }

        private void Load()
        {
            if (this.module != IntPtr.Zero) return;

            File.Copy(ModulePath("dll"), ModuleLoadedPath("dll"), true);
            File.Copy(ModulePath("pdb"), ModuleLoadedPath("pdb"), true);

            this.module = NativeLibrary.Load(ModuleLoadedPath("dll"));
            if (this.module == IntPtr.Zero)
            {
                throw new Exception(string.Format("Failed to load native module: {0}", Marshal.GetLastWin32Error()));
            }

            unsafe
            {
                LoadParameters parameters = default;
                parameters.logger = this.logDelegate;
                parameters.imguiContext = ImGui.GetCurrentContext();
                ImGui.GetAllocatorFunctions(
                    ref parameters.imguiAllocatorAlloc,
                    ref parameters.imguiAllocatorFree,
                    ref parameters.imguiAllocatorUserData
                );

                IntPtr ptr = Marshal.AllocHGlobal(Marshal.SizeOf(parameters));
                Marshal.StructureToPtr(parameters, ptr, false);

                try
                {
                    ((delegate* unmanaged<IntPtr, bool>)ModuleFunction("xivr_load"))(ptr);
                }
                finally
                {
                    Marshal.FreeHGlobal(ptr);
                }
            }
        }

        private void OnDraw()
        {
            if (ImGui.Begin("XIVR Loader", ref this.visible, ImGuiWindowFlags.NoScrollbar | ImGuiWindowFlags.NoScrollWithMouse))
            {
                if (ImGui.Button("Reload"))
                {
                    Reload();
                }
                if (this.module == IntPtr.Zero)
                {
                    if (ImGui.Button("Load"))
                    {
                        Load();
                    }
                }
                else
                {
                    if (ImGui.Button("Unload"))
                    {
                        Unload(() => { });
                    }
                }
            }

            if (this.module == IntPtr.Zero) return;
            unsafe
            {
                ((delegate* unmanaged<void>)ModuleFunction("xivr_draw_ui"))();
            }
        }
    }
}
