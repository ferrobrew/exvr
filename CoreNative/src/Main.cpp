#include <Windows.h>

#include "Log.h"
#include "game/graphics/kernel/Device.h"

#include "xr/XR.h"

#include <MinHook.h>

LogType g_Log = nullptr;
void* g_BaseAddress = nullptr;
HMODULE g_Module;

struct LoadParameters
{
	LogType logger;
	void* baseAddress;
};

void* (*Orig_GameMain_Update)(void* self);
void* Hook_GameMain_Update(void* self)
{
	// todo: we should probably do something with these
	bool exit, restart;
	xr::PollEvents(exit, restart);
	xr::PreTick();
	auto ret = Orig_GameMain_Update(self);
	xr::PostTick();

	return ret;
}

extern "C" __declspec(dllexport) void XIVR_Load(LoadParameters loadParameters)
{
	g_Log = loadParameters.logger;
	g_BaseAddress = loadParameters.baseAddress;

	auto device = game::graphics::kernel::Device::Get();
	auto featureLevel = device->FeatureLevel();

	Logf("Loaded, base address 0x%X, feature level 0x%X", g_BaseAddress, featureLevel);

	xr::Initialize(true);

	MH_Initialize();

	auto updatePtr = game::MakeBaseRelativePtr<void>(game::offsets::functions::Game_GameMain_Update);
	MH_CreateHook(updatePtr, &Hook_GameMain_Update, (void**)&Orig_GameMain_Update);
	MH_EnableHook(updatePtr);
}

extern "C" __declspec(dllexport) void XIVR_Unload()
{
	MH_DisableHook(MH_ALL_HOOKS);
	MH_Uninitialize();

	xr::Shutdown();

	g_Log("Unloaded");
}

BOOL APIENTRY DllMain(HMODULE hModule,
					  DWORD ul_reason_for_call,
					  LPVOID lpReserved)
{
	g_Module = hModule;

	switch (ul_reason_for_call)
	{
	case DLL_PROCESS_ATTACH:
		break;
	case DLL_THREAD_ATTACH:
		break;
	case DLL_THREAD_DETACH:
		break;
	case DLL_PROCESS_DETACH:
		break;
	}
	return TRUE;
}