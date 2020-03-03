use std::cell::UnsafeCell;

struct Global<T: 'static>(UnsafeCell<T>);

impl<T: 'static> Global<T> {
    #[inline]
    pub const fn new(value: T) -> Self {
        Global(UnsafeCell::new(value))
    }
    #[inline]
    pub fn get(&self) -> &'static T {
        unsafe { &*self.0.get() }
    }
    #[inline]
    pub fn get_mut(&self) -> &'static mut T {
        unsafe { &mut *self.0.get() }
    }
}

unsafe impl<T: 'static> Sync for Global<T> {}

impl<T: 'static> From<T> for Global<T> {
    fn from(value: T) -> Self {
        Global::new(value)
    }
}

static VALUE: Global<u32> = Global::new(0);

fn main() {
    println!("{}", VALUE.get());
    *VALUE.get_mut() = 1;
    println!("{}", VALUE.get());
    std::thread::spawn(move || {
        *VALUE.get_mut() = 2;
        println!("{}", VALUE.get());
        *VALUE.get_mut() = 3;
    })
    .join()
    .unwrap();
    println!("{}", VALUE.get());
}
