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

        private void Unload(Action onUnload)
        {
            if (this.module == IntPtr.Zero) return;

            unsafe
            {
                ((delegate* unmanaged<void>)ModuleFunction("xivr_unload"))();
            }
            this.module = IntPtr.Zero;

            onUnload();
        }

        private void Load()
        {
            if (this.module != IntPtr.Zero) return;

            this.module = NativeLibrary.Load(ModulePath("dll"));
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
