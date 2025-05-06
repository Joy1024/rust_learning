use std::sync::{Once, Mutex};

struct Singleton {
    data: i32,
}

impl Singleton {
    fn get_instance() -> &'static Mutex<Singleton> {
        static mut INSTANCE: *const Mutex<Singleton> = std::ptr::null();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            let singleton = Mutex::new(Singleton { data: 42 });
            unsafe {
                INSTANCE = Box::into_raw(Box::new(singleton));
            }
        });

        unsafe { &*INSTANCE }
    }
}

fn main() {
    let singleton1 = Singleton::get_instance();
    let singleton2 = Singleton::get_instance();

    assert_eq!(std::ptr::eq(singleton1, singleton2), true);
}