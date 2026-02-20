use std::{mem::zeroed, ptr::null_mut, thread};

use windows_sys::Win32::{
    Foundation,
    Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow},
    UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowRect, IsZoomed, SW_SHOWNORMAL, SWP_ASYNCWINDOWPOS,
        SWP_FRAMECHANGED, SetWindowPos, ShowWindow,
    },
};

use crate::monitor::monitor::{Bounds, Monitor, MonitorHandler};

impl MonitorHandler for Monitor {
    fn get_active_monitor() -> Monitor {
        let monitor_info = unsafe {
            let active_window = GetForegroundWindow();
            let active_monitor = MonitorFromWindow(active_window, MONITOR_DEFAULTTONEAREST);

            let mut monitor_info: MONITORINFO = zeroed();
            monitor_info.cbSize = size_of::<MONITORINFO>() as u32;

            GetMonitorInfoW(active_monitor, &mut monitor_info);
            monitor_info
        };

        let monitor_work_area = monitor_info.rcWork;
        Monitor {
            bounds: Bounds {
                top: monitor_work_area.top,
                right: monitor_work_area.right,
                bottom: monitor_work_area.bottom,
                left: monitor_work_area.left,
            },
        }
    }

    fn set_position(&self, bounds: &Bounds) {
        let bounds = bounds.clone();

        unsafe {
            let active_window = GetForegroundWindow();

            let mut window_rect: Foundation::RECT = zeroed();
            GetWindowRect(active_window, &mut window_rect);

            let monitor_handle = MonitorFromWindow(active_window, MONITOR_DEFAULTTONEAREST);
            let mut monitor_info: MONITORINFO = zeroed();
            monitor_info.cbSize = size_of::<MONITORINFO>() as u32;
            GetMonitorInfoW(monitor_handle, &mut monitor_info);

            let monitor_bounds = monitor_info.rcMonitor;

            let is_fullscreen = monitor_bounds.left == window_rect.left
                && monitor_bounds.top == window_rect.top
                && monitor_bounds.right == window_rect.right
                && monitor_bounds.bottom == window_rect.bottom;
            if is_fullscreen {
                return;
            }

            let active_window = active_window as isize;
            thread::spawn(move || {
                let active_window = active_window as *mut std::ffi::c_void;
                if IsZoomed(active_window) == 1 {
                    ShowWindow(active_window, SW_SHOWNORMAL);
                }

                SetWindowPos(
                    active_window,
                    null_mut(),
                    bounds.left,
                    bounds.top,
                    bounds.right,
                    bounds.bottom,
                    SWP_FRAMECHANGED,
                );
            });
        }
    }
}
