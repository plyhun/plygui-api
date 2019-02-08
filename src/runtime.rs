use super::development::ApplicationInner;
use std::cell::{RefCell, UnsafeCell};
use std::rc::Rc;

static mut READY: bool = false;

thread_local! {
    pub static APPLICATION: RefCell<usize> = RefCell::new(0);
}

pub fn get<T: ApplicationInner>() -> Option<Rc<UnsafeCell<T>>> {
    if unsafe { READY } {
        APPLICATION.with(|a| {
            let a = *a.borrow() as *const UnsafeCell<T>;
            if a.is_null() {
                unreachable!()
            } else {
                Some(unsafe { Rc::from_raw(a) })
            }
        })
    } else {
        None
    }
}

pub fn init<T: ApplicationInner>(app: Rc<UnsafeCell<T>>) {
    if unsafe { READY } {
        panic!("Trying to access Application from a non-UI thread!") // TODO perhaps allow this if windows run in an own thread each
    } else {
        // TODO here may come the race!
        APPLICATION.with(|a| {
            *a.borrow_mut() = Rc::into_raw(app) as *const _ as usize;
        });
        unsafe {
            READY = true;
        }
    }
}
