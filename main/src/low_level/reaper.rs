use super::bindings::root;
use c_str_macro::c_str;

pub type FunctionProvider = Box<dyn Fn(&std::ffi::CStr) -> isize>;

#[derive(Default)]
pub struct Reaper {
    pub Audio_RegHardwareHook:
        Option<fn(isAdd: bool, reg: *mut root::audio_hook_register_t) -> ::std::os::raw::c_int>,
    pub ClearConsole: Option<fn()>,
    pub CountSelectedTracks2:
        Option<fn(proj: *mut root::ReaProject, wantmaster: bool) -> ::std::os::raw::c_int>,
    pub CountTracks: Option<fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub CreateTrackSend: Option<
        fn(
            tr: *mut root::MediaTrack,
            desttrInOptional: *mut root::MediaTrack,
        ) -> ::std::os::raw::c_int,
    >,
    pub CSurf_OnInputMonitorChangeEx: Option<
        fn(
            trackid: *mut root::MediaTrack,
            monitor: ::std::os::raw::c_int,
            allowgang: bool,
        ) -> ::std::os::raw::c_int,
    >,
    pub CSurf_OnPanChangeEx: Option<
        fn(trackid: *mut root::MediaTrack, pan: f64, relative: bool, allowGang: bool) -> f64,
    >,
    pub CSurf_OnPlayRateChange: Option<fn(playrate: f64)>,
    pub CSurf_OnRecArmChangeEx: Option<
        fn(trackid: *mut root::MediaTrack, recarm: ::std::os::raw::c_int, allowgang: bool) -> bool,
    >,
    pub CSurf_OnSendPanChange: Option<
        fn(
            trackid: *mut root::MediaTrack,
            send_index: ::std::os::raw::c_int,
            pan: f64,
            relative: bool,
        ) -> f64,
    >,
    pub CSurf_OnSendVolumeChange: Option<
        fn(
            trackid: *mut root::MediaTrack,
            send_index: ::std::os::raw::c_int,
            volume: f64,
            relative: bool,
        ) -> f64,
    >,
    pub CSurf_OnVolumeChangeEx: Option<
        fn(trackid: *mut root::MediaTrack, volume: f64, relative: bool, allowGang: bool) -> f64,
    >,
    pub CSurf_SetSurfaceMute: Option<
        fn(
            trackid: *mut root::MediaTrack,
            mute: bool,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetSurfacePan: Option<
        fn(trackid: *mut root::MediaTrack, pan: f64, ignoresurf: *mut root::IReaperControlSurface),
    >,
    pub CSurf_SetSurfaceSolo: Option<
        fn(
            trackid: *mut root::MediaTrack,
            solo: bool,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub CSurf_SetSurfaceVolume: Option<
        fn(
            trackid: *mut root::MediaTrack,
            volume: f64,
            ignoresurf: *mut root::IReaperControlSurface,
        ),
    >,
    pub DB2SLIDER: Option<fn(x: f64) -> f64>,
    pub DeleteTrack: Option<fn(tr: *mut root::MediaTrack)>,
    pub EnumProjects: Option<
        fn(
            idx: ::std::os::raw::c_int,
            projfnOutOptional: *mut ::std::os::raw::c_char,
            projfnOutOptional_sz: ::std::os::raw::c_int,
        ) -> *mut root::ReaProject,
    >,
    pub genGuid: Option<fn(g: *mut root::GUID)>,
    pub GetAppVersion: Option<fn() -> *const ::std::os::raw::c_char>,
    pub GetCurrentProjectInLoadSave: Option<fn() -> *mut root::ReaProject>,
    pub GetFocusedFX: Option<
        fn(
            tracknumberOut: *mut ::std::os::raw::c_int,
            itemnumberOut: *mut ::std::os::raw::c_int,
            fxnumberOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetGlobalAutomationOverride: Option<fn() -> ::std::os::raw::c_int>,
    pub GetLastTouchedFX: Option<
        fn(
            tracknumberOut: *mut ::std::os::raw::c_int,
            fxnumberOut: *mut ::std::os::raw::c_int,
            paramnumberOut: *mut ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetMainHwnd: Option<fn() -> root::HWND>,
    pub GetMasterTrack: Option<fn(proj: *mut root::ReaProject) -> *mut root::MediaTrack>,
    pub GetMaxMidiInputs: Option<fn() -> ::std::os::raw::c_int>,
    pub GetMaxMidiOutputs: Option<fn() -> ::std::os::raw::c_int>,
    pub GetMediaTrackInfo_Value:
        Option<fn(tr: *mut root::MediaTrack, parmname: *const ::std::os::raw::c_char) -> f64>,
    pub GetMIDIInputName: Option<
        fn(
            dev: ::std::os::raw::c_int,
            nameout: *mut ::std::os::raw::c_char,
            nameout_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetMIDIOutputName: Option<
        fn(
            dev: ::std::os::raw::c_int,
            nameout: *mut ::std::os::raw::c_char,
            nameout_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub GetSelectedTrack2: Option<
        fn(
            proj: *mut root::ReaProject,
            seltrackidx: ::std::os::raw::c_int,
            wantmaster: bool,
        ) -> *mut root::MediaTrack,
    >,
    pub GetSetMediaTrackInfo: Option<
        fn(
            tr: *mut root::MediaTrack,
            parmname: *const ::std::os::raw::c_char,
            setNewValue: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetSetTrackSendInfo: Option<
        fn(
            tr: *mut root::MediaTrack,
            category: ::std::os::raw::c_int,
            sendidx: ::std::os::raw::c_int,
            parmname: *const ::std::os::raw::c_char,
            setNewValue: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetToggleCommandState2: Option<
        fn(
            section: *mut root::KbdSectionInfo,
            command_id: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub GetTrack: Option<
        fn(proj: *mut root::ReaProject, trackidx: ::std::os::raw::c_int) -> *mut root::MediaTrack,
    >,
    pub GetTrackAutomationMode: Option<fn(tr: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub GetTrackEnvelopeByName: Option<
        fn(
            track: *mut root::MediaTrack,
            envname: *const ::std::os::raw::c_char,
        ) -> *mut root::TrackEnvelope,
    >,
    pub GetTrackNumSends: Option<
        fn(tr: *mut root::MediaTrack, category: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub GetTrackSendUIVolPan: Option<
        fn(
            track: *mut root::MediaTrack,
            send_index: ::std::os::raw::c_int,
            volumeOut: *mut f64,
            panOut: *mut f64,
        ) -> bool,
    >,
    pub GetTrackStateChunk: Option<
        fn(
            track: *mut root::MediaTrack,
            strNeedBig: *mut ::std::os::raw::c_char,
            strNeedBig_sz: ::std::os::raw::c_int,
            isundoOptional: bool,
        ) -> bool,
    >,
    pub GetTrackUIVolPan:
        Option<fn(track: *mut root::MediaTrack, volumeOut: *mut f64, panOut: *mut f64) -> bool>,
    pub guidToString: Option<fn(g: *const root::GUID, destNeed64: *mut ::std::os::raw::c_char)>,
    pub InsertTrackAtIndex: Option<fn(idx: ::std::os::raw::c_int, wantDefaults: bool)>,
    pub IsProjectDirty: Option<fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub kbd_getTextFromCmd: Option<
        fn(cmd: root::DWORD, section: *mut root::KbdSectionInfo) -> *const ::std::os::raw::c_char,
    >,
    pub KBD_OnMainActionEx: Option<
        fn(
            cmd: ::std::os::raw::c_int,
            val: ::std::os::raw::c_int,
            valhw: ::std::os::raw::c_int,
            relmode: ::std::os::raw::c_int,
            hwnd: root::HWND,
            proj: *mut root::ReaProject,
        ) -> ::std::os::raw::c_int,
    >,
    pub Main_OnCommandEx: Option<
        fn(
            command: ::std::os::raw::c_int,
            flag: ::std::os::raw::c_int,
            proj: *mut root::ReaProject,
        ),
    >,
    pub MarkProjectDirty: Option<fn(proj: *mut root::ReaProject)>,
    pub Master_GetPlayRate: Option<fn(project: *mut root::ReaProject) -> f64>,
    pub Master_GetTempo: Option<fn() -> f64>,
    pub NamedCommandLookup:
        Option<fn(command_name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int>,
    pub plugin_register: Option<
        fn(
            name: *const ::std::os::raw::c_char,
            infostruct: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub ReverseNamedCommandLookup:
        Option<fn(command_id: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char>,
    pub SectionFromUniqueID:
        Option<fn(uniqueID: ::std::os::raw::c_int) -> *mut root::KbdSectionInfo>,
    pub SetCurrentBPM: Option<fn(__proj: *mut root::ReaProject, bpm: f64, wantUndo: bool)>,
    pub SetMediaTrackInfo_Value: Option<
        fn(
            tr: *mut root::MediaTrack,
            parmname: *const ::std::os::raw::c_char,
            newvalue: f64,
        ) -> bool,
    >,
    pub SetOnlyTrackSelected: Option<fn(track: *mut root::MediaTrack)>,
    pub SetTrackSelected: Option<fn(track: *mut root::MediaTrack, selected: bool)>,
    pub SetTrackStateChunk: Option<
        fn(
            track: *mut root::MediaTrack,
            str: *const ::std::os::raw::c_char,
            isundoOptional: bool,
        ) -> bool,
    >,
    pub ShowConsoleMsg: Option<fn(msg: *const ::std::os::raw::c_char)>,
    pub ShowMessageBox: Option<
        fn(
            msg: *const ::std::os::raw::c_char,
            title: *const ::std::os::raw::c_char,
            type_: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub SLIDER2DB: Option<fn(y: f64) -> f64>,
    pub stringToGuid: Option<fn(str: *const ::std::os::raw::c_char, g: *mut root::GUID)>,
    pub StuffMIDIMessage: Option<
        fn(
            mode: ::std::os::raw::c_int,
            msg1: ::std::os::raw::c_int,
            msg2: ::std::os::raw::c_int,
            msg3: ::std::os::raw::c_int,
        ),
    >,
    pub TrackFX_AddByName: Option<
        fn(
            track: *mut root::MediaTrack,
            fxname: *const ::std::os::raw::c_char,
            recFX: bool,
            instantiate: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_CopyToTrack: Option<
        fn(
            src_track: *mut root::MediaTrack,
            src_fx: ::std::os::raw::c_int,
            dest_track: *mut root::MediaTrack,
            dest_fx: ::std::os::raw::c_int,
            is_move: bool,
        ),
    >,
    pub TrackFX_Delete: Option<fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> bool>,
    pub TrackFX_FormatParamValueNormalized: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            value: f64,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetCount: Option<fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub TrackFX_GetEnabled:
        Option<fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> bool>,
    pub TrackFX_GetFloatingWindow:
        Option<fn(track: *mut root::MediaTrack, index: ::std::os::raw::c_int) -> root::HWND>,
    pub TrackFX_GetFormattedParamValue: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetFXGUID:
        Option<fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> *mut root::GUID>,
    pub TrackFX_GetFXName: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetInstrument: Option<fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub TrackFX_GetNumParams: Option<
        fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_GetOpen:
        Option<fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int) -> bool>,
    pub TrackFX_GetParameterStepSizes: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            stepOut: *mut f64,
            smallstepOut: *mut f64,
            largestepOut: *mut f64,
            istoggleOut: *mut bool,
        ) -> bool,
    >,
    pub TrackFX_GetParamEx: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            minvalOut: *mut f64,
            maxvalOut: *mut f64,
            midvalOut: *mut f64,
        ) -> f64,
    >,
    pub TrackFX_GetParamName: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            buf: *mut ::std::os::raw::c_char,
            buf_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetParamNormalized: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
        ) -> f64,
    >,
    pub TrackFX_GetPreset: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            presetname: *mut ::std::os::raw::c_char,
            presetname_sz: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_GetPresetIndex: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            numberOfPresetsOut: *mut ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub TrackFX_GetRecCount: Option<fn(track: *mut root::MediaTrack) -> ::std::os::raw::c_int>,
    pub TrackFX_NavigatePresets: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            presetmove: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_SetEnabled:
        Option<fn(track: *mut root::MediaTrack, fx: ::std::os::raw::c_int, enabled: bool)>,
    pub TrackFX_SetParamNormalized: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            param: ::std::os::raw::c_int,
            value: f64,
        ) -> bool,
    >,
    pub TrackFX_SetPresetByIndex: Option<
        fn(
            track: *mut root::MediaTrack,
            fx: ::std::os::raw::c_int,
            idx: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub TrackFX_Show: Option<
        fn(
            track: *mut root::MediaTrack,
            index: ::std::os::raw::c_int,
            showFlag: ::std::os::raw::c_int,
        ),
    >,
    pub TrackList_UpdateAllExternalSurfaces: Option<fn()>,
    pub Undo_BeginBlock2: Option<fn(proj: *mut root::ReaProject)>,
    pub Undo_CanRedo2: Option<fn(proj: *mut root::ReaProject) -> *const ::std::os::raw::c_char>,
    pub Undo_CanUndo2: Option<fn(proj: *mut root::ReaProject) -> *const ::std::os::raw::c_char>,
    pub Undo_DoRedo2: Option<fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub Undo_DoUndo2: Option<fn(proj: *mut root::ReaProject) -> ::std::os::raw::c_int>,
    pub Undo_EndBlock2: Option<
        fn(
            proj: *mut root::ReaProject,
            descchange: *const ::std::os::raw::c_char,
            extraflags: ::std::os::raw::c_int,
        ),
    >,
    pub ValidatePtr2: Option<
        fn(
            proj: *mut root::ReaProject,
            pointer: *mut ::std::os::raw::c_void,
            ctypename: *const ::std::os::raw::c_char,
        ) -> bool,
    >,
    pub GetMidiInput: Option<fn(idx: ::std::os::raw::c_int) -> *mut root::midi_Input>,
    pub GetMidiOutput: Option<fn(idx: ::std::os::raw::c_int) -> *mut root::midi_Output>,
}

impl Reaper {
    #[doc = r" Loads all available REAPER functions from the given function provider and"]
    #[doc = r" returns a Reaper instance which allows you to call these functions."]
    pub fn load(get_func: FunctionProvider) -> Reaper {
        unsafe {
            Reaper {
                Audio_RegHardwareHook: std::mem::transmute(get_func(c_str!(stringify!(
                    Audio_RegHardwareHook
                )))),
                ClearConsole: std::mem::transmute(get_func(c_str!(stringify!(ClearConsole)))),
                CountSelectedTracks2: std::mem::transmute(get_func(c_str!(stringify!(
                    CountSelectedTracks2
                )))),
                CountTracks: std::mem::transmute(get_func(c_str!(stringify!(CountTracks)))),
                CreateTrackSend: std::mem::transmute(get_func(c_str!(stringify!(CreateTrackSend)))),
                CSurf_OnInputMonitorChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnInputMonitorChangeEx
                )))),
                CSurf_OnPanChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnPanChangeEx
                )))),
                CSurf_OnPlayRateChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnPlayRateChange
                )))),
                CSurf_OnRecArmChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnRecArmChangeEx
                )))),
                CSurf_OnSendPanChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnSendPanChange
                )))),
                CSurf_OnSendVolumeChange: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnSendVolumeChange
                )))),
                CSurf_OnVolumeChangeEx: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_OnVolumeChangeEx
                )))),
                CSurf_SetSurfaceMute: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfaceMute
                )))),
                CSurf_SetSurfacePan: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfacePan
                )))),
                CSurf_SetSurfaceSolo: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfaceSolo
                )))),
                CSurf_SetSurfaceVolume: std::mem::transmute(get_func(c_str!(stringify!(
                    CSurf_SetSurfaceVolume
                )))),
                DB2SLIDER: std::mem::transmute(get_func(c_str!(stringify!(DB2SLIDER)))),
                DeleteTrack: std::mem::transmute(get_func(c_str!(stringify!(DeleteTrack)))),
                EnumProjects: std::mem::transmute(get_func(c_str!(stringify!(EnumProjects)))),
                genGuid: std::mem::transmute(get_func(c_str!(stringify!(genGuid)))),
                GetAppVersion: std::mem::transmute(get_func(c_str!(stringify!(GetAppVersion)))),
                GetCurrentProjectInLoadSave: std::mem::transmute(get_func(c_str!(stringify!(
                    GetCurrentProjectInLoadSave
                )))),
                GetFocusedFX: std::mem::transmute(get_func(c_str!(stringify!(GetFocusedFX)))),
                GetGlobalAutomationOverride: std::mem::transmute(get_func(c_str!(stringify!(
                    GetGlobalAutomationOverride
                )))),
                GetLastTouchedFX: std::mem::transmute(get_func(c_str!(stringify!(
                    GetLastTouchedFX
                )))),
                GetMainHwnd: std::mem::transmute(get_func(c_str!(stringify!(GetMainHwnd)))),
                GetMasterTrack: std::mem::transmute(get_func(c_str!(stringify!(GetMasterTrack)))),
                GetMaxMidiInputs: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMaxMidiInputs
                )))),
                GetMaxMidiOutputs: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMaxMidiOutputs
                )))),
                GetMediaTrackInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMediaTrackInfo_Value
                )))),
                GetMIDIInputName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMIDIInputName
                )))),
                GetMIDIOutputName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetMIDIOutputName
                )))),
                GetSelectedTrack2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSelectedTrack2
                )))),
                GetSetMediaTrackInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetMediaTrackInfo
                )))),
                GetSetTrackSendInfo: std::mem::transmute(get_func(c_str!(stringify!(
                    GetSetTrackSendInfo
                )))),
                GetToggleCommandState2: std::mem::transmute(get_func(c_str!(stringify!(
                    GetToggleCommandState2
                )))),
                GetTrack: std::mem::transmute(get_func(c_str!(stringify!(GetTrack)))),
                GetTrackAutomationMode: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackAutomationMode
                )))),
                GetTrackEnvelopeByName: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackEnvelopeByName
                )))),
                GetTrackNumSends: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackNumSends
                )))),
                GetTrackSendUIVolPan: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackSendUIVolPan
                )))),
                GetTrackStateChunk: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackStateChunk
                )))),
                GetTrackUIVolPan: std::mem::transmute(get_func(c_str!(stringify!(
                    GetTrackUIVolPan
                )))),
                guidToString: std::mem::transmute(get_func(c_str!(stringify!(guidToString)))),
                InsertTrackAtIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    InsertTrackAtIndex
                )))),
                IsProjectDirty: std::mem::transmute(get_func(c_str!(stringify!(IsProjectDirty)))),
                kbd_getTextFromCmd: std::mem::transmute(get_func(c_str!(stringify!(
                    kbd_getTextFromCmd
                )))),
                KBD_OnMainActionEx: std::mem::transmute(get_func(c_str!(stringify!(
                    KBD_OnMainActionEx
                )))),
                Main_OnCommandEx: std::mem::transmute(get_func(c_str!(stringify!(
                    Main_OnCommandEx
                )))),
                MarkProjectDirty: std::mem::transmute(get_func(c_str!(stringify!(
                    MarkProjectDirty
                )))),
                Master_GetPlayRate: std::mem::transmute(get_func(c_str!(stringify!(
                    Master_GetPlayRate
                )))),
                Master_GetTempo: std::mem::transmute(get_func(c_str!(stringify!(Master_GetTempo)))),
                NamedCommandLookup: std::mem::transmute(get_func(c_str!(stringify!(
                    NamedCommandLookup
                )))),
                plugin_register: std::mem::transmute(get_func(c_str!(stringify!(plugin_register)))),
                ReverseNamedCommandLookup: std::mem::transmute(get_func(c_str!(stringify!(
                    ReverseNamedCommandLookup
                )))),
                SectionFromUniqueID: std::mem::transmute(get_func(c_str!(stringify!(
                    SectionFromUniqueID
                )))),
                SetCurrentBPM: std::mem::transmute(get_func(c_str!(stringify!(SetCurrentBPM)))),
                SetMediaTrackInfo_Value: std::mem::transmute(get_func(c_str!(stringify!(
                    SetMediaTrackInfo_Value
                )))),
                SetOnlyTrackSelected: std::mem::transmute(get_func(c_str!(stringify!(
                    SetOnlyTrackSelected
                )))),
                SetTrackSelected: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackSelected
                )))),
                SetTrackStateChunk: std::mem::transmute(get_func(c_str!(stringify!(
                    SetTrackStateChunk
                )))),
                ShowConsoleMsg: std::mem::transmute(get_func(c_str!(stringify!(ShowConsoleMsg)))),
                ShowMessageBox: std::mem::transmute(get_func(c_str!(stringify!(ShowMessageBox)))),
                SLIDER2DB: std::mem::transmute(get_func(c_str!(stringify!(SLIDER2DB)))),
                stringToGuid: std::mem::transmute(get_func(c_str!(stringify!(stringToGuid)))),
                StuffMIDIMessage: std::mem::transmute(get_func(c_str!(stringify!(
                    StuffMIDIMessage
                )))),
                TrackFX_AddByName: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_AddByName
                )))),
                TrackFX_CopyToTrack: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_CopyToTrack
                )))),
                TrackFX_Delete: std::mem::transmute(get_func(c_str!(stringify!(TrackFX_Delete)))),
                TrackFX_FormatParamValueNormalized: std::mem::transmute(get_func(c_str!(
                    stringify!(TrackFX_FormatParamValueNormalized)
                ))),
                TrackFX_GetCount: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetCount
                )))),
                TrackFX_GetEnabled: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetEnabled
                )))),
                TrackFX_GetFloatingWindow: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetFloatingWindow
                )))),
                TrackFX_GetFormattedParamValue: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetFormattedParamValue
                )))),
                TrackFX_GetFXGUID: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetFXGUID
                )))),
                TrackFX_GetFXName: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetFXName
                )))),
                TrackFX_GetInstrument: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetInstrument
                )))),
                TrackFX_GetNumParams: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetNumParams
                )))),
                TrackFX_GetOpen: std::mem::transmute(get_func(c_str!(stringify!(TrackFX_GetOpen)))),
                TrackFX_GetParameterStepSizes: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParameterStepSizes
                )))),
                TrackFX_GetParamEx: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParamEx
                )))),
                TrackFX_GetParamName: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParamName
                )))),
                TrackFX_GetParamNormalized: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetParamNormalized
                )))),
                TrackFX_GetPreset: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetPreset
                )))),
                TrackFX_GetPresetIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetPresetIndex
                )))),
                TrackFX_GetRecCount: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_GetRecCount
                )))),
                TrackFX_NavigatePresets: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_NavigatePresets
                )))),
                TrackFX_SetEnabled: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetEnabled
                )))),
                TrackFX_SetParamNormalized: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetParamNormalized
                )))),
                TrackFX_SetPresetByIndex: std::mem::transmute(get_func(c_str!(stringify!(
                    TrackFX_SetPresetByIndex
                )))),
                TrackFX_Show: std::mem::transmute(get_func(c_str!(stringify!(TrackFX_Show)))),
                TrackList_UpdateAllExternalSurfaces: std::mem::transmute(get_func(c_str!(
                    stringify!(TrackList_UpdateAllExternalSurfaces)
                ))),
                Undo_BeginBlock2: std::mem::transmute(get_func(c_str!(stringify!(
                    Undo_BeginBlock2
                )))),
                Undo_CanRedo2: std::mem::transmute(get_func(c_str!(stringify!(Undo_CanRedo2)))),
                Undo_CanUndo2: std::mem::transmute(get_func(c_str!(stringify!(Undo_CanUndo2)))),
                Undo_DoRedo2: std::mem::transmute(get_func(c_str!(stringify!(Undo_DoRedo2)))),
                Undo_DoUndo2: std::mem::transmute(get_func(c_str!(stringify!(Undo_DoUndo2)))),
                Undo_EndBlock2: std::mem::transmute(get_func(c_str!(stringify!(Undo_EndBlock2)))),
                ValidatePtr2: std::mem::transmute(get_func(c_str!(stringify!(ValidatePtr2)))),
                GetMidiInput: std::mem::transmute(get_func(c_str!(stringify!(GetMidiInput)))),
                GetMidiOutput: std::mem::transmute(get_func(c_str!(stringify!(GetMidiOutput)))),
            }
        }
    }
}
