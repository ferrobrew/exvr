#pragma once

typedef void (*LogType)(const char* message);
extern LogType g_Log;

void Logf(char const* fmt, ...);