use eframe::egui;
use std::any::Any;
pub trait Tab<Context>: Send + Sync + TabDowncast {
    fn title(&self) -> &str;
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &mut Context);
}

pub trait TabDowncast {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> TabDowncast for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<Context> dyn Tab<Context> {
    /// Returns true if the trait object wraps an object of type `T`.
    #[inline]
    pub fn is<T: Tab<Context> + 'static>(&self) -> bool {
        TabDowncast::as_any(self).is::<T>()
    }

    /// Returns a reference to the object within the trait object if it is of type `T`, or `None` if it isn't.
    #[inline]
    pub fn downcast_ref<T: Tab<Context> + 'static>(&self) -> Option<&T> {
        TabDowncast::as_any(self).downcast_ref::<T>()
    }

    /// Returns a mutable reference to the object within the trait object if it is of type `T`, or `None` if it isn't.
    #[inline]
    pub fn downcast_mut<T: Tab<Context> + 'static>(&mut self) -> Option<&mut T> {
        TabDowncast::as_any_mut(self).downcast_mut::<T>()
    }
}
