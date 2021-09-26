# XIVR
Bringing today's Final Fantasy XIV to tomorrow's virtual reality.

## Goals
- To be the first real VRMMORPG by taking a critically acclaimed MMORPG and lifting it into VR.
- We should create a high-quality, comfortable, native experience comparable to an official project.
- VR and non-VR players should be able to play, just like with VRChat.
- It should be possible to play the actual game to completion, even if this is not necessarily the case on day one.
- To the best of our ability, we should be open-source so that we can accept contributions from anyone.
- Walking around the world of Eorzea should inspire joy in our players.
- Our primary target will be standing play, with roomscale and sitting being side concerns.
- The experience we create should entice flatscreen players to get headsets, and for non-players to become players.
- Additional hardware capabilities should be supported where possible, including facial expression tracking, feet tracking, and more.
- The game should remain broadly compatible with existing modifications, including Dalamud and others.
- People should be able to have real social experiences.

## Structure
- `dalamud_host` is the Dalamud plugin responsible for loading the native DLL. It just loads `xivr_native.dll` from whatever directory it is, and attempts to hot-reload it. (the hot-reloading doesn't really work - it should be reworked)
- `scripts` contains miscellaneous scripts for code generation. Code generation should generally be avoided (use macros where possible), but it's useful for generating the skeleton of code to be manually filled in, as is the case with the DX11 vftable hook.
- `src` is where the majority of `xivr` code is. More documentation is required here.