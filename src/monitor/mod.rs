#[cfg(target_os = "windows")]
mod windows;

pub mod monitor {
    #[derive(Debug)]
    pub struct Bounds {
        pub top: i32,
        pub right: i32,
        pub bottom: i32,
        pub left: i32,
    }

    pub struct Monitor {
        pub bounds: Bounds,
    }

    impl Monitor {
        pub fn get_center(&self) -> (i32, i32) {
            println!("get center bounds {:?}", self.bounds);

            let x = (self.bounds.left + self.bounds.right) / 2;
            let y = (self.bounds.top + self.bounds.bottom) / 2;

            (x + 1, y + 1)
        }
    }

    pub trait MonitorHandler {
        fn get_active_monitor() -> Monitor;
        fn set_position(&self, bounds: &Bounds);
    }
}
