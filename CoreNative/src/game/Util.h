#pragma once

#include <cstdint>
#include <minwindef.h>

extern void* g_BaseAddress;
extern HMODULE g_Module;

namespace game
{
template<typename Ty, typename PtrTy>
inline Ty* MakePtr(PtrTy address, ptrdiff_t offset)
{
	return (Ty*)(size_t(address) + offset);
}

template<typename Ty>
inline Ty* MakeBaseRelativePtr(ptrdiff_t offset)
{
	return game::MakePtr<Ty>(g_BaseAddress, offset);
}

template<typename Ty, typename PtrTy>
inline Ty DerefPtr(PtrTy address, ptrdiff_t offset)
{
	return *game::MakePtr<Ty>(address, offset);
}

template<typename Ty>
inline Ty DerefBaseRelativePtr(ptrdiff_t offset)
{
	return game::DerefPtr<Ty>(g_BaseAddress, offset);
}
}  // namespace game

#define OFFSET_PROPERTY(type, name, offset) \
	inline type& name() { return *game::MakePtr<type>(this, offset); }