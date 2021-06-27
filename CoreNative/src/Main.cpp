#include <Windows.h>
#include <winnt.h>

#include "Log.h"
#include "game/graphics/kernel/Device.h"
#include "game/graphics/kernel/Texture.h"

#include "debug/CaptureCallstack.h"

#include "xr/XR.h"
#include "xr/Session.h"

#include <MinHook.h>
#include <chrono>
#include <nlohmann/json.hpp>
#include <fstream>
#include <filesystem>
#include <fmt/format.h>
#include <mutex>

LogType g_Log = nullptr;
extern "C" void* g_BaseAddress = nullptr;
void* g_ContextPushBackEventPtr = nullptr;
HMODULE g_Module;
std::filesystem::path g_DirPath;

struct LoadParameters
{
	LogType logger;
	void* baseAddress;
	void* contextPushBackEventPtr;
	char dirPath[260];
};

class Patcher
{
public:
	~Patcher()
	{
		std::lock_guard<std::mutex> lock(this->mutex);

		for (auto it = this->patches.rbegin(); it != this->patches.rend(); ++it)
		{
			auto const& patch = *it;

			memcpy(patch.address, patch.originalBytes.data(), patch.originalBytes.size());

			DWORD discard;
			VirtualProtect(patch.address, patch.originalBytes.size(), patch.originalProtect, &discard);
		}
	}

	void Patch(uint8_t* address, uint8_t const* data, size_t byteCount)
	{
		std::lock_guard<std::mutex> lock(this->mutex);

		DWORD originalProtect;
		VirtualProtect(address, byteCount, PAGE_EXECUTE_READWRITE, &originalProtect);

		std::vector<uint8_t> originalBytes(byteCount);
		memcpy(originalBytes.data(), address, byteCount);
		memcpy(address, data, byteCount);

		this->patches.push_back({ address, originalBytes, originalProtect });
	}

	template<typename Ty>
	void Patch(uint8_t* address, Ty const& value)
	{
		this->Patch(address, reinterpret_cast<uint8_t const*>(&value), sizeof(Ty));
	}

private:
	struct PatchData
	{
		uint8_t* address;
		std::vector<uint8_t> originalBytes;
		DWORD originalProtect;
	};
	std::vector<PatchData> patches;

	std::mutex mutex;
};

enum class ShaderCommandType : uint32_t
{
	SetRenderTargets = 0,
	SetViewport = 1,
	SetScissorRect = 3,
	Clear = 4,
	Draw = 5,
	DrawIndexed = 6,
	DrawIndexedInstanced = 7,
	DispatchComputeShader = 8,
	CopyTexture = 0x0A,
	UnknownDraw = 0x0B,
	CopyResource = 0x0C,
	ResetRendererMaybe = 0x0D,
	CopySubresourceRegion = 0x10,
};

#pragma pack(push, 1)
struct ShaderCommand
{
	ShaderCommandType type;
	union
	{
		struct
		{
			uint32_t renderTargetCount;
			uint64_t renderTargets[4];
			uint64_t field_28;
			int field_30;
			int field_34;
		} SetRenderTargets;
		static_assert(sizeof(SetRenderTargets) + sizeof(ShaderCommandType) == 0x38);

		struct
		{
			int bounds[4];
			float minDepth;
			float maxDepth;
		} SetViewport;
		static_assert(sizeof(SetViewport) + sizeof(ShaderCommandType) == 0x1C);

		struct
		{
			D3D11_RECT rect;
		} SetScissorRect;
		static_assert(sizeof(SetScissorRect) + sizeof(ShaderCommandType) == 0x14);

		struct
		{
			int clearFlags;
			float colour[4];
			double field_18;
			uint64_t field_20;
			int field_28;
			int field_2C;
			int field_30;
			int field_34;
		} Clear;
		static_assert(sizeof(Clear) + sizeof(ShaderCommandType) == 0x38);

		struct
		{
			int field_4;
			int startVertexLocation;
			uint32_t vertexCount;
			uint8_t probablyModel[0x70];
		} Draw;
		static_assert(sizeof(Draw) + sizeof(ShaderCommandType) == 0x80);

		struct
		{
			int field_4;
			uint32_t baseVertexLocation;
			int field_C;
			int field_10;
			uint32_t startIndexLocation;
			uint32_t indexCount;
			int field_1C;
			int field_20;
			int field_24;
			int field_28;
			int field_2C;
			int field_30;
			int field_34;
			int field_38;
			int field_3C;
			int field_40;
			int field_44;
			int field_48;
			int field_4C;
			int field_50;
			int field_54;
			int field_58;
			int field_5C;
			int field_60;
			int field_64;
			int field_68;
			int field_6C;
			int field_70;
			int field_74;
			int field_78;
			uint64_t field_7C;
			uint64_t field_84;
		} DrawIndexed;
		static_assert(sizeof(DrawIndexed) + sizeof(ShaderCommandType) == 0x8C);

		struct
		{
			int field_4;
			int baseVertexLocation;
			int field_C;
			uint32_t field_10;
			int startIndexLocation;
			int indexCountPerInstance;
			int instanceCount;
			uint8_t probablyModel[0x70];
		} DrawIndexedInstanced;
		static_assert(sizeof(DrawIndexedInstanced) + sizeof(ShaderCommandType) == 0x90);

		struct
		{
			int field_4;
			uint64_t field_8;
			uint64_t field_10;
			uint64_t field_18;
			uint64_t field_20;
			uint32_t threadGroupCountX;
			uint32_t threadGroupCountY;
			uint32_t threadGroupCountZ;
			int field_34;
		} DispatchComputeShader;
		static_assert(sizeof(DispatchComputeShader) + sizeof(ShaderCommandType) == 0x38);

		struct
		{
			int field_4;
			game::graphics::kernel::Texture* dstResource;
			uint32_t dstSubresource;
			uint32_t dstXY[2];
			int field_1C;
			game::graphics::kernel::Texture* srcResource;
			int srcSubresource;
			int field_2C;
			uint64_t useSuppliedRect;
			uint32_t rect[4];
		} CopyTexture;
		static_assert(sizeof(CopyTexture) + sizeof(ShaderCommandType) == 0x48);

		struct
		{
			int field_4;
			game::graphics::kernel::Texture* texture;
			int field_10;
			int field_14;
			uint64_t field_18;
			uint64_t field_20;
			int field_28;
			int field_2C;
			uint64_t field_30;
			int field_38;
			int field_3C;
			int field_40;
			int field_44;
			int field_48;
			int field_4C;
			int field_50;
			int field_54;
		} UnknownDraw;
		static_assert(sizeof(UnknownDraw) + sizeof(ShaderCommandType) == 0x58);

		struct
		{
			int field_4;
			uint64_t callback;
			uint64_t field_10;
			int field_18;
			int field_1C;
		} ResetRendererMaybe;
		static_assert(sizeof(ResetRendererMaybe) + sizeof(ShaderCommandType) == 0x20);
	};
};
#pragma pack(pop)

struct CapturedCommand
{
	ShaderCommand command;
	uint32_t threadId;
	std::vector<debug::CallstackEntry> callstack;

	ShaderCommand* operator->() { return &command; }
	ShaderCommand const* operator->() const { return &command; }
};

bool g_IsCapturing = false;
namespace ch = std::chrono;
ch::high_resolution_clock::time_point g_LastCapture;
std::vector<CapturedCommand> g_CapturedCommands;
std::mutex g_CaptureMutex;

template<typename Ty>
nlohmann::json ConvertCArray(Ty* arr, int N)
{
	auto j = nlohmann::json::array();
	for (size_t i = 0; i < N; ++i)
		j.push_back(arr[i]);
	return j;
}

template<typename Ty, int N>
nlohmann::json ConvertCArray(Ty (&arr)[N])
{
	return ConvertCArray(arr, N);
}

nlohmann::json ConvertCommand(CapturedCommand const& cmd)
{
	nlohmann::json j;
	j["type"] = cmd->type;
	j["threadId"] = cmd.threadId;

	if (cmd->type == ShaderCommandType::SetRenderTargets)
	{
		j["renderTargets"] = ConvertCArray(cmd->SetRenderTargets.renderTargets, cmd->SetRenderTargets.renderTargetCount);
	}
	else if (cmd->type == ShaderCommandType::SetViewport)
	{
		j["bounds"] = ConvertCArray(cmd->SetViewport.bounds);
		j["minDepth"] = cmd->SetViewport.minDepth;
		j["maxDepth"] = cmd->SetViewport.maxDepth;
	}
	else if (cmd->type == ShaderCommandType::SetScissorRect)
	{
		D3D11_RECT rect = cmd->SetScissorRect.rect;
		j["rect"] = {
			{ "top", rect.top },
			{ "left", rect.left },
			{ "bottom", rect.bottom },
			{ "right", rect.right }
		};
	}
	else if (cmd->type == ShaderCommandType::Clear)
	{
		j["clearFlags"] = cmd->Clear.clearFlags;
		j["colour"] = ConvertCArray(cmd->Clear.colour);
	}
	else if (cmd->type == ShaderCommandType::Draw)
	{
		j["startVertexLocation"] = cmd->Draw.startVertexLocation;
		j["vertexCount"] = cmd->Draw.vertexCount;
	}
	else if (cmd->type == ShaderCommandType::DrawIndexed)
	{
		j["baseVertexLocation"] = cmd->DrawIndexed.baseVertexLocation;
		j["startIndexLocation"] = cmd->DrawIndexed.startIndexLocation;
		j["indexCount"] = cmd->DrawIndexed.indexCount;
	}
	else if (cmd->type == ShaderCommandType::DrawIndexedInstanced)
	{
		j["baseVertexLocation"] = cmd->DrawIndexedInstanced.baseVertexLocation;
		j["startIndexLocation"] = cmd->DrawIndexedInstanced.startIndexLocation;
		j["indexCountPerInstance"] = cmd->DrawIndexedInstanced.indexCountPerInstance;
		j["instanceCount"] = cmd->DrawIndexedInstanced.instanceCount;
	}
	else if (cmd->type == ShaderCommandType::DispatchComputeShader)
	{
		j["threadGroupCountX"] = cmd->DispatchComputeShader.threadGroupCountX;
		j["threadGroupCountY"] = cmd->DispatchComputeShader.threadGroupCountY;
		j["threadGroupCountZ"] = cmd->DispatchComputeShader.threadGroupCountZ;
	}
	else if (cmd->type == ShaderCommandType::CopyTexture)
	{
		j["dstResource"] = uint64_t(cmd->CopyTexture.dstResource);
		j["dstSubresource"] = cmd->CopyTexture.dstSubresource;
		j["dstXY"] = ConvertCArray(cmd->CopyTexture.dstXY);
		j["srcResource"] = uint64_t(cmd->CopyTexture.srcResource);
		j["srcSubresource"] = cmd->CopyTexture.srcSubresource;
		j["useSuppliedRect"] = cmd->CopyTexture.useSuppliedRect;
		j["rect"] = ConvertCArray(cmd->CopyTexture.rect);
	}
	else if (cmd->type == ShaderCommandType::UnknownDraw)
	{
		j["texture"] = uint64_t(cmd->UnknownDraw.texture);
	}
	else if (cmd->type == ShaderCommandType::ResetRendererMaybe)
	{
		j["callback"] = cmd->ResetRendererMaybe.callback;
	}

	j["callstack"] = nlohmann::json::array();
	for (auto const& entry : cmd.callstack)
	{
		j["callstack"].push_back({
			{ "module", entry.module.value_or("unk") },
			{ "symbol", entry.symbol.value_or("unk") },
			{ "address", entry.address },
		});
	}

	return j;
}

void* (*Orig_Context_PushBackEvent)(void* a1, ShaderCommand* cmd);
void* Hook_Context_PushBackEvent(void* a1, ShaderCommand* cmd)
{
	if (g_IsCapturing)
	{
		std::lock_guard<std::mutex> l(g_CaptureMutex);
		g_CapturedCommands.push_back({ *cmd, GetCurrentThreadId(), debug::CaptureCallstack() });
	}

	return Orig_Context_PushBackEvent(a1, cmd);
}

void* (*Orig_GameMain_Update)(void* self);
void* Hook_GameMain_Update(void* self)
{
	if (g_IsCapturing)
	{
		std::vector<CapturedCommand> commands;
		{
			std::lock_guard<std::mutex> l(g_CaptureMutex);
			commands = g_CapturedCommands;
			g_IsCapturing = false;
		}
		Logf("Captured, %u events", commands.size());

		std::filesystem::path p = g_DirPath / "xivr_capture_log.json";
		{
			nlohmann::json j;
			j["commands"] = nlohmann::json::array();
			std::transform(commands.begin(), commands.end(), std::back_inserter(j["commands"]), &ConvertCommand);
			std::ofstream f(p);
			f << j << std::endl;
		}
	}

	if (GetAsyncKeyState(VK_F7) && ch::duration_cast<ch::milliseconds>(ch::high_resolution_clock::now() - g_LastCapture).count() > 250)
	{
		std::lock_guard<std::mutex> l(g_CaptureMutex);

		g_CapturedCommands.clear();
		Logf("Capturing");

		g_IsCapturing = true;
		g_LastCapture = ch::high_resolution_clock::now();
	}

	// todo: we should probably do something with these
	bool exit, restart;
	xr::PollEvents(exit, restart);
	xr::PreTick();
	auto ret = Orig_GameMain_Update(self);
	xr::PostTick();

	return ret;
}

extern "C" void ProcessEventsTrampoline();
extern "C" void* GetRenderContextForThread();

std::unique_ptr<Patcher> g_Patcher;

struct ShaderCommandXIVR
{
	ShaderCommandType type = ShaderCommandType(9);
	std::function<void()> callback;
};

extern "C" void ProcessEventsJumpTable9(ShaderCommandXIVR* cmd)
{
	if (cmd->callback)
		cmd->callback();
}

__int64 (*Orig_RenderManager_RenderUI)(__int64 a1, unsigned __int8 a2);
__int64 Hook_RenderManager_RenderUI(__int64 a1, unsigned __int8 a2)
{
	return 0;
}

__int64 (*Orig_RenderManager_Render)(__int64 a1);
__int64 Hook_RenderManager_Render(__int64 a1)
{
	auto ret = Orig_RenderManager_Render(a1);
	auto context = GetRenderContextForThread();
	auto contextAlloc = game::MakeBaseRelativePtr<void*(void*, size_t)>(0x1D6650);

	auto cmd = (ShaderCommandXIVR*)contextAlloc(context, sizeof(ShaderCommandXIVR));
	new (cmd) ShaderCommandXIVR();
	cmd->callback = [] {
		auto ctx = game::graphics::kernel::Device::Get()->D3DDeviceContext();
		ID3D11RenderTargetView* rtv;
		ctx->OMGetRenderTargets(1, &rtv, nullptr);

		ID3D11Resource* resource;
		rtv->GetResource(&resource);

		ID3D11Texture2D* tex;
		resource->QueryInterface(&tex);

		if (auto session = xr::GetSession())
		{
			session->CopyImageToEye(0, tex);
			session->CopyImageToEye(1, tex);
		}
	};

	Orig_Context_PushBackEvent(context, (ShaderCommand*)cmd);
	return 0;
}

extern "C" __declspec(dllexport) void XIVR_Load(LoadParameters loadParameters)
{
	g_Log = loadParameters.logger;
	g_BaseAddress = loadParameters.baseAddress;
	g_ContextPushBackEventPtr = loadParameters.contextPushBackEventPtr;
	g_DirPath = loadParameters.dirPath;

	g_Patcher.reset(new Patcher());

	auto device = game::graphics::kernel::Device::Get();
	auto featureLevel = device->FeatureLevel();

	Logf("Loaded, base address 0x%X, feature level 0x%X", g_BaseAddress, featureLevel);

	xr::Initialize(true);

	MH_Initialize();

	auto updatePtr = game::MakeBaseRelativePtr<void>(game::offsets::functions::Game_GameMain_Update);
	MH_CreateHook(updatePtr, &Hook_GameMain_Update, (void**)&Orig_GameMain_Update);
	MH_EnableHook(updatePtr);

	MH_CreateHook(g_ContextPushBackEventPtr, &Hook_Context_PushBackEvent, (void**)&Orig_Context_PushBackEvent);
	MH_EnableHook(g_ContextPushBackEventPtr);

	//auto renderUIPtr = game::MakeBaseRelativePtr<void>(0x4B30D0);
	//MH_CreateHook(renderUIPtr, &Hook_RenderManager_RenderUI, (void**)&Orig_RenderManager_RenderUI);
	//MH_EnableHook(renderUIPtr);

	auto renderPtr = game::MakeBaseRelativePtr<void>(0x368AF0);
	MH_CreateHook(renderPtr, &Hook_RenderManager_Render, (void**)&Orig_RenderManager_Render);
	MH_EnableHook(renderPtr);

	// Rewrite the ten 0xCC bytes after the function to be a jump to our minhook bridge function
	uint32_t processEventsPostFunctionOffset = 0x30FE40;
	auto processEventsPostPaddingPtr = game::MakeBaseRelativePtr<void>(processEventsPostFunctionOffset);
	MH_CreateHook(processEventsPostPaddingPtr, &ProcessEventsTrampoline, nullptr);
	MH_EnableHook(processEventsPostPaddingPtr);

	// Change the ninth entry in the jump table to point at our post-function jump
	auto processEventsJumpTablePtr = game::MakeBaseRelativePtr<uint32_t>(0x30FDF8);
	g_Patcher->Patch(game::MakePtr<uint8_t>(processEventsJumpTablePtr, 9 * sizeof(uint32_t)), processEventsPostFunctionOffset);
}

extern "C" __declspec(dllexport) void XIVR_Unload()
{
	MH_DisableHook(MH_ALL_HOOKS);
	MH_Uninitialize();

	xr::Shutdown();

	g_Patcher.reset();
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