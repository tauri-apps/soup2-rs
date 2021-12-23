// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::LoggerLogLevel;
use crate::Message;
use crate::Session;
#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
use glib::StaticType;
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_56", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
use std::mem::transmute;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
    #[doc(alias = "SoupLogger")]
    pub struct Logger(Object<ffi::SoupLogger, ffi::SoupLoggerClass>) @implements SessionFeature;

    match fn {
        type_ => || ffi::soup_logger_get_type(),
    }
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
    #[doc(alias = "SoupLogger")]
    pub struct Logger(Object<ffi::SoupLogger, ffi::SoupLoggerClass>);

    match fn {
        type_ => || ffi::soup_logger_get_type(),
    }
}

impl Logger {
    #[doc(alias = "soup_logger_new")]
    pub fn new(level: LoggerLogLevel, max_body_size: i32) -> Logger {
        crate::assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_logger_new(level.into_glib(), max_body_size))
        }
    }
}

pub const NONE_LOGGER: Option<&Logger> = None;

pub trait LoggerExt: 'static {
    #[cfg(any(not(feature = "v2_24"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_24"))))]
    #[doc(alias = "soup_logger_attach")]
    fn attach(&self, session: &impl IsA<Session>);

    #[cfg(any(not(feature = "v2_24"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_24"))))]
    #[doc(alias = "soup_logger_detach")]
    fn detach(&self, session: &impl IsA<Session>);

    #[doc(alias = "soup_logger_set_request_filter")]
    fn set_request_filter<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(&self, request_filter: P);

    #[doc(alias = "soup_logger_set_response_filter")]
    fn set_response_filter<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(&self, response_filter: P);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn level(&self) -> LoggerLogLevel;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn set_level(&self, level: LoggerLogLevel);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "max-body-size")]
    fn max_body_size(&self) -> i32;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "max-body-size")]
    fn set_max_body_size(&self, max_body_size: i32);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "level")]
    fn connect_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    #[doc(alias = "max-body-size")]
    fn connect_max_body_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Logger>> LoggerExt for O {
    #[cfg(any(not(feature = "v2_24"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_24"))))]
    fn attach(&self, session: &impl IsA<Session>) {
        unsafe {
            ffi::soup_logger_attach(self.as_ref().to_glib_none().0, session.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(not(feature = "v2_24"), feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(not(feature = "v2_24"))))]
    fn detach(&self, session: &impl IsA<Session>) {
        unsafe {
            ffi::soup_logger_detach(self.as_ref().to_glib_none().0, session.as_ref().to_glib_none().0);
        }
    }

    fn set_request_filter<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(&self, request_filter: P) {
        let request_filter_data: Box_<P> = Box_::new(request_filter);
        unsafe extern "C" fn request_filter_func<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(logger: *mut ffi::SoupLogger, msg: *mut ffi::SoupMessage, user_data: glib::ffi::gpointer) -> ffi::SoupLoggerLogLevel {
            let logger = from_glib_borrow(logger);
            let msg = from_glib_borrow(msg);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&logger, &msg);
            res.into_glib()
        }
        let request_filter = Some(request_filter_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = request_filter_data;
        unsafe {
            ffi::soup_logger_set_request_filter(self.as_ref().to_glib_none().0, request_filter, Box_::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn set_response_filter<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(&self, response_filter: P) {
        let response_filter_data: Box_<P> = Box_::new(response_filter);
        unsafe extern "C" fn response_filter_func<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(logger: *mut ffi::SoupLogger, msg: *mut ffi::SoupMessage, user_data: glib::ffi::gpointer) -> ffi::SoupLoggerLogLevel {
            let logger = from_glib_borrow(logger);
            let msg = from_glib_borrow(msg);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&logger, &msg);
            res.into_glib()
        }
        let response_filter = Some(response_filter_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&Logger, &Message) -> LoggerLogLevel + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = response_filter_data;
        unsafe {
            ffi::soup_logger_set_response_filter(self.as_ref().to_glib_none().0, response_filter, Box_::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn level(&self) -> LoggerLogLevel {
        unsafe {
            let mut value = glib::Value::from_type(<LoggerLogLevel as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"level\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `level` getter")
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn set_level(&self, level: LoggerLogLevel) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"level\0".as_ptr() as *const _, level.to_value().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn max_body_size(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"max-body-size\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `max-body-size` getter")
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn set_max_body_size(&self, max_body_size: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"max-body-size\0".as_ptr() as *const _, max_body_size.to_value().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn connect_level_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_level_trampoline<P: IsA<Logger>, F: Fn(&P) + 'static>(this: *mut ffi::SoupLogger, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Logger::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::level\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_level_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_56")))]
    fn connect_max_body_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_body_size_trampoline<P: IsA<Logger>, F: Fn(&P) + 'static>(this: *mut ffi::SoupLogger, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Logger::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::max-body-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_max_body_size_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Logger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Logger")
    }
}