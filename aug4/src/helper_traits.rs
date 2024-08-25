pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait DynEq: AsAny {
    fn dyn_eq(&self, other: &dyn DynEq) -> bool;
}

impl<T: AsAny + PartialEq + 'static> DynEq for T {
    fn dyn_eq(&self, other: &dyn DynEq) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}

#[macro_export]
macro_rules! impl_dyn_eq_for_trait {
    ($trait:ident) => {
        impl $crate::helper_traits::DynEq for dyn $trait {
            fn dyn_eq(&self, other: &dyn $crate::helper_traits::DynEq) -> bool {
                if let Some(other) = other.as_any().downcast_ref::<Self>() {
                    self == other
                } else {
                    false
                }
            }
        }
    };
}