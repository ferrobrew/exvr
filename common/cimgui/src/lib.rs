use std::ffi::{CString, NulError};
use std::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[rustfmt::skip] pub fn align_text_to_frame_padding() { unsafe { sys::igAlignTextToFramePadding(); } }
#[rustfmt::skip] pub fn begin_drag_drop_target() { unsafe { sys::igBeginDragDropTarget(); } }
#[rustfmt::skip] pub fn begin_group() { unsafe { sys::igBeginGroup(); } }
#[rustfmt::skip] pub fn begin_main_menu_bar() { unsafe { sys::igBeginMainMenuBar(); } }
#[rustfmt::skip] pub fn begin_menu_bar() { unsafe { sys::igBeginMenuBar(); } }
#[rustfmt::skip] pub fn begin_tooltip() { unsafe { sys::igBeginTooltip(); } }
#[rustfmt::skip] pub fn bullet() { unsafe { sys::igBullet(); } }
#[rustfmt::skip] pub fn close_current_popup() { unsafe { sys::igCloseCurrentPopup(); } }
#[rustfmt::skip] pub fn end() { unsafe { sys::igEnd(); } }
#[rustfmt::skip] pub fn end_child() { unsafe { sys::igEndChild(); } }
#[rustfmt::skip] pub fn end_child_frame() { unsafe { sys::igEndChildFrame(); } }
#[rustfmt::skip] pub fn end_combo() { unsafe { sys::igEndCombo(); } }
#[rustfmt::skip] pub fn end_drag_drop_source() { unsafe { sys::igEndDragDropSource(); } }
#[rustfmt::skip] pub fn end_drag_drop_target() { unsafe { sys::igEndDragDropTarget(); } }
#[rustfmt::skip] pub fn end_frame() { unsafe { sys::igEndFrame(); } }
#[rustfmt::skip] pub fn end_group() { unsafe { sys::igEndGroup(); } }
#[rustfmt::skip] pub fn end_list_box() { unsafe { sys::igEndListBox(); } }
#[rustfmt::skip] pub fn end_main_menu_bar() { unsafe { sys::igEndMainMenuBar(); } }
#[rustfmt::skip] pub fn end_menu() { unsafe { sys::igEndMenu(); } }
#[rustfmt::skip] pub fn end_menu_bar() { unsafe { sys::igEndMenuBar(); } }
#[rustfmt::skip] pub fn end_popup() { unsafe { sys::igEndPopup(); } }
#[rustfmt::skip] pub fn end_tab_bar() { unsafe { sys::igEndTabBar(); } }
#[rustfmt::skip] pub fn end_tab_item() { unsafe { sys::igEndTabItem(); } }
#[rustfmt::skip] pub fn end_table() { unsafe { sys::igEndTable(); } }
#[rustfmt::skip] pub fn end_tooltip() { unsafe { sys::igEndTooltip(); } }
#[rustfmt::skip] pub fn is_any_item_active() { unsafe { sys::igIsAnyItemActive(); } }
#[rustfmt::skip] pub fn is_any_item_focused() { unsafe { sys::igIsAnyItemFocused(); } }
#[rustfmt::skip] pub fn is_any_item_hovered() { unsafe { sys::igIsAnyItemHovered(); } }
#[rustfmt::skip] pub fn is_any_mouse_down() { unsafe { sys::igIsAnyMouseDown(); } }
#[rustfmt::skip] pub fn is_item_activated() { unsafe { sys::igIsItemActivated(); } }
#[rustfmt::skip] pub fn is_item_active() { unsafe { sys::igIsItemActive(); } }
#[rustfmt::skip] pub fn is_item_deactivated() { unsafe { sys::igIsItemDeactivated(); } }
#[rustfmt::skip] pub fn is_item_deactivated_after_edit() { unsafe { sys::igIsItemDeactivatedAfterEdit(); } }
#[rustfmt::skip] pub fn is_item_edited() { unsafe { sys::igIsItemEdited(); } }
#[rustfmt::skip] pub fn is_item_focused() { unsafe { sys::igIsItemFocused(); } }
#[rustfmt::skip] pub fn is_item_toggled_open() { unsafe { sys::igIsItemToggledOpen(); } }
#[rustfmt::skip] pub fn is_item_visible() { unsafe { sys::igIsItemVisible(); } }
#[rustfmt::skip] pub fn is_window_appearing() { unsafe { sys::igIsWindowAppearing(); } }
#[rustfmt::skip] pub fn is_window_collapsed() { unsafe { sys::igIsWindowCollapsed(); } }
#[rustfmt::skip] pub fn log_buttons() { unsafe { sys::igLogButtons(); } }
#[rustfmt::skip] pub fn log_finish() { unsafe { sys::igLogFinish(); } }
#[rustfmt::skip] pub fn new_frame() { unsafe { sys::igNewFrame(); } }
#[rustfmt::skip] pub fn new_line() { unsafe { sys::igNewLine(); } }
#[rustfmt::skip] pub fn next_column() { unsafe { sys::igNextColumn(); } }
#[rustfmt::skip] pub fn pop_allow_keyboard_focus() { unsafe { sys::igPopAllowKeyboardFocus(); } }
#[rustfmt::skip] pub fn pop_button_repeat() { unsafe { sys::igPopButtonRepeat(); } }
#[rustfmt::skip] pub fn pop_clip_rect() { unsafe { sys::igPopClipRect(); } }
#[rustfmt::skip] pub fn pop_font() { unsafe { sys::igPopFont(); } }
#[rustfmt::skip] pub fn pop_id() { unsafe { sys::igPopID(); } }
#[rustfmt::skip] pub fn pop_item_width() { unsafe { sys::igPopItemWidth(); } }
#[rustfmt::skip] pub fn pop_text_wrap_pos() { unsafe { sys::igPopTextWrapPos(); } }
#[rustfmt::skip] pub fn render() { unsafe { sys::igRender(); } }
#[rustfmt::skip] pub fn separator() { unsafe { sys::igSeparator(); } }
#[rustfmt::skip] pub fn set_item_allow_overlap() { unsafe { sys::igSetItemAllowOverlap(); } }
#[rustfmt::skip] pub fn set_item_default_focus() { unsafe { sys::igSetItemDefaultFocus(); } }
#[rustfmt::skip] pub fn set_next_window_focus() { unsafe { sys::igSetNextWindowFocus(); } }
#[rustfmt::skip] pub fn show_user_guide() { unsafe { sys::igShowUserGuide(); } }
#[rustfmt::skip] pub fn spacing() { unsafe { sys::igSpacing(); } }
#[rustfmt::skip] pub fn table_headers_row() { unsafe { sys::igTableHeadersRow(); } }
#[rustfmt::skip] pub fn table_next_column() { unsafe { sys::igTableNextColumn(); } }
#[rustfmt::skip] pub fn tree_pop() { unsafe { sys::igTreePop(); } }

pub use sys::igSetAllocatorFunctions as set_allocator_functions;
pub use sys::igSetCurrentContext as set_current_context;

pub use sys::ImGuiID as ID;
pub use sys::ImTextureID as TextureID;

pub use sys::ImGuiContext as Context;
pub use sys::ImGuiMemAllocFunc as MemAllocFunc;
pub use sys::ImGuiMemFreeFunc as MemFreeFunc;

pub type Vec2 = sys::ImVec2;
impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

pub type Vec4 = sys::ImVec4;
impl Vec4 {
    pub const ZERO: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const ONE: Vec4 = Vec4 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }
}

pub type Color = sys::ImColor;
impl Color {
    pub const ZERO: Color = Color { Value: Vec4::ZERO };
    pub const ONE: Color = Color { Value: Vec4::ONE };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            Value: Vec4::new(r, g, b, a),
        }
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Color {
        let mut col = MaybeUninit::<Color>::uninit();
        unsafe {
            sys::ImColor_SetHSV(col.as_mut_ptr(), h, s, v, 1.0);
            col.assume_init()
        }
    }
}
impl From<Color> for Vec4 {
    fn from(color: Color) -> Self {
        color.Value
    }
}

#[repr(i32)]
pub enum WindowFlags {
    None = sys::ImGuiWindowFlags_None,
    NoTitleBar = sys::ImGuiWindowFlags_NoTitleBar,
    NoResize = sys::ImGuiWindowFlags_NoResize,
    NoMove = sys::ImGuiWindowFlags_NoMove,
    NoScrollbar = sys::ImGuiWindowFlags_NoScrollbar,
    NoScrollWithMouse = sys::ImGuiWindowFlags_NoScrollWithMouse,
    NoCollapse = sys::ImGuiWindowFlags_NoCollapse,
    AlwaysAutoResize = sys::ImGuiWindowFlags_AlwaysAutoResize,
    NoBackground = sys::ImGuiWindowFlags_NoBackground,
    NoSavedSettings = sys::ImGuiWindowFlags_NoSavedSettings,
    NoMouseInputs = sys::ImGuiWindowFlags_NoMouseInputs,
    MenuBar = sys::ImGuiWindowFlags_MenuBar,
    HorizontalScrollbar = sys::ImGuiWindowFlags_HorizontalScrollbar,
    NoFocusOnAppearing = sys::ImGuiWindowFlags_NoFocusOnAppearing,
    NoBringToFrontOnFocus = sys::ImGuiWindowFlags_NoBringToFrontOnFocus,
    AlwaysVerticalScrollbar = sys::ImGuiWindowFlags_AlwaysVerticalScrollbar,
    AlwaysHorizontalScrollbar = sys::ImGuiWindowFlags_AlwaysHorizontalScrollbar,
    AlwaysUseWindowPadding = sys::ImGuiWindowFlags_AlwaysUseWindowPadding,
    NoNavInputs = sys::ImGuiWindowFlags_NoNavInputs,
    NoNavFocus = sys::ImGuiWindowFlags_NoNavFocus,
    UnsavedDocument = sys::ImGuiWindowFlags_UnsavedDocument,
    NoNav = sys::ImGuiWindowFlags_NoNav,
    NoDecoration = sys::ImGuiWindowFlags_NoDecoration,
    NoInputs = sys::ImGuiWindowFlags_NoInputs,
    NavFlattened = sys::ImGuiWindowFlags_NavFlattened,
    ChildWindow = sys::ImGuiWindowFlags_ChildWindow,
    Tooltip = sys::ImGuiWindowFlags_Tooltip,
    Popup = sys::ImGuiWindowFlags_Popup,
    Modal = sys::ImGuiWindowFlags_Modal,
    ChildMenu = sys::ImGuiWindowFlags_ChildMenu,
}

#[repr(i32)]
pub enum InputTextFlags {
    None = sys::ImGuiInputTextFlags_None,
    CharsDecimal = sys::ImGuiInputTextFlags_CharsDecimal,
    CharsHexadecimal = sys::ImGuiInputTextFlags_CharsHexadecimal,
    CharsUppercase = sys::ImGuiInputTextFlags_CharsUppercase,
    CharsNoBlank = sys::ImGuiInputTextFlags_CharsNoBlank,
    AutoSelectAll = sys::ImGuiInputTextFlags_AutoSelectAll,
    EnterReturnsTrue = sys::ImGuiInputTextFlags_EnterReturnsTrue,
    CallbackCompletion = sys::ImGuiInputTextFlags_CallbackCompletion,
    CallbackHistory = sys::ImGuiInputTextFlags_CallbackHistory,
    CallbackAlways = sys::ImGuiInputTextFlags_CallbackAlways,
    CallbackCharFilter = sys::ImGuiInputTextFlags_CallbackCharFilter,
    AllowTabInput = sys::ImGuiInputTextFlags_AllowTabInput,
    CtrlEnterForNewLine = sys::ImGuiInputTextFlags_CtrlEnterForNewLine,
    NoHorizontalScroll = sys::ImGuiInputTextFlags_NoHorizontalScroll,
    AlwaysOverwrite = sys::ImGuiInputTextFlags_AlwaysOverwrite,
    ReadOnly = sys::ImGuiInputTextFlags_ReadOnly,
    Password = sys::ImGuiInputTextFlags_Password,
    NoUndoRedo = sys::ImGuiInputTextFlags_NoUndoRedo,
    CharsScientific = sys::ImGuiInputTextFlags_CharsScientific,
    CallbackResize = sys::ImGuiInputTextFlags_CallbackResize,
    CallbackEdit = sys::ImGuiInputTextFlags_CallbackEdit,
    Multiline = sys::ImGuiInputTextFlags_Multiline,
    NoMarkEdited = sys::ImGuiInputTextFlags_NoMarkEdited,
}

#[repr(i32)]
pub enum TreeNodeFlags {
    None = sys::ImGuiTreeNodeFlags_None,
    Selected = sys::ImGuiTreeNodeFlags_Selected,
    Framed = sys::ImGuiTreeNodeFlags_Framed,
    AllowItemOverlap = sys::ImGuiTreeNodeFlags_AllowItemOverlap,
    NoTreePushOnOpen = sys::ImGuiTreeNodeFlags_NoTreePushOnOpen,
    NoAutoOpenOnLog = sys::ImGuiTreeNodeFlags_NoAutoOpenOnLog,
    DefaultOpen = sys::ImGuiTreeNodeFlags_DefaultOpen,
    OpenOnDoubleClick = sys::ImGuiTreeNodeFlags_OpenOnDoubleClick,
    OpenOnArrow = sys::ImGuiTreeNodeFlags_OpenOnArrow,
    Leaf = sys::ImGuiTreeNodeFlags_Leaf,
    Bullet = sys::ImGuiTreeNodeFlags_Bullet,
    FramePadding = sys::ImGuiTreeNodeFlags_FramePadding,
    SpanAvailWidth = sys::ImGuiTreeNodeFlags_SpanAvailWidth,
    SpanFullWidth = sys::ImGuiTreeNodeFlags_SpanFullWidth,
    NavLeftJumpsBackHere = sys::ImGuiTreeNodeFlags_NavLeftJumpsBackHere,
    CollapsingHeader = sys::ImGuiTreeNodeFlags_CollapsingHeader,
}

#[repr(i32)]
pub enum PopupFlags {
    MouseButtonLeft = sys::ImGuiPopupFlags_MouseButtonLeft,
    MouseButtonRight = sys::ImGuiPopupFlags_MouseButtonRight,
    MouseButtonMiddle = sys::ImGuiPopupFlags_MouseButtonMiddle,
    MouseButtonMask = sys::ImGuiPopupFlags_MouseButtonMask_,
    NoOpenOverExistingPopup = sys::ImGuiPopupFlags_NoOpenOverExistingPopup,
    NoOpenOverItems = sys::ImGuiPopupFlags_NoOpenOverItems,
    AnyPopupId = sys::ImGuiPopupFlags_AnyPopupId,
    AnyPopupLevel = sys::ImGuiPopupFlags_AnyPopupLevel,
    AnyPopup = sys::ImGuiPopupFlags_AnyPopup,
}

#[repr(i32)]
pub enum SelectableFlags {
    None = sys::ImGuiSelectableFlags_None,
    DontClosePopups = sys::ImGuiSelectableFlags_DontClosePopups,
    SpanAllColumns = sys::ImGuiSelectableFlags_SpanAllColumns,
    AllowDoubleClick = sys::ImGuiSelectableFlags_AllowDoubleClick,
    Disabled = sys::ImGuiSelectableFlags_Disabled,
    AllowItemOverlap = sys::ImGuiSelectableFlags_AllowItemOverlap,
}

#[repr(i32)]
pub enum ComboFlags {
    None = sys::ImGuiComboFlags_None,
    PopupAlignLeft = sys::ImGuiComboFlags_PopupAlignLeft,
    HeightSmall = sys::ImGuiComboFlags_HeightSmall,
    HeightRegular = sys::ImGuiComboFlags_HeightRegular,
    HeightLarge = sys::ImGuiComboFlags_HeightLarge,
    HeightLargest = sys::ImGuiComboFlags_HeightLargest,
    NoArrowButton = sys::ImGuiComboFlags_NoArrowButton,
    NoPreview = sys::ImGuiComboFlags_NoPreview,
    HeightMask = sys::ImGuiComboFlags_HeightMask_,
}

#[repr(i32)]
pub enum TabBarFlags {
    None = sys::ImGuiTabBarFlags_None,
    Reorderable = sys::ImGuiTabBarFlags_Reorderable,
    AutoSelectNewTabs = sys::ImGuiTabBarFlags_AutoSelectNewTabs,
    TabListPopupButton = sys::ImGuiTabBarFlags_TabListPopupButton,
    NoCloseWithMiddleMouseButton = sys::ImGuiTabBarFlags_NoCloseWithMiddleMouseButton,
    NoTabListScrollingButtons = sys::ImGuiTabBarFlags_NoTabListScrollingButtons,
    NoTooltip = sys::ImGuiTabBarFlags_NoTooltip,
    FittingPolicyResizeDown = sys::ImGuiTabBarFlags_FittingPolicyResizeDown,
    FittingPolicyScroll = sys::ImGuiTabBarFlags_FittingPolicyScroll,
    FittingPolicyMask = sys::ImGuiTabBarFlags_FittingPolicyMask_,
}

#[repr(i32)]
pub enum TabItemFlags {
    None = sys::ImGuiTabItemFlags_None,
    UnsavedDocument = sys::ImGuiTabItemFlags_UnsavedDocument,
    SetSelected = sys::ImGuiTabItemFlags_SetSelected,
    NoCloseWithMiddleMouseButton = sys::ImGuiTabItemFlags_NoCloseWithMiddleMouseButton,
    NoPushId = sys::ImGuiTabItemFlags_NoPushId,
    NoTooltip = sys::ImGuiTabItemFlags_NoTooltip,
    NoReorder = sys::ImGuiTabItemFlags_NoReorder,
    Leading = sys::ImGuiTabItemFlags_Leading,
    Trailing = sys::ImGuiTabItemFlags_Trailing,
}

#[repr(i32)]
pub enum TableFlags {
    None = sys::ImGuiTableFlags_None,
    Resizable = sys::ImGuiTableFlags_Resizable,
    Reorderable = sys::ImGuiTableFlags_Reorderable,
    Hideable = sys::ImGuiTableFlags_Hideable,
    Sortable = sys::ImGuiTableFlags_Sortable,
    NoSavedSettings = sys::ImGuiTableFlags_NoSavedSettings,
    ContextMenuInBody = sys::ImGuiTableFlags_ContextMenuInBody,
    RowBg = sys::ImGuiTableFlags_RowBg,
    BordersInnerH = sys::ImGuiTableFlags_BordersInnerH,
    BordersOuterH = sys::ImGuiTableFlags_BordersOuterH,
    BordersInnerV = sys::ImGuiTableFlags_BordersInnerV,
    BordersOuterV = sys::ImGuiTableFlags_BordersOuterV,
    BordersH = sys::ImGuiTableFlags_BordersH,
    BordersV = sys::ImGuiTableFlags_BordersV,
    BordersInner = sys::ImGuiTableFlags_BordersInner,
    BordersOuter = sys::ImGuiTableFlags_BordersOuter,
    Borders = sys::ImGuiTableFlags_Borders,
    NoBordersInBody = sys::ImGuiTableFlags_NoBordersInBody,
    NoBordersInBodyUntilResize = sys::ImGuiTableFlags_NoBordersInBodyUntilResize,
    SizingFixedFit = sys::ImGuiTableFlags_SizingFixedFit,
    SizingFixedSame = sys::ImGuiTableFlags_SizingFixedSame,
    SizingStretchProp = sys::ImGuiTableFlags_SizingStretchProp,
    SizingStretchSame = sys::ImGuiTableFlags_SizingStretchSame,
    NoHostExtendX = sys::ImGuiTableFlags_NoHostExtendX,
    NoHostExtendY = sys::ImGuiTableFlags_NoHostExtendY,
    NoKeepColumnsVisible = sys::ImGuiTableFlags_NoKeepColumnsVisible,
    PreciseWidths = sys::ImGuiTableFlags_PreciseWidths,
    NoClip = sys::ImGuiTableFlags_NoClip,
    PadOuterX = sys::ImGuiTableFlags_PadOuterX,
    NoPadOuterX = sys::ImGuiTableFlags_NoPadOuterX,
    NoPadInnerX = sys::ImGuiTableFlags_NoPadInnerX,
    ScrollX = sys::ImGuiTableFlags_ScrollX,
    ScrollY = sys::ImGuiTableFlags_ScrollY,
    SortMulti = sys::ImGuiTableFlags_SortMulti,
    SortTristate = sys::ImGuiTableFlags_SortTristate,
    SizingMask = sys::ImGuiTableFlags_SizingMask_,
}

#[repr(i32)]
pub enum TableColumnFlags {
    None = sys::ImGuiTableColumnFlags_None,
    DefaultHide = sys::ImGuiTableColumnFlags_DefaultHide,
    DefaultSort = sys::ImGuiTableColumnFlags_DefaultSort,
    WidthStretch = sys::ImGuiTableColumnFlags_WidthStretch,
    WidthFixed = sys::ImGuiTableColumnFlags_WidthFixed,
    NoResize = sys::ImGuiTableColumnFlags_NoResize,
    NoReorder = sys::ImGuiTableColumnFlags_NoReorder,
    NoHide = sys::ImGuiTableColumnFlags_NoHide,
    NoClip = sys::ImGuiTableColumnFlags_NoClip,
    NoSort = sys::ImGuiTableColumnFlags_NoSort,
    NoSortAscending = sys::ImGuiTableColumnFlags_NoSortAscending,
    NoSortDescending = sys::ImGuiTableColumnFlags_NoSortDescending,
    NoHeaderWidth = sys::ImGuiTableColumnFlags_NoHeaderWidth,
    PreferSortAscending = sys::ImGuiTableColumnFlags_PreferSortAscending,
    PreferSortDescending = sys::ImGuiTableColumnFlags_PreferSortDescending,
    IndentEnable = sys::ImGuiTableColumnFlags_IndentEnable,
    IndentDisable = sys::ImGuiTableColumnFlags_IndentDisable,
    IsEnabled = sys::ImGuiTableColumnFlags_IsEnabled,
    IsVisible = sys::ImGuiTableColumnFlags_IsVisible,
    IsSorted = sys::ImGuiTableColumnFlags_IsSorted,
    IsHovered = sys::ImGuiTableColumnFlags_IsHovered,
    WidthMask = sys::ImGuiTableColumnFlags_WidthMask_,
    IndentMask = sys::ImGuiTableColumnFlags_IndentMask_,
    StatusMask = sys::ImGuiTableColumnFlags_StatusMask_,
    NoDirectResize = sys::ImGuiTableColumnFlags_NoDirectResize_,
}

#[repr(i32)]
pub enum TableRowFlags {
    None = sys::ImGuiTableRowFlags_None,
    Headers = sys::ImGuiTableRowFlags_Headers,
}

#[repr(i32)]
pub enum TableBgTarget {
    None = sys::ImGuiTableBgTarget_None,
    RowBg0 = sys::ImGuiTableBgTarget_RowBg0,
    RowBg1 = sys::ImGuiTableBgTarget_RowBg1,
    CellBg = sys::ImGuiTableBgTarget_CellBg,
}

#[repr(i32)]
pub enum FocusedFlags {
    None = sys::ImGuiFocusedFlags_None,
    ChildWindows = sys::ImGuiFocusedFlags_ChildWindows,
    RootWindow = sys::ImGuiFocusedFlags_RootWindow,
    AnyWindow = sys::ImGuiFocusedFlags_AnyWindow,
    RootAndChildWindows = sys::ImGuiFocusedFlags_RootAndChildWindows,
}

#[repr(i32)]
pub enum HoveredFlags {
    None = sys::ImGuiHoveredFlags_None,
    ChildWindows = sys::ImGuiHoveredFlags_ChildWindows,
    RootWindow = sys::ImGuiHoveredFlags_RootWindow,
    AnyWindow = sys::ImGuiHoveredFlags_AnyWindow,
    AllowWhenBlockedByPopup = sys::ImGuiHoveredFlags_AllowWhenBlockedByPopup,
    AllowWhenBlockedByActiveItem = sys::ImGuiHoveredFlags_AllowWhenBlockedByActiveItem,
    AllowWhenOverlapped = sys::ImGuiHoveredFlags_AllowWhenOverlapped,
    AllowWhenDisabled = sys::ImGuiHoveredFlags_AllowWhenDisabled,
    RectOnly = sys::ImGuiHoveredFlags_RectOnly,
    RootAndChildWindows = sys::ImGuiHoveredFlags_RootAndChildWindows,
}

#[repr(i32)]
pub enum DragDropFlags {
    None = sys::ImGuiDragDropFlags_None,
    SourceNoPreviewTooltip = sys::ImGuiDragDropFlags_SourceNoPreviewTooltip,
    SourceNoDisableHover = sys::ImGuiDragDropFlags_SourceNoDisableHover,
    SourceNoHoldToOpenOthers = sys::ImGuiDragDropFlags_SourceNoHoldToOpenOthers,
    SourceAllowNullID = sys::ImGuiDragDropFlags_SourceAllowNullID,
    SourceExtern = sys::ImGuiDragDropFlags_SourceExtern,
    SourceAutoExpirePayload = sys::ImGuiDragDropFlags_SourceAutoExpirePayload,
    AcceptBeforeDelivery = sys::ImGuiDragDropFlags_AcceptBeforeDelivery,
    AcceptNoDrawDefaultRect = sys::ImGuiDragDropFlags_AcceptNoDrawDefaultRect,
    AcceptNoPreviewTooltip = sys::ImGuiDragDropFlags_AcceptNoPreviewTooltip,
    AcceptPeekOnly = sys::ImGuiDragDropFlags_AcceptPeekOnly,
}

#[repr(i32)]
pub enum Dir {
    None = sys::ImGuiDir_None,
    Left = sys::ImGuiDir_Left,
    Right = sys::ImGuiDir_Right,
    Up = sys::ImGuiDir_Up,
    Down = sys::ImGuiDir_Down,
    COUNT = sys::ImGuiDir_COUNT,
}

#[repr(i32)]
pub enum SortDirection {
    None = sys::ImGuiSortDirection_None,
    Ascending = sys::ImGuiSortDirection_Ascending,
    Descending = sys::ImGuiSortDirection_Descending,
}

#[repr(i32)]
pub enum KeyModFlags {
    None = sys::ImGuiKeyModFlags_None,
    Ctrl = sys::ImGuiKeyModFlags_Ctrl,
    Shift = sys::ImGuiKeyModFlags_Shift,
    Alt = sys::ImGuiKeyModFlags_Alt,
    Super = sys::ImGuiKeyModFlags_Super,
}

#[repr(i32)]
pub enum ConfigFlags {
    None = sys::ImGuiConfigFlags_None,
    NavEnableKeyboard = sys::ImGuiConfigFlags_NavEnableKeyboard,
    NavEnableGamepad = sys::ImGuiConfigFlags_NavEnableGamepad,
    NavEnableSetMousePos = sys::ImGuiConfigFlags_NavEnableSetMousePos,
    NavNoCaptureKeyboard = sys::ImGuiConfigFlags_NavNoCaptureKeyboard,
    NoMouse = sys::ImGuiConfigFlags_NoMouse,
    NoMouseCursorChange = sys::ImGuiConfigFlags_NoMouseCursorChange,
    IsSRGB = sys::ImGuiConfigFlags_IsSRGB,
    IsTouchScreen = sys::ImGuiConfigFlags_IsTouchScreen,
}

#[repr(i32)]
pub enum BackendFlags {
    None = sys::ImGuiBackendFlags_None,
    HasGamepad = sys::ImGuiBackendFlags_HasGamepad,
    HasMouseCursors = sys::ImGuiBackendFlags_HasMouseCursors,
    HasSetMousePos = sys::ImGuiBackendFlags_HasSetMousePos,
    RendererHasVtxOffset = sys::ImGuiBackendFlags_RendererHasVtxOffset,
}

#[repr(i32)]
pub enum ButtonFlags {
    None = sys::ImGuiButtonFlags_None,
    MouseButtonLeft = sys::ImGuiButtonFlags_MouseButtonLeft,
    MouseButtonRight = sys::ImGuiButtonFlags_MouseButtonRight,
    MouseButtonMiddle = sys::ImGuiButtonFlags_MouseButtonMiddle,
    MouseButtonMask = sys::ImGuiButtonFlags_MouseButtonMask_,
}

#[repr(i32)]
pub enum ColorEditFlags {
    None = sys::ImGuiColorEditFlags_None,
    NoAlpha = sys::ImGuiColorEditFlags_NoAlpha,
    NoPicker = sys::ImGuiColorEditFlags_NoPicker,
    NoOptions = sys::ImGuiColorEditFlags_NoOptions,
    NoSmallPreview = sys::ImGuiColorEditFlags_NoSmallPreview,
    NoInputs = sys::ImGuiColorEditFlags_NoInputs,
    NoTooltip = sys::ImGuiColorEditFlags_NoTooltip,
    NoLabel = sys::ImGuiColorEditFlags_NoLabel,
    NoSidePreview = sys::ImGuiColorEditFlags_NoSidePreview,
    NoDragDrop = sys::ImGuiColorEditFlags_NoDragDrop,
    NoBorder = sys::ImGuiColorEditFlags_NoBorder,
    AlphaBar = sys::ImGuiColorEditFlags_AlphaBar,
    AlphaPreview = sys::ImGuiColorEditFlags_AlphaPreview,
    AlphaPreviewHalf = sys::ImGuiColorEditFlags_AlphaPreviewHalf,
    HDR = sys::ImGuiColorEditFlags_HDR,
    DisplayRGB = sys::ImGuiColorEditFlags_DisplayRGB,
    DisplayHSV = sys::ImGuiColorEditFlags_DisplayHSV,
    DisplayHex = sys::ImGuiColorEditFlags_DisplayHex,
    Uint8 = sys::ImGuiColorEditFlags_Uint8,
    Float = sys::ImGuiColorEditFlags_Float,
    PickerHueBar = sys::ImGuiColorEditFlags_PickerHueBar,
    PickerHueWheel = sys::ImGuiColorEditFlags_PickerHueWheel,
    InputRGB = sys::ImGuiColorEditFlags_InputRGB,
    InputHSV = sys::ImGuiColorEditFlags_InputHSV,
    DefaultOptions_ = sys::ImGuiColorEditFlags_DefaultOptions_,
    DisplayMask = sys::ImGuiColorEditFlags_DisplayMask_,
    DataTypeMask = sys::ImGuiColorEditFlags_DataTypeMask_,
    PickerMask = sys::ImGuiColorEditFlags_PickerMask_,
    InputMask = sys::ImGuiColorEditFlags_InputMask_,
}

#[repr(i32)]
pub enum SliderFlags {
    None = sys::ImGuiSliderFlags_None,
    AlwaysClamp = sys::ImGuiSliderFlags_AlwaysClamp,
    Logarithmic = sys::ImGuiSliderFlags_Logarithmic,
    NoRoundToFormat = sys::ImGuiSliderFlags_NoRoundToFormat,
    NoInput = sys::ImGuiSliderFlags_NoInput,
    InvalidMask = sys::ImGuiSliderFlags_InvalidMask_,
}

#[repr(i32)]
pub enum MouseButton {
    Left = sys::ImGuiMouseButton_Left,
    Right = sys::ImGuiMouseButton_Right,
    Middle = sys::ImGuiMouseButton_Middle,
    COUNT = sys::ImGuiMouseButton_COUNT,
}

#[repr(i32)]
pub enum Cond {
    None = sys::ImGuiCond_None,
    Always = sys::ImGuiCond_Always,
    Once = sys::ImGuiCond_Once,
    FirstUseEver = sys::ImGuiCond_FirstUseEver,
    Appearing = sys::ImGuiCond_Appearing,
}

#[repr(i32)]
pub enum DataType {
    S8 = sys::ImGuiDataType_S8,
    U8 = sys::ImGuiDataType_U8,
    S16 = sys::ImGuiDataType_S16,
    U16 = sys::ImGuiDataType_U16,
    S32 = sys::ImGuiDataType_S32,
    U32 = sys::ImGuiDataType_U32,
    S64 = sys::ImGuiDataType_S64,
    U64 = sys::ImGuiDataType_U64,
    Float = sys::ImGuiDataType_Float,
    Double = sys::ImGuiDataType_Double,
    COUNT = sys::ImGuiDataType_COUNT,
}

#[repr(i32)]
pub enum Key {
    Tab = sys::ImGuiKey_Tab,
    LeftArrow = sys::ImGuiKey_LeftArrow,
    RightArrow = sys::ImGuiKey_RightArrow,
    UpArrow = sys::ImGuiKey_UpArrow,
    DownArrow = sys::ImGuiKey_DownArrow,
    PageUp = sys::ImGuiKey_PageUp,
    PageDown = sys::ImGuiKey_PageDown,
    Home = sys::ImGuiKey_Home,
    End = sys::ImGuiKey_End,
    Insert = sys::ImGuiKey_Insert,
    Delete = sys::ImGuiKey_Delete,
    Backspace = sys::ImGuiKey_Backspace,
    Space = sys::ImGuiKey_Space,
    Enter = sys::ImGuiKey_Enter,
    Escape = sys::ImGuiKey_Escape,
    KeyPadEnter = sys::ImGuiKey_KeyPadEnter,
    A = sys::ImGuiKey_A,
    C = sys::ImGuiKey_C,
    V = sys::ImGuiKey_V,
    X = sys::ImGuiKey_X,
    Y = sys::ImGuiKey_Y,
    Z = sys::ImGuiKey_Z,
    COUNT = sys::ImGuiKey_COUNT,
}

#[repr(i32)]
pub enum NavInput {
    Activate = sys::ImGuiNavInput_Activate,
    Cancel = sys::ImGuiNavInput_Cancel,
    Input = sys::ImGuiNavInput_Input,
    Menu = sys::ImGuiNavInput_Menu,
    DpadLeft = sys::ImGuiNavInput_DpadLeft,
    DpadRight = sys::ImGuiNavInput_DpadRight,
    DpadUp = sys::ImGuiNavInput_DpadUp,
    DpadDown = sys::ImGuiNavInput_DpadDown,
    LStickLeft = sys::ImGuiNavInput_LStickLeft,
    LStickRight = sys::ImGuiNavInput_LStickRight,
    LStickUp = sys::ImGuiNavInput_LStickUp,
    LStickDown = sys::ImGuiNavInput_LStickDown,
    FocusPrev = sys::ImGuiNavInput_FocusPrev,
    FocusNext = sys::ImGuiNavInput_FocusNext,
    TweakSlow = sys::ImGuiNavInput_TweakSlow,
    TweakFast = sys::ImGuiNavInput_TweakFast,
    KeyLeft = sys::ImGuiNavInput_KeyLeft_,
    KeyRight = sys::ImGuiNavInput_KeyRight_,
    KeyUp = sys::ImGuiNavInput_KeyUp_,
    KeyDown = sys::ImGuiNavInput_KeyDown_,
    COUNT = sys::ImGuiNavInput_COUNT,
}

#[repr(i32)]
pub enum Col {
    Text = sys::ImGuiCol_Text,
    TextDisabled = sys::ImGuiCol_TextDisabled,
    WindowBg = sys::ImGuiCol_WindowBg,
    ChildBg = sys::ImGuiCol_ChildBg,
    PopupBg = sys::ImGuiCol_PopupBg,
    Border = sys::ImGuiCol_Border,
    BorderShadow = sys::ImGuiCol_BorderShadow,
    FrameBg = sys::ImGuiCol_FrameBg,
    FrameBgHovered = sys::ImGuiCol_FrameBgHovered,
    FrameBgActive = sys::ImGuiCol_FrameBgActive,
    TitleBg = sys::ImGuiCol_TitleBg,
    TitleBgActive = sys::ImGuiCol_TitleBgActive,
    TitleBgCollapsed = sys::ImGuiCol_TitleBgCollapsed,
    MenuBarBg = sys::ImGuiCol_MenuBarBg,
    ScrollbarBg = sys::ImGuiCol_ScrollbarBg,
    ScrollbarGrab = sys::ImGuiCol_ScrollbarGrab,
    ScrollbarGrabHovered = sys::ImGuiCol_ScrollbarGrabHovered,
    ScrollbarGrabActive = sys::ImGuiCol_ScrollbarGrabActive,
    CheckMark = sys::ImGuiCol_CheckMark,
    SliderGrab = sys::ImGuiCol_SliderGrab,
    SliderGrabActive = sys::ImGuiCol_SliderGrabActive,
    Button = sys::ImGuiCol_Button,
    ButtonHovered = sys::ImGuiCol_ButtonHovered,
    ButtonActive = sys::ImGuiCol_ButtonActive,
    Header = sys::ImGuiCol_Header,
    HeaderHovered = sys::ImGuiCol_HeaderHovered,
    HeaderActive = sys::ImGuiCol_HeaderActive,
    Separator = sys::ImGuiCol_Separator,
    SeparatorHovered = sys::ImGuiCol_SeparatorHovered,
    SeparatorActive = sys::ImGuiCol_SeparatorActive,
    ResizeGrip = sys::ImGuiCol_ResizeGrip,
    ResizeGripHovered = sys::ImGuiCol_ResizeGripHovered,
    ResizeGripActive = sys::ImGuiCol_ResizeGripActive,
    Tab = sys::ImGuiCol_Tab,
    TabHovered = sys::ImGuiCol_TabHovered,
    TabActive = sys::ImGuiCol_TabActive,
    TabUnfocused = sys::ImGuiCol_TabUnfocused,
    TabUnfocusedActive = sys::ImGuiCol_TabUnfocusedActive,
    PlotLines = sys::ImGuiCol_PlotLines,
    PlotLinesHovered = sys::ImGuiCol_PlotLinesHovered,
    PlotHistogram = sys::ImGuiCol_PlotHistogram,
    PlotHistogramHovered = sys::ImGuiCol_PlotHistogramHovered,
    TableHeaderBg = sys::ImGuiCol_TableHeaderBg,
    TableBorderStrong = sys::ImGuiCol_TableBorderStrong,
    TableBorderLight = sys::ImGuiCol_TableBorderLight,
    TableRowBg = sys::ImGuiCol_TableRowBg,
    TableRowBgAlt = sys::ImGuiCol_TableRowBgAlt,
    TextSelectedBg = sys::ImGuiCol_TextSelectedBg,
    DragDropTarget = sys::ImGuiCol_DragDropTarget,
    NavHighlight = sys::ImGuiCol_NavHighlight,
    NavWindowingHighlight = sys::ImGuiCol_NavWindowingHighlight,
    NavWindowingDimBg = sys::ImGuiCol_NavWindowingDimBg,
    ModalWindowDimBg = sys::ImGuiCol_ModalWindowDimBg,
    COUNT = sys::ImGuiCol_COUNT,
}

#[repr(i32)]
pub enum StyleVar {
    Alpha = sys::ImGuiStyleVar_Alpha,
    WindowPadding = sys::ImGuiStyleVar_WindowPadding,
    WindowRounding = sys::ImGuiStyleVar_WindowRounding,
    WindowBorderSize = sys::ImGuiStyleVar_WindowBorderSize,
    WindowMinSize = sys::ImGuiStyleVar_WindowMinSize,
    WindowTitleAlign = sys::ImGuiStyleVar_WindowTitleAlign,
    ChildRounding = sys::ImGuiStyleVar_ChildRounding,
    ChildBorderSize = sys::ImGuiStyleVar_ChildBorderSize,
    PopupRounding = sys::ImGuiStyleVar_PopupRounding,
    PopupBorderSize = sys::ImGuiStyleVar_PopupBorderSize,
    FramePadding = sys::ImGuiStyleVar_FramePadding,
    FrameRounding = sys::ImGuiStyleVar_FrameRounding,
    FrameBorderSize = sys::ImGuiStyleVar_FrameBorderSize,
    ItemSpacing = sys::ImGuiStyleVar_ItemSpacing,
    ItemInnerSpacing = sys::ImGuiStyleVar_ItemInnerSpacing,
    IndentSpacing = sys::ImGuiStyleVar_IndentSpacing,
    CellPadding = sys::ImGuiStyleVar_CellPadding,
    ScrollbarSize = sys::ImGuiStyleVar_ScrollbarSize,
    ScrollbarRounding = sys::ImGuiStyleVar_ScrollbarRounding,
    GrabMinSize = sys::ImGuiStyleVar_GrabMinSize,
    GrabRounding = sys::ImGuiStyleVar_GrabRounding,
    TabRounding = sys::ImGuiStyleVar_TabRounding,
    ButtonTextAlign = sys::ImGuiStyleVar_ButtonTextAlign,
    SelectableTextAlign = sys::ImGuiStyleVar_SelectableTextAlign,
    COUNT = sys::ImGuiStyleVar_COUNT,
}

#[repr(i32)]
pub enum MouseCursor {
    None = sys::ImGuiMouseCursor_None,
    Arrow = sys::ImGuiMouseCursor_Arrow,
    TextInput = sys::ImGuiMouseCursor_TextInput,
    ResizeAll = sys::ImGuiMouseCursor_ResizeAll,
    ResizeNS = sys::ImGuiMouseCursor_ResizeNS,
    ResizeEW = sys::ImGuiMouseCursor_ResizeEW,
    ResizeNESW = sys::ImGuiMouseCursor_ResizeNESW,
    ResizeNWSE = sys::ImGuiMouseCursor_ResizeNWSE,
    Hand = sys::ImGuiMouseCursor_Hand,
    NotAllowed = sys::ImGuiMouseCursor_NotAllowed,
    COUNT = sys::ImGuiMouseCursor_COUNT,
}

fn bool_opt_to_ptr(opt: Option<&mut bool>) -> *mut bool {
    opt.map(|x| x as *mut bool).unwrap_or(std::ptr::null_mut())
}

pub fn begin(
    name: &str,
    open: Option<&mut bool>,
    flags: Option<WindowFlags>,
) -> Result<bool, NulError> {
    let name = CString::new(name)?;
    unsafe {
        Ok(sys::igBegin(
            name.as_ptr(),
            bool_opt_to_ptr(open),
            flags.unwrap_or(WindowFlags::None) as i32,
        ))
    }
}

pub fn begin_child(
    str_id: &str,
    size: Option<Vec2>,
    border: Option<bool>,
    flags: Option<WindowFlags>,
) -> Result<bool, NulError> {
    let str_id = CString::new(str_id)?;
    unsafe {
        Ok(sys::igBeginChild_Str(
            str_id.as_ptr(),
            size.unwrap_or(Vec2::ZERO),
            border.unwrap_or(false),
            flags.unwrap_or(WindowFlags::None) as i32,
        ))
    }
}

pub fn get_window_pos() -> Vec2 {
    let mut ret = MaybeUninit::<Vec2>::uninit();
    unsafe {
        sys::igGetWindowPos(ret.as_mut_ptr());
        ret.assume_init()
    }
}

pub fn get_window_size() -> Vec2 {
    let mut ret = MaybeUninit::<Vec2>::uninit();
    unsafe {
        sys::igGetWindowSize(ret.as_mut_ptr());
        ret.assume_init()
    }
}

pub fn set_next_window_size(size: Vec2, cond: Option<Cond>) {
    unsafe { sys::igSetNextWindowSize(size, cond.unwrap_or(Cond::None) as i32) }
}

pub fn set_next_window_bg_alpha(alpha: f32) {
    unsafe { sys::igSetNextWindowBgAlpha(alpha) }
}

pub fn same_line(offset_from_start_x: Option<f32>, spacing: Option<f32>) {
    unsafe { sys::igSameLine(offset_from_start_x.unwrap_or(0.0), spacing.unwrap_or(-1.0)) }
}

pub fn collapsing_header(
    label: &str,
    visible: Option<&mut bool>,
    flags: Option<TreeNodeFlags>,
) -> Result<bool, NulError> {
    let label = CString::new(label)?;
    Ok(unsafe {
        sys::igCollapsingHeader_BoolPtr(
            label.as_ptr(),
            bool_opt_to_ptr(visible),
            flags.unwrap_or(TreeNodeFlags::None) as i32,
        )
    })
}

pub fn selectable(
    label: &str,
    selected: Option<bool>,
    flags: Option<SelectableFlags>,
    size: Option<Vec2>,
) -> Result<bool, NulError> {
    let label = CString::new(label)?;
    Ok(unsafe {
        sys::igSelectable_Bool(
            label.as_ptr(),
            selected.unwrap_or(false),
            flags.unwrap_or(SelectableFlags::None) as i32,
            size.unwrap_or(Vec2::ZERO),
        )
    })
}

pub fn text(txt: &str) {
    let txt = txt.as_bytes().as_ptr_range();
    unsafe { sys::igTextUnformatted(txt.start as *const i8, txt.end as *const i8) };
}
#[macro_export]
macro_rules! textf {
    ($($arg:tt)*) => {
        $crate::text(&format!($($arg)*))
    }
}
#[macro_export]
macro_rules! bulletf {
    ($($arg:tt)*) => {
        $crate::bullet();
        $crate::textf!($($arg)*)
    }
}

pub fn button(label: &str, size: Option<Vec2>) -> Result<bool, NulError> {
    let label = CString::new(label)?;
    Ok(unsafe { sys::igButton(label.as_ptr(), size.unwrap_or(Vec2::ZERO)) })
}

pub fn small_button(label: &str) -> Result<bool, NulError> {
    let label = CString::new(label)?;
    Ok(unsafe { sys::igSmallButton(label.as_ptr()) })
}

pub fn image(
    user_texture_id: TextureID,
    size: Vec2,
    uv0: Option<Vec2>,
    uv1: Option<Vec2>,
    tint_col: Option<Color>,
    border_col: Option<Color>,
) {
    unsafe {
        sys::igImage(
            user_texture_id,
            size,
            uv0.unwrap_or(Vec2::ZERO),
            uv1.unwrap_or(Vec2::ONE),
            tint_col.unwrap_or(Color::ONE).into(),
            border_col.unwrap_or(Color::ZERO).into(),
        )
    }
}

pub fn image_button(
    user_texture_id: TextureID,
    size: Vec2,
    uv0: Option<Vec2>,
    uv1: Option<Vec2>,
    frame_padding: Option<i32>,
    bg_col: Option<Color>,
    tint_col: Option<Color>,
) -> bool {
    unsafe {
        sys::igImageButton(
            user_texture_id,
            size,
            uv0.unwrap_or(Vec2::ZERO),
            uv1.unwrap_or(Vec2::ONE),
            frame_padding.unwrap_or(-1),
            bg_col.unwrap_or(Color::ZERO).into(),
            tint_col.unwrap_or(Color::ONE).into(),
        )
    }
}

pub fn begin_table(
    str_id: &str,
    column: i32,
    flags: Option<TableFlags>,
    outer_size: Option<Vec2>,
    inner_width: Option<f32>,
) -> Result<bool, NulError> {
    let str_id = CString::new(str_id)?;
    unsafe {
        Ok(sys::igBeginTable(
            str_id.as_ptr(),
            column,
            flags.unwrap_or(TableFlags::None) as sys::ImGuiTableFlags,
            outer_size.unwrap_or(Vec2::ZERO),
            inner_width.unwrap_or(0.0),
        ))
    }
}

pub fn table_next_row(row_flags: Option<TableRowFlags>, min_row_height: Option<f32>) {
    unsafe {
        sys::igTableNextRow(
            row_flags.unwrap_or(TableRowFlags::None) as sys::ImGuiTableRowFlags,
            min_row_height.unwrap_or(0.0),
        )
    }
}

pub fn table_setup_column(
    label: &str,
    flags: Option<TableColumnFlags>,
    init_width_or_weight: Option<f32>,
    user_id: Option<ID>,
) -> Result<(), NulError> {
    let label = CString::new(label)?;
    unsafe {
        sys::igTableSetupColumn(
            label.as_ptr(),
            flags.unwrap_or(TableColumnFlags::None) as sys::ImGuiTableColumnFlags,
            init_width_or_weight.unwrap_or(0.0),
            user_id.unwrap_or(0),
        );
    }
    Ok(())
}

pub fn begin_tab_bar(str_id: &str, flags: Option<TabBarFlags>) -> Result<bool, NulError> {
    let str_id = CString::new(str_id)?;
    unsafe {
        Ok(sys::igBeginTabBar(
            str_id.as_ptr(),
            flags.unwrap_or(TabBarFlags::None) as sys::ImGuiTabBarFlags,
        ))
    }
}

pub fn begin_tab_item(
    str_id: &str,
    open: Option<&mut bool>,
    flags: Option<TabItemFlags>,
) -> Result<bool, NulError> {
    let str_id = CString::new(str_id)?;
    unsafe {
        Ok(sys::igBeginTabItem(
            str_id.as_ptr(),
            bool_opt_to_ptr(open),
            flags.unwrap_or(TabItemFlags::None) as sys::ImGuiTabItemFlags,
        ))
    }
}

pub fn push_style_color(idx: Col, col: Color) {
    unsafe { sys::igPushStyleColor_Vec4(idx as i32, col.into()) }
}

pub fn pop_style_color(count: i32) {
    unsafe { sys::igPopStyleColor(count) }
}

pub fn set_clipboard_text(text: &str) -> Result<(), NulError> {
    let text = CString::new(text)?;
    unsafe { Ok(sys::igSetClipboardText(text.as_ptr())) }
}
