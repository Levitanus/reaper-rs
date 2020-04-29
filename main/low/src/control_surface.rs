#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use super::{firewall, raw::MediaTrack};
use crate::raw;

use std::os::raw::c_void;
use std::panic::RefUnwindSafe;
use std::ptr::{null, null_mut, NonNull};
use std::sync::Once;

/// This is the Rust analog to the C++ virtual base class `IReaperControlSurface`. An implementation
/// of this trait can be passed to [`install_control_surface`](fn.install_control_surface.html).
/// As a consequence, REAPER will invoke the respective callback methods.
///
/// # Design
///
/// ## Why do most methods here don't take `&mut self` as parameter?
/// **Short answer:** Because we follow the spirit of Rust here, which is to fail fast and thereby
/// prevent undefined behavior.
///
/// **Long answer:** Taking `self` as `&mut` in control surface methods would give us a dangerous
/// illusion of safety (safety as defined by Rust). It would tell Rust developers "It's safe here to
/// mutate the state of my control surface struct". But in reality it's not safe. Not because of
/// multi-threading (ControlSurfaces methods are invoked by REAPER's main thread only) but because
/// of reentrancy. That can happen quite easily, just think of this scenario: A track is changed,
/// REAPER notifies us about it by calling a ControlSurface method, thereby causing another change
/// in REAPER which in turn synchronously notifies our ControlSurface again while our first method
/// is still running ... and there you go: 2 mutable borrows of `self`. In a Rust-only world, Rust's
/// compiler wouldn't allow us to do that. But Rust won't save us here because the call comes from
/// "outside". By not having a `&mut self` reference, developers are forced to explicitly think
/// about this scenario. One can use a `RefCell` along with `borrow_mut()` to still mutate some
/// control surface state and failing fast whenever reentrancy happens - at runtime, by getting a
/// panic. This is not as good as failing fast at compile time but still much better than to run
/// into undefined behavior, which could cause hard-to-find bugs and crash REAPER - that's the last
/// thing we want! Panicking is not so bad. We can catch it before it reaches REAPER and therefore
/// let REAPER continue running. Ideally it's observed by the developer when he tests his plugin.
/// Then he can think about how to solve that issue. They might find out that it's okay and
/// therefore use some unsafe code to prevent the panic. They might find out that they want to check
/// for reentrancy by using `try_borrow_mut()`. Or they might find out that they want to
/// avoid this situation by just deferring the event handling to the next main loop cycle.
pub trait IReaperControlSurface: RefUnwindSafe {
    fn GetTypeString(&self) -> *const ::std::os::raw::c_char {
        null()
    }

    fn GetDescString(&self) -> *const ::std::os::raw::c_char {
        null()
    }

    fn GetConfigString(&self) -> *const ::std::os::raw::c_char {
        null()
    }

    fn CloseNoReset(&self) {}

    fn Run(&mut self) {}

    fn SetTrackListChange(&self) {}

    fn SetSurfaceVolume(&self, _trackid: *mut MediaTrack, _volume: f64) {}

    fn SetSurfacePan(&self, _trackid: *mut MediaTrack, _pan: f64) {}

    fn SetSurfaceMute(&self, _trackid: *mut MediaTrack, _mute: bool) {}

    fn SetSurfaceSelected(&self, _trackid: *mut MediaTrack, _selected: bool) {}

    fn SetSurfaceSolo(&self, _trackid: *mut MediaTrack, _solo: bool) {}

    fn SetSurfaceRecArm(&self, _trackid: *mut MediaTrack, _recarm: bool) {}

    fn SetPlayState(&self, _play: bool, _pause: bool, _rec: bool) {}

    fn SetRepeatState(&self, _rep: bool) {}

    fn SetTrackTitle(&self, _trackid: *mut MediaTrack, _title: *const ::std::os::raw::c_char) {}

    fn GetTouchState(&self, _trackid: *mut MediaTrack, _isPan: ::std::os::raw::c_int) -> bool {
        false
    }

    fn SetAutoMode(&self, _mode: ::std::os::raw::c_int) {}

    fn ResetCachedVolPanStates(&self) {}

    fn OnTrackSelection(&self, _trackid: *mut MediaTrack) {}

    fn IsKeyDown(&self, _key: ::std::os::raw::c_int) -> bool {
        false
    }

    fn Extended(
        &self,
        _call: ::std::os::raw::c_int,
        _parm1: *mut ::std::os::raw::c_void,
        _parm2: *mut ::std::os::raw::c_void,
        _parm3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        0
    }
}

/// This function for installing a REAPER control surface is provided because
/// `plugin_register("csurf_inst", my_rust_struct)` isn't going to work. Rust structs can't
/// implement C++ virtual base classes.
///
/// This function sets up the given control surface implemented in Rust but **doesn't yet
/// register it**! Because you are not using the high-level API, the usual REAPER C++ way to
/// register a control surface still applies. See
/// [`get_cpp_control_surface`](fn.get_cpp_control_surface.html). If you register a control surface,
/// you also must take care of unregistering it at the end. This is especially important for VST
/// plug-ins because they live shorter than a REAPER session! **If you don't unregister the control
/// surface before the VST plug-in is destroyed, REAPER will crash** because it will attempt to
/// invoke functions which are not loaded anymore. This kind of responsibility is gone when using
/// the high-level API.     
/// TODO-medium Maybe better to return a NonNull pointer?
/// This returns a reference of a `IReaperControlSurface`-implementing C++ object which will
/// delegate to the Rust [`ControlSurface`](trait.ControlSurface.html) which you installed by
/// invoking [`install_control_surface`](fn.install_control_surface.html). It needs to be
/// passed to [`plugin_register`](struct.Reaper.html#structfield.plugin_register) as a pointer as in
/// `plugin_register("csurf_inst", cs as *mut _ as *mut c_void)` for registering and as in
/// `plugin_register("-csurf_inst", cs as *mut _ as *mut c_void)` for unregistering.
pub unsafe fn add_cpp_control_surface(
    callback_target: NonNull<Box<dyn IReaperControlSurface>>,
) -> NonNull<raw::IReaperControlSurface> {
    let instance = crate::bindings::root::reaper_rs_control_surface::add_control_surface(
        callback_target.as_ptr() as *mut c_void,
    );
    NonNull::new_unchecked(instance)
}

pub unsafe fn remove_cpp_control_surface(surface: NonNull<raw::IReaperControlSurface>) {
    crate::bindings::root::reaper_rs_control_surface::remove_control_surface(surface.as_ptr());
}

#[no_mangle]
extern "C" fn GetTypeString(
    callback_target: *mut Box<dyn IReaperControlSurface>,
) -> *const ::std::os::raw::c_char {
    firewall(|| unsafe { &*callback_target }.GetTypeString()).unwrap_or(null_mut())
}

#[no_mangle]
extern "C" fn GetDescString(
    callback_target: *mut Box<dyn IReaperControlSurface>,
) -> *const ::std::os::raw::c_char {
    firewall(|| unsafe { &*callback_target }.GetDescString()).unwrap_or(null_mut())
}

#[no_mangle]
extern "C" fn GetConfigString(
    callback_target: *mut Box<dyn IReaperControlSurface>,
) -> *const ::std::os::raw::c_char {
    firewall(|| unsafe { &*callback_target }.GetConfigString()).unwrap_or(null_mut())
}

#[no_mangle]
extern "C" fn CloseNoReset(callback_target: *mut Box<dyn IReaperControlSurface>) {
    firewall(|| unsafe { &*callback_target }.CloseNoReset());
}

#[no_mangle]
extern "C" fn Run(mut callback_target: *mut Box<dyn IReaperControlSurface>) {
    // "Decoding" the thin pointer is not necessary right now because we have a static variable.
    // However, we leave it. Might come in handy one day to support multiple control surfaces
    // (see https://users.rust-lang.org/t/sending-a-boxed-trait-over-ffi/21708/6)
    firewall(|| unsafe { &mut *callback_target }.Run());
}

#[no_mangle]
extern "C" fn SetTrackListChange(callback_target: *mut Box<dyn IReaperControlSurface>) {
    firewall(|| unsafe { &*callback_target }.SetTrackListChange());
}

#[no_mangle]
extern "C" fn SetSurfaceVolume(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
    volume: f64,
) {
    firewall(|| unsafe { &*callback_target }.SetSurfaceVolume(trackid, volume));
}

#[no_mangle]
extern "C" fn SetSurfacePan(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
    pan: f64,
) {
    firewall(|| unsafe { &*callback_target }.SetSurfacePan(trackid, pan));
}

#[no_mangle]
extern "C" fn SetSurfaceMute(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
    mute: bool,
) {
    firewall(|| unsafe { &*callback_target }.SetSurfaceMute(trackid, mute));
}

#[no_mangle]
extern "C" fn SetSurfaceSelected(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
    selected: bool,
) {
    firewall(|| unsafe { &*callback_target }.SetSurfaceSelected(trackid, selected));
}

#[no_mangle]
extern "C" fn SetSurfaceSolo(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
    solo: bool,
) {
    firewall(|| unsafe { &*callback_target }.SetSurfaceSolo(trackid, solo));
}

#[no_mangle]
extern "C" fn SetSurfaceRecArm(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
    recarm: bool,
) {
    firewall(|| unsafe { &*callback_target }.SetSurfaceRecArm(trackid, recarm));
}

#[no_mangle]
extern "C" fn SetPlayState(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    play: bool,
    pause: bool,
    rec: bool,
) {
    firewall(|| unsafe { &*callback_target }.SetPlayState(play, pause, rec));
}

#[no_mangle]
extern "C" fn SetRepeatState(callback_target: *mut Box<dyn IReaperControlSurface>, rep: bool) {
    firewall(|| unsafe { &*callback_target }.SetRepeatState(rep));
}

#[no_mangle]
extern "C" fn SetTrackTitle(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
    title: *const ::std::os::raw::c_char,
) {
    firewall(|| unsafe { &*callback_target }.SetTrackTitle(trackid, title));
}

#[no_mangle]
extern "C" fn GetTouchState(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
    isPan: ::std::os::raw::c_int,
) -> bool {
    firewall(|| unsafe { &*callback_target }.GetTouchState(trackid, isPan)).unwrap_or(false)
}

#[no_mangle]
extern "C" fn SetAutoMode(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    mode: ::std::os::raw::c_int,
) {
    firewall(|| unsafe { &*callback_target }.SetAutoMode(mode));
}

#[no_mangle]
extern "C" fn ResetCachedVolPanStates(callback_target: *mut Box<dyn IReaperControlSurface>) {
    firewall(|| unsafe { &*callback_target }.ResetCachedVolPanStates());
}

#[no_mangle]
extern "C" fn OnTrackSelection(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    trackid: *mut MediaTrack,
) {
    firewall(|| unsafe { &*callback_target }.OnTrackSelection(trackid));
}

#[no_mangle]
extern "C" fn IsKeyDown(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    key: ::std::os::raw::c_int,
) -> bool {
    firewall(|| unsafe { &*callback_target }.IsKeyDown(key)).unwrap_or(false)
}

#[no_mangle]
extern "C" fn Extended(
    callback_target: *mut Box<dyn IReaperControlSurface>,
    call: ::std::os::raw::c_int,
    parm1: *mut ::std::os::raw::c_void,
    parm2: *mut ::std::os::raw::c_void,
    parm3: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    firewall(|| unsafe { &*callback_target }.Extended(call, parm1, parm2, parm3)).unwrap_or(0)
}
