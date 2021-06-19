#include "xr/XRUtils.h"
#include <fmt/format.h>

#include "debug/CaptureCallstack.h"
#include "game/Util.h"
#include "Log.h"

void ExitWithError(const char* fmt, ...)
{
	char buf[1024];
	va_list argptr;
	va_start(argptr, fmt);
	int len = vsnprintf(buf, 1024, fmt, argptr);
	va_end(argptr);

	std::string output = buf;
	auto callstack = debug::CaptureCallstack();
	output += fmt::format("\nCallstack ({} entries)", callstack.size());
	for (auto const& entry : callstack)
	{
		output += fmt::format(
			"\n  {}:{:X} ({})",
			entry.module.value_or("unk"),
			entry.address - (entry.module.has_value() ? 0u : uint64_t(g_BaseAddress)),
			entry.symbol.value_or("unk"));
	}

#ifdef WIN32
	MessageBoxA(nullptr, output.c_str(), "Fatal Error!", MB_OK);
#endif
	DebugBreak();
	Logf("%s\n", buf);
	exit(-1);
}

XrResult CheckXrResult(XrResult res, const char* originator, const char* sourceLocation)
{
	if (XR_FAILED(res))
	{
		ExitWithError("XR failure! Code: %s", to_string(res));
	}

	return res;
}
