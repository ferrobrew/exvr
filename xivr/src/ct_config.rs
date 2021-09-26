pub mod rendering {
    pub const DISABLE_ALL: bool = false;
    pub const DISABLE_GAME: bool = false || DISABLE_ALL;
    pub const DISABLE_UI: bool = true || DISABLE_ALL;

    pub const CAPTURE_D3D_COMMANDS: bool = false;

    pub const SHADER_COMMAND_HIJACKED_TYPE: usize = 9;
}

pub mod xr {
    // temporary settings while I fix other code
    pub const CHANGE_WINDOW_SIZE: bool = true;
    pub const RUN_XR_PER_FRAME: bool = true;

    pub const VIEW_COUNT: u32 = 2;
}
