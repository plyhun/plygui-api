use std::rc::{Rc, Weak};
use std::borrow::Cow;
use std::cell::UnsafeCell;
use super::development::ApplicationInner;
use super::{types, ids, controls};

static mut READY: bool = false;

thread_local! {
    pub static APPLICATION: Weak<UnsafeCell<dyn ApplicationInner>> = {
        let dummy: Rc<UnsafeCell<dyn ApplicationInner>> = Rc::new(UnsafeCell::new(Dummy));
        Rc::downgrade(&dummy)
    };
}

pub fn try_init(app: Rc<UnsafeCell<dyn ApplicationInner>>) -> Rc<UnsafeCell<dyn ApplicationInner>> {
    if unsafe { READY } {
        APPLICATION.with(|a| a.clone()).clone().upgrade().unwrap()
    } else {
        // TODO here may come the race!
        APPLICATION.with(|a| unsafe {
                let a = a as &_ as *const Weak<UnsafeCell<dyn ApplicationInner>> as *mut Weak<UnsafeCell<dyn ApplicationInner>>;
                let mut b = Rc::downgrade(&app.clone());
                ::std::mem::swap(&mut *a, &mut b);
        });
        unsafe { READY = true; }
        app
    }
}

struct Dummy;

impl ApplicationInner for Dummy {
    fn new_window(&mut self, _: &str, _: types::WindowStartSize, _: types::WindowMenu) -> Box<dyn controls::Window> {
        unreachable!()
    }
    fn name(&self) -> Cow<'_, str> {
        unreachable!()
    }
    fn start(&mut self) {
        unreachable!()
    }
    fn find_member_by_id_mut(&mut self, _: ids::Id) -> Option<&mut dyn controls::Member> {
        unreachable!()
    }
    fn find_member_by_id(&self, _: ids::Id) -> Option<&dyn controls::Member> {
        unreachable!()
    }
}