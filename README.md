# exvr
Virtual reality is a nascent field with a significant amount of potential. It is criminally underserved by its current ecosystem; highly-social, immersive experiences are possible today, but the state of affairs is that there isn't enough of a market to allow for these to be developed by the traditional games industry.

It doesn't have to be that way. Through extensive reverse engineering and the construction of a shared framework, we can look at bringing VR to flatscreen games in a way that both honours them and makes them so, _so_ much more.

You've always been able to create characters in games and interact with other players. The only thing stopping us from _being_ our characters is execution.

## Objective
To build a general-purpose framework for adding VR to existing flatscreen games, with the following high-level goals to guide us (excerpt from `xivr` readme):

- We should create a high-quality, comfortable, native experience comparable to an official project.
- VR and non-VR players should be able to play, just like with VRChat.
- It should be possible to play the actual game to completion, even if this is not necessarily the case on day one.
- To the best of our ability, we should be open-source so that we can accept contributions from anyone.
- The experience we create should entice flatscreen players to get headsets, and for non-players to become players.
- Additional hardware capabilities should be supported where possible, including facial expression tracking, feet tracking, and more.
- People should be able to have real social experiences.

This will necessitate a significant amount of work, including per-game reverse engineering, the development of generic VR abstractions, discovering new fields of VR design, external networking for the synchronisation of pose data, infrastructure around servers, and much more. It is not a project I intend to complete alone.

## Projects
### xivr
The lead project of exvr is `xivr`, an experimental project to bring Final Fantasy XIV to VR. It does not work, but it could. Only time and effort will tell.

### common
Contains the crates that are shared between `exvr` projects. As `xivr` code matures, more of it will be moved to `common`, and `xivr` will consume `common` code to provide its functionality.