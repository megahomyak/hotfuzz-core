/// This crate is entirely internal and is designed to help me work with pointers to `Box`es easier

pub(crate) struct BoxPtr<T> {
    value: *mut T,
}

pub(crate) fn r#box<T>(value: T) -> (Box<T>, BoxPtr<T>) {
    let r#box = Box::new(value);
    let ptr = r#box.as_mut() as *mut _;
    (r#box, ptr)
}

impl<T> BoxPtr<T> {
    pub fn value(&self) -> *mut T {
        self.value
    }
}
