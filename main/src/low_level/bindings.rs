/* automatically generated by rust-bindgen */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub const REAPER_PLUGIN_VERSION: u32 = 526;
    pub const CSURF_EXT_RESET: u32 = 131071;
    pub const CSURF_EXT_SETINPUTMONITOR: u32 = 65537;
    pub const CSURF_EXT_SETMETRONOME: u32 = 65538;
    pub const CSURF_EXT_SETAUTORECARM: u32 = 65539;
    pub const CSURF_EXT_SETRECMODE: u32 = 65540;
    pub const CSURF_EXT_SETSENDVOLUME: u32 = 65541;
    pub const CSURF_EXT_SETSENDPAN: u32 = 65542;
    pub const CSURF_EXT_SETFXENABLED: u32 = 65543;
    pub const CSURF_EXT_SETFXPARAM: u32 = 65544;
    pub const CSURF_EXT_SETLASTTOUCHEDFX: u32 = 65546;
    pub const CSURF_EXT_SETFOCUSEDFX: u32 = 65547;
    pub const CSURF_EXT_SETLASTTOUCHEDTRACK: u32 = 65548;
    pub const CSURF_EXT_SETMIXERSCROLL: u32 = 65549;
    pub const CSURF_EXT_SETBPMANDPLAYRATE: u32 = 65545;
    pub const CSURF_EXT_SETPAN_EX: u32 = 65550;
    pub const CSURF_EXT_SETRECVVOLUME: u32 = 65552;
    pub const CSURF_EXT_SETRECVPAN: u32 = 65553;
    pub const CSURF_EXT_SETFXOPEN: u32 = 65554;
    pub const CSURF_EXT_SETFXCHANGE: u32 = 65555;
    pub const CSURF_EXT_SETPROJECTMARKERCHANGE: u32 = 65556;
    pub const CSURF_EXT_SETFXPARAM_RECFX: u32 = 65560;
    pub const CSURF_EXT_SUPPORTS_EXTENDED_TOUCH: u32 = 524289;
    pub mod std {
        #[allow(unused_imports)]
        use self::super::super::root;
    }
    pub mod __gnu_cxx {
        #[allow(unused_imports)]
        use self::super::super::root;
    }
    pub type DWORD = ::std::os::raw::c_uint;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct HWND__ {
        _unused: [u8; 0],
    }
    pub type HWND = *mut root::HWND__;
    pub type HINSTANCE = *mut ::std::os::raw::c_void;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct ACCEL {
        pub fVirt: ::std::os::raw::c_uchar,
        pub key: ::std::os::raw::c_ushort,
        pub cmd: ::std::os::raw::c_ushort,
    }
    #[test]
    fn bindgen_test_layout_ACCEL() {
        assert_eq!(
            ::std::mem::size_of::<ACCEL>(),
            6usize,
            concat!("Size of: ", stringify!(ACCEL))
        );
        assert_eq!(
            ::std::mem::align_of::<ACCEL>(),
            2usize,
            concat!("Alignment of ", stringify!(ACCEL))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<ACCEL>())).fVirt as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ACCEL),
                "::",
                stringify!(fVirt)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<ACCEL>())).key as *const _ as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(ACCEL),
                "::",
                stringify!(key)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<ACCEL>())).cmd as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(ACCEL),
                "::",
                stringify!(cmd)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct _GUID {
        pub Data1: ::std::os::raw::c_uint,
        pub Data2: ::std::os::raw::c_ushort,
        pub Data3: ::std::os::raw::c_ushort,
        pub Data4: [::std::os::raw::c_uchar; 8usize],
    }
    #[test]
    fn bindgen_test_layout__GUID() {
        assert_eq!(
            ::std::mem::size_of::<_GUID>(),
            16usize,
            concat!("Size of: ", stringify!(_GUID))
        );
        assert_eq!(
            ::std::mem::align_of::<_GUID>(),
            4usize,
            concat!("Alignment of ", stringify!(_GUID))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<_GUID>())).Data1 as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_GUID),
                "::",
                stringify!(Data1)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<_GUID>())).Data2 as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(_GUID),
                "::",
                stringify!(Data2)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<_GUID>())).Data3 as *const _ as usize },
            6usize,
            concat!(
                "Offset of field: ",
                stringify!(_GUID),
                "::",
                stringify!(Data3)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<_GUID>())).Data4 as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_GUID),
                "::",
                stringify!(Data4)
            )
        );
    }
    pub type GUID = root::_GUID;
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct reaper_plugin_info_t {
        pub caller_version: ::std::os::raw::c_int,
        pub hwnd_main: root::HWND,
        pub Register: ::std::option::Option<
            unsafe extern "C" fn(
                name: *const ::std::os::raw::c_char,
                infostruct: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
        pub GetFunc: ::std::option::Option<
            unsafe extern "C" fn(
                name: *const ::std::os::raw::c_char,
            ) -> *mut ::std::os::raw::c_void,
        >,
    }
    #[test]
    fn bindgen_test_layout_reaper_plugin_info_t() {
        assert_eq!(
            ::std::mem::size_of::<reaper_plugin_info_t>(),
            32usize,
            concat!("Size of: ", stringify!(reaper_plugin_info_t))
        );
        assert_eq!(
            ::std::mem::align_of::<reaper_plugin_info_t>(),
            8usize,
            concat!("Alignment of ", stringify!(reaper_plugin_info_t))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<reaper_plugin_info_t>())).caller_version as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(reaper_plugin_info_t),
                "::",
                stringify!(caller_version)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<reaper_plugin_info_t>())).hwnd_main as *const _ as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(reaper_plugin_info_t),
                "::",
                stringify!(hwnd_main)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<reaper_plugin_info_t>())).Register as *const _ as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(reaper_plugin_info_t),
                "::",
                stringify!(Register)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<reaper_plugin_info_t>())).GetFunc as *const _ as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(reaper_plugin_info_t),
                "::",
                stringify!(GetFunc)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct gaccel_register_t {
        pub accel: root::ACCEL,
        pub desc: *const ::std::os::raw::c_char,
    }
    #[test]
    fn bindgen_test_layout_gaccel_register_t() {
        assert_eq!(
            ::std::mem::size_of::<gaccel_register_t>(),
            16usize,
            concat!("Size of: ", stringify!(gaccel_register_t))
        );
        assert_eq!(
            ::std::mem::align_of::<gaccel_register_t>(),
            8usize,
            concat!("Alignment of ", stringify!(gaccel_register_t))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<gaccel_register_t>())).accel as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(gaccel_register_t),
                "::",
                stringify!(accel)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<gaccel_register_t>())).desc as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(gaccel_register_t),
                "::",
                stringify!(desc)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct KbdCmd {
        pub cmd: root::DWORD,
        pub text: *const ::std::os::raw::c_char,
    }
    #[test]
    fn bindgen_test_layout_KbdCmd() {
        assert_eq!(
            ::std::mem::size_of::<KbdCmd>(),
            16usize,
            concat!("Size of: ", stringify!(KbdCmd))
        );
        assert_eq!(
            ::std::mem::align_of::<KbdCmd>(),
            8usize,
            concat!("Alignment of ", stringify!(KbdCmd))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdCmd>())).cmd as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdCmd),
                "::",
                stringify!(cmd)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdCmd>())).text as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdCmd),
                "::",
                stringify!(text)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct KbdKeyBindingInfo {
        pub key: ::std::os::raw::c_int,
        pub cmd: ::std::os::raw::c_int,
        pub flags: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_KbdKeyBindingInfo() {
        assert_eq!(
            ::std::mem::size_of::<KbdKeyBindingInfo>(),
            12usize,
            concat!("Size of: ", stringify!(KbdKeyBindingInfo))
        );
        assert_eq!(
            ::std::mem::align_of::<KbdKeyBindingInfo>(),
            4usize,
            concat!("Alignment of ", stringify!(KbdKeyBindingInfo))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdKeyBindingInfo>())).key as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdKeyBindingInfo),
                "::",
                stringify!(key)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdKeyBindingInfo>())).cmd as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdKeyBindingInfo),
                "::",
                stringify!(cmd)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdKeyBindingInfo>())).flags as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdKeyBindingInfo),
                "::",
                stringify!(flags)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct KbdSectionInfo {
        pub uniqueID: ::std::os::raw::c_int,
        pub name: *const ::std::os::raw::c_char,
        pub action_list: *mut root::KbdCmd,
        pub action_list_cnt: ::std::os::raw::c_int,
        pub def_keys: *mut root::KbdKeyBindingInfo,
        pub def_keys_cnt: ::std::os::raw::c_int,
        pub onAction: ::std::option::Option<
            unsafe extern "C" fn(
                cmd: ::std::os::raw::c_int,
                val: ::std::os::raw::c_int,
                valhw: ::std::os::raw::c_int,
                relmode: ::std::os::raw::c_int,
                hwnd: root::HWND,
            ) -> bool,
        >,
        pub accels: *mut ::std::os::raw::c_void,
        pub recent_cmds: *mut ::std::os::raw::c_void,
        pub extended_data: [*mut ::std::os::raw::c_void; 32usize],
    }
    #[test]
    fn bindgen_test_layout_KbdSectionInfo() {
        assert_eq!(
            ::std::mem::size_of::<KbdSectionInfo>(),
            328usize,
            concat!("Size of: ", stringify!(KbdSectionInfo))
        );
        assert_eq!(
            ::std::mem::align_of::<KbdSectionInfo>(),
            8usize,
            concat!("Alignment of ", stringify!(KbdSectionInfo))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdSectionInfo>())).uniqueID as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(uniqueID)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdSectionInfo>())).name as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(name)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdSectionInfo>())).action_list as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(action_list)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<KbdSectionInfo>())).action_list_cnt as *const _ as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(action_list_cnt)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdSectionInfo>())).def_keys as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(def_keys)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdSectionInfo>())).def_keys_cnt as *const _ as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(def_keys_cnt)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdSectionInfo>())).onAction as *const _ as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(onAction)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdSectionInfo>())).accels as *const _ as usize },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(accels)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<KbdSectionInfo>())).recent_cmds as *const _ as usize },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(recent_cmds)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<KbdSectionInfo>())).extended_data as *const _ as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(KbdSectionInfo),
                "::",
                stringify!(extended_data)
            )
        );
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct ReaProject {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct MediaTrack {
        _unused: [u8; 0],
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct TrackEnvelope {
        _unused: [u8; 0],
    }
    #[repr(C)]
    pub struct IReaperControlSurface__bindgen_vtable(::std::os::raw::c_void);
    #[repr(C)]
    #[derive(Debug, Hash, PartialEq, Eq)]
    pub struct IReaperControlSurface {
        pub vtable_: *const IReaperControlSurface__bindgen_vtable,
    }
    #[test]
    fn bindgen_test_layout_IReaperControlSurface() {
        assert_eq!(
            ::std::mem::size_of::<IReaperControlSurface>(),
            8usize,
            concat!("Size of: ", stringify!(IReaperControlSurface))
        );
        assert_eq!(
            ::std::mem::align_of::<IReaperControlSurface>(),
            8usize,
            concat!("Alignment of ", stringify!(IReaperControlSurface))
        );
    }
    extern "C" {
        pub static mut ClearConsole: ::std::option::Option<unsafe extern "C" fn()>;
    }
    extern "C" {
        pub static mut CountTracks: ::std::option::Option<
            unsafe extern "C" fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int,
        >;
    }
    extern "C" {
        pub static mut CSurf_OnInputMonitorChangeEx: ::std::option::Option<
            unsafe extern "C" fn(
                trackid: *mut root::MediaTrack,
                monitor: ::std::os::raw::c_int,
                allowgang: bool,
            ) -> ::std::os::raw::c_int,
        >;
    }
    extern "C" {
        pub static mut CSurf_OnPanChangeEx: ::std::option::Option<
            unsafe extern "C" fn(
                trackid: *mut root::MediaTrack,
                pan: f64,
                relative: bool,
                allowGang: bool,
            ) -> f64,
        >;
    }
    extern "C" {
        pub static mut CSurf_OnVolumeChangeEx: ::std::option::Option<
            unsafe extern "C" fn(
                trackid: *mut root::MediaTrack,
                volume: f64,
                relative: bool,
                allowGang: bool,
            ) -> f64,
        >;
    }
    extern "C" {
        pub static mut CSurf_SetSurfacePan: ::std::option::Option<
            unsafe extern "C" fn(
                trackid: *mut root::MediaTrack,
                pan: f64,
                ignoresurf: *mut root::IReaperControlSurface,
            ),
        >;
    }
    extern "C" {
        pub static mut CSurf_SetSurfaceVolume: ::std::option::Option<
            unsafe extern "C" fn(
                trackid: *mut root::MediaTrack,
                volume: f64,
                ignoresurf: *mut root::IReaperControlSurface,
            ),
        >;
    }
    extern "C" {
        pub static mut DB2SLIDER: ::std::option::Option<unsafe extern "C" fn(x: f64) -> f64>;
    }
    extern "C" {
        pub static mut EnumProjects: ::std::option::Option<
            unsafe extern "C" fn(
                idx: ::std::os::raw::c_int,
                projfnOutOptional: *mut ::std::os::raw::c_char,
                projfnOutOptional_sz: ::std::os::raw::c_int,
            ) -> *mut root::ReaProject,
        >;
    }
    extern "C" {
        pub static mut GetAppVersion:
            ::std::option::Option<unsafe extern "C" fn() -> *const ::std::os::raw::c_char>;
    }
    extern "C" {
        pub static mut GetGlobalAutomationOverride:
            ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;
    }
    extern "C" {
        pub static mut GetMainHwnd: ::std::option::Option<unsafe extern "C" fn() -> root::HWND>;
    }
    extern "C" {
        pub static mut GetMasterTrack: ::std::option::Option<
            unsafe extern "C" fn(proj: *mut root::ReaProject) -> *mut root::MediaTrack,
        >;
    }
    extern "C" {
        pub static mut GetMediaTrackInfo_Value: ::std::option::Option<
            unsafe extern "C" fn(
                tr: *mut root::MediaTrack,
                parmname: *const ::std::os::raw::c_char,
            ) -> f64,
        >;
    }
    extern "C" {
        pub static mut GetSetMediaTrackInfo: ::std::option::Option<
            unsafe extern "C" fn(
                tr: *mut root::MediaTrack,
                parmname: *const ::std::os::raw::c_char,
                setNewValue: *mut ::std::os::raw::c_void,
            ) -> *mut ::std::os::raw::c_void,
        >;
    }
    extern "C" {
        pub static mut GetTrack: ::std::option::Option<
            unsafe extern "C" fn(
                proj: *mut root::ReaProject,
                trackidx: ::std::os::raw::c_int,
            ) -> *mut root::MediaTrack,
        >;
    }
    extern "C" {
        pub static mut GetTrackAutomationMode: ::std::option::Option<
            unsafe extern "C" fn(tr: *mut root::MediaTrack) -> ::std::os::raw::c_int,
        >;
    }
    extern "C" {
        pub static mut GetTrackEnvelopeByName: ::std::option::Option<
            unsafe extern "C" fn(
                track: *mut root::MediaTrack,
                envname: *const ::std::os::raw::c_char,
            ) -> *mut root::TrackEnvelope,
        >;
    }
    extern "C" {
        pub static mut GetTrackUIVolPan: ::std::option::Option<
            unsafe extern "C" fn(
                track: *mut root::MediaTrack,
                volumeOut: *mut f64,
                panOut: *mut f64,
            ) -> bool,
        >;
    }
    extern "C" {
        pub static mut guidToString: ::std::option::Option<
            unsafe extern "C" fn(g: *const root::GUID, destNeed64: *mut ::std::os::raw::c_char),
        >;
    }
    extern "C" {
        pub static mut InsertTrackAtIndex: ::std::option::Option<
            unsafe extern "C" fn(idx: ::std::os::raw::c_int, wantDefaults: bool),
        >;
    }
    extern "C" {
        pub static mut KBD_OnMainActionEx: ::std::option::Option<
            unsafe extern "C" fn(
                cmd: ::std::os::raw::c_int,
                val: ::std::os::raw::c_int,
                valhw: ::std::os::raw::c_int,
                relmode: ::std::os::raw::c_int,
                hwnd: root::HWND,
                proj: *mut root::ReaProject,
            ) -> ::std::os::raw::c_int,
        >;
    }
    extern "C" {
        pub static mut NamedCommandLookup: ::std::option::Option<
            unsafe extern "C" fn(
                command_name: *const ::std::os::raw::c_char,
            ) -> ::std::os::raw::c_int,
        >;
    }
    extern "C" {
        pub static mut plugin_register: ::std::option::Option<
            unsafe extern "C" fn(
                name: *const ::std::os::raw::c_char,
                infostruct: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >;
    }
    extern "C" {
        pub static mut SectionFromUniqueID: ::std::option::Option<
            unsafe extern "C" fn(uniqueID: ::std::os::raw::c_int) -> *mut root::KbdSectionInfo,
        >;
    }
    extern "C" {
        pub static mut SetMediaTrackInfo_Value: ::std::option::Option<
            unsafe extern "C" fn(
                tr: *mut root::MediaTrack,
                parmname: *const ::std::os::raw::c_char,
                newvalue: f64,
            ) -> bool,
        >;
    }
    extern "C" {
        pub static mut ShowConsoleMsg:
            ::std::option::Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>;
    }
    extern "C" {
        pub static mut SLIDER2DB: ::std::option::Option<unsafe extern "C" fn(y: f64) -> f64>;
    }
    extern "C" {
        pub static mut stringToGuid: ::std::option::Option<
            unsafe extern "C" fn(str: *const ::std::os::raw::c_char, g: *mut root::GUID),
        >;
    }
    extern "C" {
        pub static mut TrackFX_GetCount: ::std::option::Option<
            unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int,
        >;
    }
    extern "C" {
        pub static mut TrackFX_GetFXGUID: ::std::option::Option<
            unsafe extern "C" fn(
                track: *mut root::MediaTrack,
                fx: ::std::os::raw::c_int,
            ) -> *mut root::GUID,
        >;
    }
    extern "C" {
        pub static mut TrackFX_GetParamNormalized: ::std::option::Option<
            unsafe extern "C" fn(
                track: *mut root::MediaTrack,
                fx: ::std::os::raw::c_int,
                param: ::std::os::raw::c_int,
            ) -> f64,
        >;
    }
    extern "C" {
        pub static mut TrackFX_GetRecCount: ::std::option::Option<
            unsafe extern "C" fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int,
        >;
    }
    extern "C" {
        pub static mut TrackList_UpdateAllExternalSurfaces:
            ::std::option::Option<unsafe extern "C" fn()>;
    }
    extern "C" {
        pub static mut ValidatePtr2: ::std::option::Option<
            unsafe extern "C" fn(
                proj: *mut root::ReaProject,
                pointer: *mut ::std::os::raw::c_void,
                ctypename: *const ::std::os::raw::c_char,
            ) -> bool,
        >;
    }
    pub mod reaper_rs_control_surface {
        #[allow(unused_imports)]
        use self::super::super::root;
        extern "C" {
            pub fn get_control_surface() -> *mut ::std::os::raw::c_void;
        }
        extern "C" {
            pub fn GetTypeString(
                callback_target: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        extern "C" {
            pub fn GetDescString(
                callback_target: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        extern "C" {
            pub fn GetConfigString(
                callback_target: *mut ::std::os::raw::c_void,
            ) -> *const ::std::os::raw::c_char;
        }
        extern "C" {
            pub fn CloseNoReset(callback_target: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            pub fn Run(callback_target: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            pub fn SetTrackListChange(callback_target: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            pub fn SetSurfaceVolume(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
                volume: f64,
            );
        }
        extern "C" {
            pub fn SetSurfacePan(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
                pan: f64,
            );
        }
        extern "C" {
            pub fn SetSurfaceMute(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
                mute: bool,
            );
        }
        extern "C" {
            pub fn SetSurfaceSelected(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
                selected: bool,
            );
        }
        extern "C" {
            pub fn SetSurfaceSolo(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
                solo: bool,
            );
        }
        extern "C" {
            pub fn SetSurfaceRecArm(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
                recarm: bool,
            );
        }
        extern "C" {
            pub fn SetPlayState(
                callback_target: *mut ::std::os::raw::c_void,
                play: bool,
                pause: bool,
                rec: bool,
            );
        }
        extern "C" {
            pub fn SetRepeatState(callback_target: *mut ::std::os::raw::c_void, rep: bool);
        }
        extern "C" {
            pub fn SetTrackTitle(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
                title: *const ::std::os::raw::c_char,
            );
        }
        extern "C" {
            pub fn GetTouchState(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
                isPan: ::std::os::raw::c_int,
            ) -> bool;
        }
        extern "C" {
            pub fn SetAutoMode(
                callback_target: *mut ::std::os::raw::c_void,
                mode: ::std::os::raw::c_int,
            );
        }
        extern "C" {
            pub fn ResetCachedVolPanStates(callback_target: *mut ::std::os::raw::c_void);
        }
        extern "C" {
            pub fn OnTrackSelection(
                callback_target: *mut ::std::os::raw::c_void,
                trackid: *mut root::MediaTrack,
            );
        }
        extern "C" {
            pub fn IsKeyDown(
                callback_target: *mut ::std::os::raw::c_void,
                key: ::std::os::raw::c_int,
            ) -> bool;
        }
        extern "C" {
            pub fn Extended(
                callback_target: *mut ::std::os::raw::c_void,
                call: ::std::os::raw::c_int,
                parm1: *mut ::std::os::raw::c_void,
                parm2: *mut ::std::os::raw::c_void,
                parm3: *mut ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int;
        }
    }
}
