use std::borrow::Cow;
use std::cell::{Cell, Ref, RefCell, RefMut, UnsafeCell};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_ushort, c_void};
use std::ptr::{null, null_mut, NonNull};
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Once};
use std::thread;
use std::thread::ThreadId;

use c_str_macro::c_str;
use num_enum::IntoPrimitive;

use rxrust::prelude::*;

use crate::fx::Fx;
use crate::fx_parameter::FxParameter;
use crate::helper_control_surface::HelperControlSurface;
use crate::track_send::TrackSend;
use crate::undo_block::UndoBlock;
use crate::ActionKind::Toggleable;
use crate::{
    create_default_console_msg_formatter, create_reaper_panic_hook, create_std_logger,
    create_terminal_logger, Action, Guid, MidiInputDevice, MidiOutputDevice, Project, Section,
    Track,
};
use helgoboss_midi::{ShortMessage, ShortMessageType};
use reaper_rs_low;
use reaper_rs_low::raw;
use reaper_rs_low::raw::{audio_hook_register_t, gaccel_register_t, ACCEL};
use reaper_rs_low::{firewall, ReaperPluginContext};
use reaper_rs_medium;
use reaper_rs_medium::ProjectContext::Proj;
use reaper_rs_medium::UndoScope::All;
use reaper_rs_medium::{
    install_control_surface, AudioHookRegister, CommandId, GaccelRegister, GetFocusedFxResult,
    GetLastTouchedFxResult, GlobalAutomationOverride, Hwnd, MediumAccelerator,
    MediumAudioHookRegister, MediumGaccelRegister, MediumHookCommand, MediumHookPostCommand,
    MediumOnAudioBuffer, MediumToggleAction, MessageBoxResult, MessageBoxType, MidiEvent,
    MidiInputDeviceId, MidiOutputDeviceId, ProjectRef, RealtimeReaper, ReaperStringArg,
    ReaperVersion, SectionId, StuffMidiMessageTarget, TrackRef,
};
use std::time::{Duration, SystemTime};

/// Access to this variable is encapsulated in 3 functions:
/// - Reaper::setup()
/// - Reaper::get()
/// - Reaper::teardown()
///
/// We  make sure that ...
///
/// 1. Reaper::setup() is only ever called from the main thread
///    => Check when entering function
/// 2. Reaper::get() is only ever called from one thread (main thread). Even the exposed
///    Reaper reference is not mutable, the struct has members that have interior
///    mutability, and that is done via RefCells.
///    => Check when entering function
///    => Provide a separate RealtimeReaper::get()
/// 4. Reaper::teardown() is only ever called from the main thread
///    => Check when entering function
/// 5. Reaper::teardown() is not called while a get() reference is still active.
///    => Wrap Option in a RefCell.
static mut REAPER_INSTANCE: RefCell<Option<Reaper>> = RefCell::new(None);

// Called by REAPER (using a delegate function)!
// Only for main section
struct HighLevelHookCommand {}

impl MediumHookCommand for HighLevelHookCommand {
    fn call(command_id: CommandId, _flag: i32) -> bool {
        // TODO-low Pass on flag
        let operation = match Reaper::get().command_by_id.borrow().get(&command_id) {
            Some(command) => command.operation.clone(),
            None => return false,
        };
        (*operation).borrow_mut().call_mut(());
        true
    }
}

// Called by REAPER directly (using a delegate function)!
// Only for main section
struct HighLevelHookPostCommand {}

impl MediumHookPostCommand for HighLevelHookPostCommand {
    fn call(command_id: CommandId, _flag: i32) {
        let reaper = Reaper::get();
        let action = reaper
            .get_main_section()
            .get_action_by_command_id(command_id);
        reaper
            .subjects
            .action_invoked
            .borrow_mut()
            .next(Payload(Rc::new(action)));
    }
}

// Called by REAPER directly!
// Only for main section
struct HighLevelToggleAction {}

impl MediumToggleAction for HighLevelToggleAction {
    fn call(command_id: CommandId) -> i32 {
        if let Some(command) = Reaper::get().command_by_id.borrow().get(&(command_id)) {
            match &command.kind {
                ActionKind::Toggleable(is_on) => {
                    if is_on() {
                        1
                    } else {
                        0
                    }
                }
                ActionKind::NotToggleable => -1,
            }
        } else {
            -1
        }
    }
}

struct HighOnAudioBuffer {}

impl MediumOnAudioBuffer for HighOnAudioBuffer {
    type UserData1 = RealtimeReaper;
    type UserData2 = ();

    fn call(is_post: bool, len: i32, srate: f64, reg: AudioHookRegister<RealtimeReaper, ()>) {
        if is_post {
            return;
        }
        // TODO-low Check performance implications for Reaper instance unwrapping
        let reaper = match reg.user_data_1() {
            None => return,
            Some(r) => r,
        };
        // TODO-low Should we use an unsafe cell here for better performance?
        // TODO-medium Fix this
        // let mut subject = reaper.subjects.midi_message_received.borrow_mut();
        // if subject.subscribed_size() == 0 {
        //     return;
        // }
        for i in 0..reaper.get_max_midi_inputs() {
            reaper.get_midi_input(MidiInputDeviceId::new(i as u8), |input| {
                let evt_list = input.get_read_buf();
                for evt in evt_list.enum_items(0) {
                    if evt.get_message().r#type() == ShortMessageType::ActiveSensing {
                        // TODO-low We should forward active sensing. Can be filtered out
                        // later.
                        continue;
                    }
                    // Erase lifetime of event so we can "send" it using rxRust
                    // TODO This is very hacky and unsafe. It works as long as there's no
                    // rxRust  subscriber (e.g. operator)
                    // involved which attempts to cache the event
                    //  and use it after this function has returned. Then segmentation
                    // faults are  about to happen. Alternative
                    // would be to turn this into an owned event
                    // and  send this instead. But note that we
                    // are in a real-time thread here so we
                    //  shouldn't allocate on the heap here (so no Rc). That means we would
                    // have to  copy the owned MIDI event.
                    // Probably not an issue because it's not
                    // big and  cheap to copy. Look into this
                    // and see if the unsafe code is worth it.
                    let fake_static_evt: MidiEvent<'static> = {
                        let raw_evt: &raw::MIDI_event_t = evt.into();
                        let raw_evt_ptr = raw_evt as *const raw::MIDI_event_t;
                        unsafe {
                            let erased_raw_evt = &*raw_evt_ptr;
                            MidiEvent::new(erased_raw_evt)
                        }
                    };
                    // subject.next(fake_static_evt);
                }
            });
        }
    }
}

pub(super) type Task = Box<dyn FnOnce() + 'static>;

pub(super) struct ScheduledTask {
    pub desired_execution_time: Option<std::time::SystemTime>,
    pub task: Task,
}

impl ScheduledTask {
    pub fn new(task: Task, desired_execution_time: Option<std::time::SystemTime>) -> ScheduledTask {
        ScheduledTask {
            desired_execution_time,
            task,
        }
    }
}

pub struct ReaperBuilder {
    medium: reaper_rs_medium::Reaper,
    logger: Option<slog::Logger>,
}

impl ReaperBuilder {
    fn with_all_functions_loaded(context: &ReaperPluginContext) -> ReaperBuilder {
        ReaperBuilder {
            medium: {
                let low = reaper_rs_low::Reaper::load(context);
                reaper_rs_medium::Reaper::new(low)
            },
            logger: Default::default(),
        }
    }

    pub fn logger(mut self, logger: slog::Logger) -> ReaperBuilder {
        self.logger = Some(logger);
        self
    }

    pub fn setup(self) {
        Reaper::setup(self.medium, self.logger.unwrap_or_else(create_std_logger));
    }
}

pub fn setup_reaper_with_defaults(context: &ReaperPluginContext, email_address: &'static str) {
    Reaper::load(context)
        .logger(create_terminal_logger())
        .setup();
    std::panic::set_hook(create_reaper_panic_hook(
        create_terminal_logger(),
        Some(create_default_console_msg_formatter(email_address)),
    ));
}

pub struct Reaper {
    medium: RefCell<reaper_rs_medium::Reaper>,
    logger: slog::Logger,
    // We take a mutable reference from this RefCell in order to add/remove commands.
    // TODO-low Adding an action in an action would panic because we have an immutable borrow of
    // the map  to obtain and execute the command, plus a mutable borrow of the map to add the
    // new command.  (the latter being unavoidable because we somehow need to modify the map!).
    //  That's not good. Is there a way to avoid this constellation? It's probably hard to avoid
    // the  immutable borrow because the `operation` is part of the map after all. And we can't
    // just  copy it before execution, at least not when it captures and mutates state, which
    // might not  be copyable (which we want to explicitly allow, that's why we accept FnMut!).
    // Or is it  possible to give up the map borrow after obtaining the command/operation
    // reference???  Look into that!!!
    command_by_id: RefCell<HashMap<CommandId, Command>>,
    pub(super) subjects: EventStreamSubjects,
    task_sender: Sender<ScheduledTask>,
    main_thread_id: ThreadId,
    undo_block_is_active: Cell<bool>,
    audio_hook_register_handle: Cell<Option<NonNull<raw::audio_hook_register_t>>>,
}

pub(super) struct EventStreamSubjects {
    // This is a RefCell. So calling next() while another next() is still running will panic.
    // I guess it's good that way because this is very generic code, panicking or not panicking
    // depending on the user's code. And getting a panic is good for becoming aware of the problem
    // instead of running into undefined behavior. The developer can always choose to defer to
    // the next `ControlSurface::run()` invocation (execute things in next main loop cycle).
    pub(super) project_switched: EventStreamSubject<Project>,
    pub(super) track_volume_changed: EventStreamSubject<Track>,
    pub(super) track_volume_touched: EventStreamSubject<Track>,
    pub(super) track_pan_changed: EventStreamSubject<Track>,
    pub(super) track_pan_touched: EventStreamSubject<Track>,
    pub(super) track_send_volume_changed: EventStreamSubject<TrackSend>,
    pub(super) track_send_volume_touched: EventStreamSubject<TrackSend>,
    pub(super) track_send_pan_changed: EventStreamSubject<TrackSend>,
    pub(super) track_send_pan_touched: EventStreamSubject<TrackSend>,
    pub(super) track_added: EventStreamSubject<Track>,
    pub(super) track_removed: EventStreamSubject<Track>,
    pub(super) tracks_reordered: EventStreamSubject<Project>,
    pub(super) track_name_changed: EventStreamSubject<Track>,
    pub(super) track_input_changed: EventStreamSubject<Track>,
    pub(super) track_input_monitoring_changed: EventStreamSubject<Track>,
    pub(super) track_arm_changed: EventStreamSubject<Track>,
    pub(super) track_mute_changed: EventStreamSubject<Track>,
    pub(super) track_mute_touched: EventStreamSubject<Track>,
    pub(super) track_solo_changed: EventStreamSubject<Track>,
    pub(super) track_selected_changed: EventStreamSubject<Track>,
    pub(super) fx_added: EventStreamSubject<Fx>,
    pub(super) fx_removed: EventStreamSubject<Fx>,
    pub(super) fx_enabled_changed: EventStreamSubject<Fx>,
    pub(super) fx_opened: EventStreamSubject<Fx>,
    pub(super) fx_closed: EventStreamSubject<Fx>,
    pub(super) fx_focused: EventStreamSubject<Payload<Option<Fx>>>,
    pub(super) fx_reordered: EventStreamSubject<Track>,
    pub(super) fx_parameter_value_changed: EventStreamSubject<FxParameter>,
    pub(super) fx_parameter_touched: EventStreamSubject<FxParameter>,
    pub(super) master_tempo_changed: EventStreamSubject<()>,
    pub(super) master_tempo_touched: EventStreamSubject<()>,
    pub(super) master_playrate_changed: EventStreamSubject<bool>,
    pub(super) master_playrate_touched: EventStreamSubject<bool>,
    pub(super) main_thread_idle: EventStreamSubject<bool>,
    pub(super) project_closed: EventStreamSubject<Project>,
    pub(super) action_invoked: EventStreamSubject<Payload<Rc<Action>>>,
    pub(super) midi_message_received: EventStreamSubject<MidiEvent<'static>>,
}

#[derive(Clone)]
pub struct Payload<T>(pub T);

impl<T: Clone> PayloadCopy for Payload<T> {}

impl EventStreamSubjects {
    fn new() -> EventStreamSubjects {
        fn default<T>() -> EventStreamSubject<T> {
            RefCell::new(LocalSubject::new())
        }
        EventStreamSubjects {
            project_switched: default(),
            track_volume_changed: default(),
            track_volume_touched: default(),
            track_pan_changed: default(),
            track_pan_touched: default(),
            track_send_volume_changed: default(),
            track_send_volume_touched: default(),
            track_send_pan_changed: default(),
            track_send_pan_touched: default(),
            track_added: default(),
            track_removed: default(),
            tracks_reordered: default(),
            track_name_changed: default(),
            track_input_changed: default(),
            track_input_monitoring_changed: default(),
            track_arm_changed: default(),
            track_mute_changed: default(),
            track_mute_touched: default(),
            track_solo_changed: default(),
            track_selected_changed: default(),
            fx_added: default(),
            fx_removed: default(),
            fx_enabled_changed: default(),
            fx_opened: default(),
            fx_closed: default(),
            fx_focused: default(),
            fx_reordered: default(),
            fx_parameter_value_changed: default(),
            fx_parameter_touched: default(),
            master_tempo_changed: default(),
            master_tempo_touched: default(),
            master_playrate_changed: default(),
            master_playrate_touched: default(),
            main_thread_idle: default(),
            project_closed: default(),
            action_invoked: default(),
            midi_message_received: default(),
        }
    }
}

pub enum ActionKind {
    NotToggleable,
    Toggleable(Box<dyn Fn() -> bool>),
}

pub fn toggleable(is_on: impl Fn() -> bool + 'static) -> ActionKind {
    Toggleable(Box::new(is_on))
}

type EventStreamSubject<T> = RefCell<LocalSubject<'static, T, ()>>;

impl Drop for Reaper {
    fn drop(&mut self) {
        self.deactivate();
    }
}

impl Reaper {
    pub fn load(context: &ReaperPluginContext) -> ReaperBuilder {
        ReaperBuilder::with_all_functions_loaded(context)
    }

    fn setup(medium: reaper_rs_medium::Reaper, logger: slog::Logger) {
        // We can't check now if we are in main thread because we don't have the main thread ID yet.
        // But at least we can make sure we are not in an audio thread. Whatever thread we are
        // in right now, this struct will memorize it as the main thread.
        assert!(
            !medium.is_in_real_time_audio(),
            "Reaper::setup() must be called from main thread"
        );
        assert!(
            unsafe { REAPER_INSTANCE.borrow().is_none() },
            "There's a Reaper instance already"
        );
        let (task_sender, task_receiver) = mpsc::channel::<ScheduledTask>();
        let reaper = Reaper {
            medium: RefCell::new(medium),
            logger,
            command_by_id: RefCell::new(HashMap::new()),
            subjects: EventStreamSubjects::new(),
            task_sender: task_sender.clone(),
            main_thread_id: thread::current().id(),
            undo_block_is_active: Cell::new(false),
            audio_hook_register_handle: Cell::new(None),
        };
        unsafe {
            REAPER_INSTANCE.replace(Some(reaper));
        }
        Reaper::get().init(task_sender, task_receiver);
    }

    // Allowing global access to native REAPER functions at all times is valid in my opinion.
    // Because REAPER itself is not written in Rust and therefore cannot take part in Rust's compile
    // time guarantees anyway. We need to rely on REAPER at that point and also take care not to do
    // something which is not allowed in Reaper (by reading documentation and learning from
    // mistakes ... no compiler is going to save us from them). REAPER as a whole is always mutable
    // from the perspective of extensions.
    //
    // We express that in Rust by making `Reaper` class an immutable (in the sense of non-`&mut`)
    // singleton and allowing all REAPER functions to be called from an immutable context ...
    // although they can and often will lead to mutations within REAPER!
    pub fn get() -> Ref<'static, Reaper> {
        let reaper =
            Reaper::obtain_reaper_ref("Reaper::setup() must be called before Reaper::get()");
        // TODO-medium Introduce RealtimeReaper and activate this!
        // assert!(
        //     !reaper.medium().is_in_real_time_audio(),
        //     "Reaper::get() must be called from main thread"
        // );
        reaper
    }

    pub fn teardown() {
        {
            let reaper = Reaper::obtain_reaper_ref("There's no Reaper instance to teardown");
            assert!(
                !reaper.medium().is_in_real_time_audio(),
                "Reaper::teardown() must be called from main thread"
            );
        }
        unsafe {
            REAPER_INSTANCE.replace(None);
        }
    }

    fn obtain_reaper_ref(error_msg_if_none: &'static str) -> Ref<'static, Reaper> {
        Ref::map(unsafe { REAPER_INSTANCE.borrow() }, |r| match r {
            None => panic!(error_msg_if_none),
            Some(r) => r,
        })
    }

    fn init(&self, task_sender: Sender<ScheduledTask>, task_receiver: Receiver<ScheduledTask>) {
        install_control_surface(
            HelperControlSurface::new(task_sender, task_receiver),
            &self.get_version(),
        );
    }

    // TODO-low Must be idempotent
    pub fn activate(&self) {
        let mut medium = self.medium_mut();
        medium.plugin_register_add_hookcommand::<HighLevelHookCommand>();
        medium.plugin_register_add_toggleaction::<HighLevelToggleAction>();
        medium.plugin_register_add_hookpostcommand::<HighLevelHookPostCommand>();
        medium.register_control_surface();
        if self.audio_hook_register_handle.get().is_none() {
            let realtime_reaper = medium.create_realtime_reaper();
            let handle = medium
                .audio_reg_hardware_hook_add(
                    MediumAudioHookRegister::new::<HighOnAudioBuffer, _, _>(
                        Some(realtime_reaper),
                        None,
                    ),
                )
                .unwrap();
            self.audio_hook_register_handle.set(Some(handle))
        }
    }

    // TODO-low Must be idempotent
    pub fn deactivate(&self) {
        let mut medium = self.medium_mut();
        if let Some(handle) = self.audio_hook_register_handle.get() {
            self.audio_hook_register_handle.set(None);
            medium.audio_reg_hardware_hook_remove(handle);
        }
        medium.unregister_control_surface();
        medium.plugin_register_remove_hookpostcommand::<HighLevelHookPostCommand>();
        medium.plugin_register_remove_toggleaction::<HighLevelToggleAction>();
        medium.plugin_register_remove_hookcommand::<HighLevelHookCommand>();
    }

    pub fn medium(&self) -> Ref<reaper_rs_medium::Reaper> {
        self.medium.borrow()
    }

    pub fn medium_mut(&self) -> RefMut<reaper_rs_medium::Reaper> {
        self.medium.borrow_mut()
    }

    pub fn get_version(&self) -> ReaperVersion {
        self.medium().get_app_version()
    }

    pub fn get_last_touched_fx_parameter(&self) -> Option<FxParameter> {
        // TODO-low Sucks: We have to assume it was a parameter in the current project
        //  Maybe we should rather rely on our own technique in ControlSurface here!
        // fxQueryIndex is only a real query index since REAPER 5.95, before it didn't say if it's
        // input FX or normal one!
        self.medium().get_last_touched_fx().and_then(|result| {
            use GetLastTouchedFxResult::*;
            match result {
                TrackFx {
                    track_ref,
                    fx_ref,
                    param_index,
                } => {
                    // Track exists in this project
                    use TrackRef::*;
                    let track = match track_ref {
                        MasterTrack => self.get_current_project().get_master_track(),
                        NormalTrack(idx) => {
                            if idx >= self.get_current_project().get_track_count() {
                                // Must be in another project
                                return None;
                            }
                            self.get_current_project().get_track_by_index(idx).unwrap()
                        }
                    };
                    // TODO We should rethink the query index methods now that we have an FxRef
                    //  enum in medium-level API
                    let fx = match track.get_fx_by_query_index(fx_ref.into()) {
                        None => return None,
                        Some(fx) => fx,
                    };
                    Some(fx.get_parameter_by_index(param_index))
                }
                ItemFx { .. } => None, // TODO-low Implement,
            }
        })
    }

    pub fn generate_guid(&self) -> Guid {
        Guid::new(Reaper::get().medium().gen_guid())
    }

    pub fn register_action(
        &self,
        command_name: &CStr,
        description: impl Into<Cow<'static, CStr>>,
        operation: impl FnMut() + 'static,
        kind: ActionKind,
    ) -> RegisteredAction {
        let mut medium = self.medium_mut();
        let command_id = medium.plugin_register_add_command_id(command_name);
        let command = Command::new(Rc::new(RefCell::new(operation)), kind);
        if let Entry::Vacant(p) = self.command_by_id.borrow_mut().entry(command_id) {
            p.insert(command);
        }
        let address = medium
            .plugin_register_add_gaccel(MediumGaccelRegister::new(
                MediumAccelerator::new(0, 0, command_id),
                description.into(),
            ))
            .unwrap();
        RegisteredAction::new(command_id, address)
    }

    fn unregister_command(&self, command_id: CommandId, gaccel_handle: GaccelRegister) {
        // Unregistering command when it's destroyed via RAII (implementing Drop)? Bad idea, because
        // this is the wrong point in time. The right point in time for unregistering is when it's
        // removed from the command hash map. Because even if the command still exists in memory,
        // if it's not in the map anymore, REAPER won't be able to find it.
        let mut command_by_id = self.command_by_id.borrow_mut();
        if let Some(command) = command_by_id.get_mut(&command_id) {
            self.medium_mut()
                .plugin_register_remove_gaccel(gaccel_handle);
            command_by_id.remove(&command_id);
        }
    }

    pub fn get_max_midi_input_devices(&self) -> u32 {
        self.medium().get_max_midi_inputs()
    }

    pub fn get_max_midi_output_devices(&self) -> u32 {
        self.medium().get_max_midi_outputs()
    }

    // It's correct that this method returns a non-optional. An id is supposed to uniquely identify
    // a device. A MidiInputDevice#isAvailable method returns if the device is actually existing
    // at runtime. That way we support (still) unloaded MidiInputDevices.
    pub fn get_midi_input_device_by_id(&self, id: MidiInputDeviceId) -> MidiInputDevice {
        MidiInputDevice::new(id)
    }

    // It's correct that this method returns a non-optional. An id is supposed to uniquely identify
    // a device. A MidiOutputDevice#isAvailable method returns if the device is actually
    // existing at runtime. That way we support (still) unloaded MidiOutputDevices.
    pub fn get_midi_output_device_by_id(&self, id: MidiOutputDeviceId) -> MidiOutputDevice {
        MidiOutputDevice::new(id)
    }

    pub fn get_midi_input_devices(&self) -> impl Iterator<Item = MidiInputDevice> + '_ {
        (0..self.get_max_midi_input_devices())
            .map(move |i| self.get_midi_input_device_by_id(MidiInputDeviceId::new(i as u8)))
            // TODO-low I think we should also return unavailable devices. Client can filter easily.
            .filter(|d| d.is_available())
    }

    pub fn get_midi_output_devices(&self) -> impl Iterator<Item = MidiOutputDevice> + '_ {
        (0..self.get_max_midi_output_devices())
            .map(move |i| self.get_midi_output_device_by_id(MidiOutputDeviceId::new(i as u8)))
            // TODO-low I think we should also return unavailable devices. Client can filter easily.
            .filter(|d| d.is_available())
    }

    pub fn get_currently_loading_or_saving_project(&self) -> Option<Project> {
        let ptr = self.medium().get_current_project_in_load_save()?;
        Some(Project::new(ptr))
    }

    // It's correct that this method returns a non-optional. A commandName is supposed to uniquely
    // identify the action, so it could be part of the resulting Action itself. An
    // Action#isAvailable method could return if the action is actually existing at runtime.
    // That way we would support (still) unloaded Actions. TODO-low Don't automatically
    // interpret command name as commandId
    pub fn get_action_by_command_name(&self, command_name: CString) -> Action {
        Action::command_name_based(command_name)
    }

    /// # Examples
    ///
    /// ## Passing literal with zero runtime overhead
    /// ```
    /// reaper.show_console_msg(c_str!("Hello from Rust!"))
    /// ```
    /// - Uses macro `c_str!` to create new 0-terminated static literal embedded in binary
    ///
    /// ## Passing 0-terminated literal with borrowing
    /// ```
    /// let literal = "Hello from Rust!\0";
    /// reaper.show_console_msg(CStr::from_bytes_with_nul(literal.as_bytes()).unwrap())
    /// ```
    /// - You *must* make sure that the literal is 0-terminated, otherwise it will panic
    /// - Checks for existing 0 bytes
    /// - No copying involved
    ///
    /// ## Passing 0-terminated owned string with borrowing
    /// ```
    /// let owned = String::from("Hello from Rust!\0");
    /// reaper.show_console_msg(CStr::from_bytes_with_nul(owned.as_bytes()).unwrap())
    /// ```
    /// - You *must* make sure that the String is 0-terminated, otherwise it will panic
    /// - Checks for existing 0 bytes
    /// - No copying involved
    ///
    /// ## Passing not 0-terminated owned string with moving
    /// ```
    /// let owned = String::from("Hello from Rust!");
    /// reaper.show_console_msg(&CString::new(owned).unwrap())
    /// ```
    /// - Moves owned string for appending 0 byte (maybe increasing String capacity)
    /// - Checks for existing 0 bytes
    /// - No copying involved
    ///
    /// ## Absolutely zero-overhead variations
    ///
    /// If you really need absolutely zero-overhead, you need to resort to unsafe functions. But
    /// this should be done only in situations when you are very constrained, e.g. in audio thread
    /// (which is forbidden to call most of the REAPER SDK functions anyway).
    ///
    /// Look into [from_vec_unchecked](CString::from_vec_unchecked) or
    /// [from_bytes_with_nul_unchecked](CStr::from_bytes_with_nul_unchecked) respectively.
    pub fn show_console_msg<'a>(&self, msg: impl Into<ReaperStringArg<'a>>) {
        self.medium().show_console_msg(msg);
    }

    // type 0=OK,1=OKCANCEL,2=ABORTRETRYIGNORE,3=YESNOCANCEL,4=YESNO,5=RETRYCANCEL : ret
    // 1=OK,2=CANCEL,3=ABORT,4=RETRY,5=IGNORE,6=YES,7=NO
    pub fn show_message_box(
        &self,
        msg: &CStr,
        title: &CStr,
        kind: MessageBoxType,
    ) -> MessageBoxResult {
        self.medium().show_message_box(msg, title, kind)
    }

    pub fn get_main_section(&self) -> Section {
        Section::new(SectionId::new(0))
    }

    pub fn create_empty_project_in_new_tab(&self) -> Project {
        self.get_main_section()
            .get_action_by_command_id(CommandId::new(41929))
            .invoke_as_trigger(None);
        self.get_current_project()
    }

    pub fn project_switched(&self) -> impl LocalObservable<'static, Err = (), Item = Project> {
        self.subjects.project_switched.borrow().clone()
    }

    pub fn fx_opened(&self) -> impl LocalObservable<'static, Err = (), Item = Fx> {
        self.subjects.fx_opened.borrow().clone()
    }

    pub fn fx_focused(
        &self,
    ) -> impl LocalObservable<'static, Err = (), Item = Payload<Option<Fx>>> {
        self.subjects.fx_focused.borrow().clone()
    }

    pub fn track_added(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_added.borrow().clone()
    }

    pub fn midi_message_received(
        &self,
    ) -> impl LocalObservable<'static, Err = (), Item = MidiEvent<'static>> {
        self.subjects.midi_message_received.borrow().clone()
    }

    // Delivers a GUID-based track (to still be able to identify it even it is deleted)
    pub fn track_removed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_removed.borrow().clone()
    }

    pub fn track_name_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_name_changed.borrow().clone()
    }

    pub fn master_tempo_changed(&self) -> impl LocalObservable<'static, Err = (), Item = ()> {
        self.subjects.master_tempo_changed.borrow().clone()
    }

    pub fn fx_added(&self) -> impl LocalObservable<'static, Err = (), Item = Fx> {
        self.subjects.fx_added.borrow().clone()
    }

    // Attention: Returns normal fx only, not input fx!
    // This is not reliable! After REAPER start no focused Fx can be found!
    pub fn get_focused_fx(&self) -> Option<Fx> {
        self.medium().get_focused_fx().and_then(|res| {
            use GetFocusedFxResult::*;
            match res {
                ItemFx { .. } => None, // TODO-low implement
                TrackFx { track_ref, fx_ref } => {
                    // We don't know the project so we must check each project
                    self.get_projects()
                        .filter_map(|p| {
                            let track = p.get_track_by_ref(track_ref)?;
                            let fx = track.get_fx_by_query_index(fx_ref.into())?;
                            if fx.window_has_focus() {
                                Some(fx)
                            } else {
                                None
                            }
                        })
                        .next()
                }
            }
        })
    }

    pub fn fx_enabled_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Fx> {
        self.subjects.fx_enabled_changed.borrow().clone()
    }

    pub fn fx_reordered(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.fx_reordered.borrow().clone()
    }

    pub fn fx_removed(&self) -> impl LocalObservable<'static, Err = (), Item = Fx> {
        self.subjects.fx_removed.borrow().clone()
    }

    pub fn fx_parameter_value_changed(
        &self,
    ) -> impl LocalObservable<'static, Err = (), Item = FxParameter> {
        self.subjects.fx_parameter_value_changed.borrow().clone()
    }

    pub fn track_input_monitoring_changed(
        &self,
    ) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects
            .track_input_monitoring_changed
            .borrow()
            .clone()
    }

    pub fn track_input_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_input_changed.borrow().clone()
    }

    pub fn track_volume_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_volume_changed.borrow().clone()
    }

    pub fn track_pan_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_pan_changed.borrow().clone()
    }

    pub fn track_selected_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_selected_changed.borrow().clone()
    }

    pub fn track_mute_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_mute_changed.borrow().clone()
    }

    pub fn track_solo_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_solo_changed.borrow().clone()
    }

    pub fn track_arm_changed(&self) -> impl LocalObservable<'static, Err = (), Item = Track> {
        self.subjects.track_arm_changed.borrow().clone()
    }

    pub fn track_send_volume_changed(
        &self,
    ) -> impl LocalObservable<'static, Err = (), Item = TrackSend> {
        self.subjects.track_send_volume_changed.borrow().clone()
    }

    pub fn track_send_pan_changed(
        &self,
    ) -> impl LocalObservable<'static, Err = (), Item = TrackSend> {
        self.subjects.track_send_pan_changed.borrow().clone()
    }

    pub fn action_invoked(
        &self,
    ) -> impl LocalObservable<'static, Err = (), Item = Payload<Rc<Action>>> {
        self.subjects.action_invoked.borrow().clone()
    }

    pub fn get_current_project(&self) -> Project {
        Project::new(
            self.medium()
                .enum_projects(ProjectRef::Current, 0)
                .unwrap()
                .project,
        )
    }

    pub fn get_main_window(&self) -> Hwnd {
        self.medium().get_main_hwnd()
    }

    pub fn get_projects(&self) -> impl Iterator<Item = Project> + '_ {
        (0..)
            .map(move |i| self.medium().enum_projects(ProjectRef::Tab(i), 0))
            .take_while(|r| !r.is_none())
            .map(|r| Project::new(r.unwrap().project))
    }

    pub fn get_project_count(&self) -> u32 {
        self.get_projects().count() as u32
    }

    pub fn clear_console(&self) {
        self.medium().clear_console();
    }

    pub fn execute_later_in_main_thread(
        &self,
        waiting_time: Duration,
        task: impl FnOnce() + 'static,
    ) {
        self.task_sender
            .send(ScheduledTask::new(
                Box::new(task),
                Some(SystemTime::now() + waiting_time),
            ))
            .unwrap();
    }

    pub fn execute_later_in_main_thread_asap(&self, task: impl FnOnce() + 'static) {
        self.task_sender
            .send(ScheduledTask::new(Box::new(task), None))
            .unwrap();
    }

    pub fn execute_when_in_main_thread(&self, task: impl FnOnce() + 'static) {
        if self.current_thread_is_main_thread() {
            task();
        } else {
            self.execute_later_in_main_thread_asap(task);
        }
    }

    pub fn stuff_midi_message(&self, target: StuffMidiMessageTarget, message: impl ShortMessage) {
        self.medium().stuff_midimessage(target, message);
    }

    pub fn current_thread_is_main_thread(&self) -> bool {
        thread::current().id() == self.main_thread_id
    }

    pub fn get_main_thread_id(&self) -> ThreadId {
        self.main_thread_id
    }

    pub fn get_global_automation_override(&self) -> Option<GlobalAutomationOverride> {
        self.medium().get_global_automation_override()
    }

    pub fn undoable_action_is_running(&self) -> bool {
        self.undo_block_is_active.get()
    }

    // Doesn't start a new block if we already are in an undo block.
    #[must_use = "Return value determines the scope of the undo block (RAII)"]
    pub(super) fn enter_undo_block_internal<'a>(
        &self,
        project: Project,
        label: &'a CStr,
    ) -> Option<UndoBlock<'a>> {
        if self.undo_block_is_active.get() {
            return None;
        }
        self.undo_block_is_active.replace(true);
        self.medium().undo_begin_block_2(Proj(project.get_raw()));
        Some(UndoBlock::new(project, label))
    }

    // Doesn't attempt to end a block if we are not in an undo block.
    pub(super) fn leave_undo_block_internal(&self, project: &Project, label: &CStr) {
        if !self.undo_block_is_active.get() {
            return;
        }
        self.medium()
            .undo_end_block_2(Proj(project.get_raw()), label, All);
        self.undo_block_is_active.replace(false);
    }
}

struct Command {
    /// Reasoning for that type (from inner to outer):
    /// - `FnMut`: We don't use just `fn` because we want to support closures. We don't use just
    ///   `Fn` because we want to support closures that keep mutable references to their captures.
    ///   We can't accept `FnOnce` because that would mean that the closure value itself is
    ///   consumed when it's called. That means we would have to remove the action from the action
    ///   list just to call it and we couldn't again it again.
    /// - `Box`: Of course we want to support very different closures with very different captures.
    ///   We don't use generic type parameters to achieve that because we need to put Commands into
    ///   a HashMap as values - so we need each Command to have the same size in memory and the
    ///   same type. Generics lead to the generation of different types and most likely also
    ///   different sizes. We don't use references because we want ownership. Yes, Box is (like
    ///   reference) a so-called trait object and therefore uses dynamic dispatch. It also needs
    ///   heap allocation (unlike general references). However, this is exactly what we want and
    ///   need here.
    /// - `RefCell`: We need this in order to make the FnMut callable in immutable context (for
    ///   safety reasons we are mostly in immutable context, see ControlSurface documentation).
    ///   It's good to use `RefCell` in a very fine-grained way like that and not for example on
    ///   the whole `Command`. That allows for very localized mutation and therefore a lower
    ///   likelihood that borrowing rules are violated (or if we wouldn't have the runtime borrow
    ///   checking of `RefCell`, the likeliness to get undefined behavior).
    /// - `Rc`: We don't want to keep an immutable reference to the surrounding `Command` around
    ///   just in order to execute this operation! Why? Because we want to support operations which
    ///   add a REAPER action when executed. And when doing that, we of course have to borrow the
    ///   command HashMap mutably. However, at that point we already have an immutable borrow to
    ///   the complete HashMap (via a `RefCell`) ... boom. Panic! With the `Rc` we can release the
    ///   borrow by cloning the first `Rc` instance and therefore gaining a short-term second
    ///   ownership of that operation.
    /// - Wait ... actually there's no `Box` anymore! Turned out that `Rc` makes all things
    ///   possible that also `Box` makes possible, in particular taking dynamically-sized types. If
    ///   we wouldn't need `Rc` (for shared references), we would have to take `Box` instead.
    operation: Rc<RefCell<dyn FnMut()>>,
    kind: ActionKind,
}

impl Command {
    fn new(operation: Rc<RefCell<dyn FnMut()>>, kind: ActionKind) -> Command {
        Command { operation, kind }
    }
}

pub struct RegisteredAction {
    // For identifying the registered command (= the functions to be executed)
    command_id: CommandId,
    // For identifying the registered action (= description, related keyboard shortcuts etc.)
    gaccel_handle: GaccelRegister,
}

impl RegisteredAction {
    fn new(command_id: CommandId, gaccel_handle: GaccelRegister) -> RegisteredAction {
        RegisteredAction {
            command_id,
            gaccel_handle,
        }
    }

    pub fn unregister(&self) {
        Reaper::get().unregister_command(self.command_id, self.gaccel_handle);
    }
}
