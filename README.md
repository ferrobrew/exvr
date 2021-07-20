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
- The experience we create should entice FFXIV players to get headsets, and for non-players to become players.
- Additional hardware capabilities should be supported where possible, including facial expression tracking, feet tracking, and more.
- The game should remain broadly compatible with existing modifications, including Dalamud and others.
- People should be able to have real social experiences.

## Tasks
- Stereo rendering
    - Writing an IDA script to export all functions beginning/end with names
    - Determining when the final composited game content is copied from (I)
    - Running the renderer twice and capturing the outputs

- Headset rendering
    - Applying headset matrix to camera matrix
    - Fixing camera FOV

- Gameplay considerations
    - Basic motion controller mapping to hotbars
        - hold button down to bring up rings that you target with your controller
        - hold button down to target someone else
    - Basic movement controls
    - Basic voice controls
    - Drop to stereo cinema rendering for cutscenes
        - Optional mode to render from your character's perspective if you're in the cutscene?

- HUD rendering
    - Capture all HUD output, including Dalamud, and render to a curved cylinder
    - Remap mouse movement to account for this
    - Allow for use of motion controls to interact with HUD

- Character IK
    - Get first-person model rendering
    - Find out how bones are driven (check out existing pose tools in ecosystem)
    - Build a basic bone calibration tool
        - maybe with lovr
        - write to json that can be loaded by xivr
        - need to get feet, knees, hips, shoulders, elbows
    - Figure out how to remap skeletons between characters of different sizes
    - Apply IK data to local character
    - Consider using handtracking data if available?

- (Character IK) Multiplayer
    - Enet probably?
    - Set up "servers" for each realm
        - Could be virtual servers
    - Find out how the game authenticates and use its login token to log into our server
    - send IK data to the server
    - broadcast IK data to everyone else
    - apply IK data locally

- (Multiplayer) Voice chat
    - Figure out how existing voice chat protocols work
    - Localisation of voice chat to position and orientation

- (Stereo rendering, Headset rendering, HUD rendering, Multiplayer) Public announcement