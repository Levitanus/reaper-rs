#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod bindings;

pub use bindings::root::{
    ReaProject, MediaTrack, ACCEL, gaccel_register_t, HINSTANCE, REAPER_PLUGIN_VERSION,
    reaper_plugin_info_t,
};
use bindings::root::reaper_rs_control_surface::get_control_surface;
pub use control_surface::ControlSurface;

mod types;

mod control_surface;

use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use std::convert::AsRef;
use c_str_macro::c_str;
use std::ptr::null_mut;
use vst::api::HostCallbackProc;
use std::sync::Once;

// See https://doc.rust-lang.org/std/sync/struct.Once.html why this is safe in combination with Once
static mut CONTROL_SURFACE_INSTANCE: Option<Box<dyn ControlSurface>> = None;
static INIT_CONTROL_SURFACE_INSTANCE: Once = Once::new();


pub(super) fn get_control_surface_instance() -> &'static Box<dyn ControlSurface> {
    unsafe {
        CONTROL_SURFACE_INSTANCE.as_ref().unwrap()
    }
}


pub fn create_reaper_plugin_function_provider(GetFunc: types::GetFunc) -> impl Fn(&CStr) -> isize {
    move |name| {
        unsafe { GetFunc(name.as_ptr()) as isize }
    }
}

pub fn create_reaper_vst_plugin_function_provider(host_callback: HostCallbackProc) -> impl Fn(&CStr) -> isize {
    move |name| {
        #[allow(overflowing_literals)]
            host_callback(null_mut(), 0xdeadbeef, 0xdeadf00d, 0, name.as_ptr() as *mut c_void, 0.0)
    }
}

macro_rules! gen_reaper_struct {
    ($($func:ident),+) => {
        #[derive(Default)]
        pub struct Reaper {
            $(
                pub $func: Option<types::$func>,
            )*
        }

        impl Reaper {
            // This is provided in addition to the original API functions because we use another
            // loading mechanism, not the one in reaper_plugin_functions.h (in order to collect all
            // function pointers into struct fields instead of global variables, and in order to
            // still keep the possibility of loading only certain functions)
            pub fn with_all_functions_loaded(get_func: &impl Fn(&CStr) -> isize) -> Reaper {
                unsafe {
                    Reaper {
                        $(
                            $func: std::mem::transmute(get_func(c_str!(stringify!($func)))),
                        )*
                    }
                }
            }

            // This is provided in addition to the original API functions because a pure
            // plugin_register("csurf_inst", my_rust_trait_implementing_IReaperControlSurface) isn't
            // going to cut it. Rust structs can't implement pure virtual C++ interfaces.
            // This function sets up the given ControlSurface implemented in Rust but doesn't yet register
            // it. Can be called only once
            pub fn install_control_surface(&self, control_surface: impl ControlSurface + 'static) {
                // TODO Ensure that only called if there's not a control surface registered already
                // Ideally we would have a generic static but as things are now, we need to box it.
                // However, this is not a big deal because control surfaces are only used in the
                // main thread where these minimal performance differences are not significant.
                unsafe {
                    // Save boxed control surface to static variable so that extern "C" functions implemented
                    // in Rust have something to delegate to.
                    INIT_CONTROL_SURFACE_INSTANCE.call_once(|| {
                        CONTROL_SURFACE_INSTANCE = Some(Box::new(control_surface));
                    });
                }
            }

            // It returns a pointer to a C++ object that will delegate to given Rust ControlSurface.
            // The pointer needs to be passed to plugin_register("csurf_inst", <here>) for registering or
            // plugin_register("-csurf_inst", <here>) for unregistering.
            pub fn get_cpp_control_surface(&self) -> *mut c_void {
                // Create and return C++ IReaperControlSurface implementations which calls extern "C"
                // functions implemented in RUst
                unsafe { get_control_surface() }
            }
        }
    }
}

gen_reaper_struct![
    EnumProjects,
    GetTrack,
    ShowConsoleMsg,
    ValidatePtr2,
    GetSetMediaTrackInfo,
    plugin_register
];

#[macro_export]
macro_rules! customize_reaper_with_functions {
    ($($func:ident),+) => {
        impl $crate::low_level::Reaper {
            pub fn with_custom_functions_loaded(get_func: &impl Fn(&CStr) -> isize) -> $crate::low_level::Reaper {
                unsafe {
                    $crate::low_level::Reaper {
                        $(
                            $func: std::mem::transmute(get_func(c_str!(stringify!($func)))),
                        )*
                        ..Default::default()
                    }
                }
            }
        }
    }
}