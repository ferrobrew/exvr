#include "Log.h"
#include <stdarg.h>
#include <stdio.h>
#include <array>

void Logf(char const* fmt, ...)
{
	std::array<char, 1024> buf;

	va_list ap;
	va_start(ap, fmt);
	vsnprintf(buf.data(), buf.size(), fmt, ap);
	va_end(ap);

	g_Log(buf.data());
}