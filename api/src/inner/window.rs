use super::auto::{HasInner, Abstract};
use super::closeable::{Closeable, CloseableInner, ACloseable};
use super::container::AContainer;
use super::container_single::{ASingleContainer, SingleContainer, SingleContainerInner};
use super::has_label::{HasLabel, HasLabelInner};
use super::has_size::{HasSize, HasSizeInner, OnSize};
use super::has_visibility::{HasVisibility, HasVisibilityInner, OnVisibility};
use super::member::{AMember, Member};
use super::application::Application;

use crate::types;

define! {
    Window: HasSize + HasVisibility + SingleContainer + HasLabel + Closeable {
        base: {
            pub visibility: types::Visibility,
            pub on_size: Option<OnSize>,
            pub on_visibility: Option<OnVisibility>,
        },
        inner: {
            fn size(&self) -> (u16, u16);
            fn position(&self) -> (i32, i32);
        }
        constructor: {
            fn with_params<S: AsRef<str>>(app: &mut dyn Application, title: S, window_size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window>;
        }
        inner_constructor_params: {
            app: &mut dyn Application, title: &str, window_size: types::WindowStartSize, menu: types::Menu
        }
    }
}
impl<T: WindowInner> AWindow<T> {
    pub fn with_inner(inner: T) -> Self {
        AWindow {
            base: WindowBase {
                visibility: types::Visibility::Visible,
                on_size: None,
                on_visibility: None,
            },
            inner,
        }
    }
}
impl<T: WindowInner> HasVisibility for AMember<AContainer<ASingleContainer<ACloseable<AWindow<T>>>>> {
    fn visibility(&self) -> types::Visibility {
        self.inner.inner.inner.inner.base.visibility
    }
    fn set_visibility(&mut self, visibility: types::Visibility) {
        if self.inner.inner.inner.inner.inner.on_visibility_set(&mut self.base, visibility) {
            self.inner.inner.inner.inner.base.visibility = visibility;
            self.call_on_visibility::<Self>(visibility);
        }
    }
    fn on_visibility(&mut self, callback: Option<OnVisibility>) {
        self.inner.inner.inner.inner.base.on_visibility = callback;
    }

    fn as_has_visibility(&self) -> &dyn HasVisibility {
        self
    }
    #[inline]
    fn as_has_visibility_mut(&mut self) -> &mut dyn HasVisibility {
        self
    }
    #[inline]
    fn into_has_visibility(self: Box<Self>) -> Box<dyn HasVisibility> {
        self
    }
}
impl<T: WindowInner> HasSize for AMember<AContainer<ASingleContainer<ACloseable<AWindow<T>>>>> {
    #[inline]
    fn size(&self) -> (u16, u16) {
        self.inner.inner.inner.inner.inner.size()
    }
    #[inline]
    fn set_size(&mut self, width: u16, height: u16) {
        if self.inner.inner.inner.inner.inner.on_size_set(&mut self.base, (width, height)) {
            self.call_on_size::<Self>(width, height);
        }
    }
    #[inline]
    fn on_size(&mut self, callback: Option<OnSize>) {
        self.inner.inner.inner.inner.base.on_size = callback;
    }

    #[inline]
    fn as_has_size(&self) -> &dyn HasSize {
        self
    }
    #[inline]
    fn as_has_size_mut(&mut self) -> &mut dyn HasSize {
        self
    }
    #[inline]
    fn into_has_size(self: Box<Self>) -> Box<dyn HasSize> {
        self
    }
}
impl<T: WindowInner> Window for AMember<AContainer<ASingleContainer<ACloseable<AWindow<T>>>>> {
    fn as_window(&self) -> &dyn Window {
        self
    }
    fn as_window_mut(&mut self) -> &mut dyn Window {
        self
    }
    fn into_window(self: Box<Self>) -> Box<dyn Window> {
        self
    }
}
impl<II: WindowInner, T: HasInner<I = II> + Abstract + 'static> WindowInner for T {
    fn with_params<S: AsRef<str>>(app: &mut dyn Application, title: S, window_size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window> {
        <<Self as HasInner>::I as WindowInner>::with_params(app, title, window_size, menu)
    }
    fn size(&self) -> (u16, u16) {
        self.inner().size()
    }
    fn position(&self) -> (i32, i32) {
        self.inner().position()
    }
}
impl<T: WindowInner> NewWindow for AMember<AContainer<ASingleContainer<ACloseable<AWindow<T>>>>> {
    #[inline]
    fn with_params<S: AsRef<str>>(app: &mut dyn Application, title: S, window_size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window> {
        T::with_params(app, title, window_size, menu)
    }
}
impl<T: WindowInner> AMember<AContainer<ASingleContainer<ACloseable<AWindow<T>>>>> {
    pub fn call_on_size<O: Window>(&mut self, w: u16, h: u16) {
        let self2 = self as *mut _ as *mut O;
        if let Some(ref mut cb) = self.inner.inner.inner.inner.base.on_size {
            (cb.as_mut())(unsafe { &mut *self2 }, w, h);
        }
    }
    pub fn call_on_visibility<O: Window>(&mut self, v: types::Visibility) {
        let self2 = self as *mut _ as *mut O;
        if let Some(ref mut cb) = self.inner.inner.inner.inner.base.on_visibility {
            (cb.as_mut())(unsafe { &mut *self2 }, v);
        }
    }
}
