use std::{
    mem::zeroed,
    ptr::{null, null_mut},
};

use windows_sys::Win32::{
    Graphics::Gdi::{GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow},
    UI::WindowsAndMessaging::{
        AW_BLEND, AW_SLIDE, AnimateWindow, GetForegroundWindow, IsZoomed, SW_SHOWNORMAL,
        SWP_ASYNCWINDOWPOS, SWP_DRAWFRAME, SWP_FRAMECHANGED, SWP_SHOWWINDOW, SetWindowPos,
        ShowWindow,
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
        println!("rcWork right {}", monitor_work_area.right);
        println!("rc right {}", monitor_info.rcMonitor.right);

        println!("rcWork bottom {}", monitor_work_area.bottom);
        println!("rc bottom {}", monitor_info.rcMonitor.bottom);

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
        println!("Updating screen positions={:?}", bounds);
        unsafe {
            let active_window = GetForegroundWindow();

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
                SWP_ASYNCWINDOWPOS | SWP_FRAMECHANGED,
            );
        }
    }
}
