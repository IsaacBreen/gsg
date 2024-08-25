use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
//
// pub trait DynEq: AsAny {
//     fn dyn_eq(&self, other: &dyn Any) -> bool;
// }

// impl<T: AsAny + PartialEq + 'static> DynEq for T {
//     fn dyn_eq(&self, other: &dyn Any) -> bool {
//         if let Some(other) = other.downcast_ref::<T>() {
//             self == other
//         } else {
//             false
//         }
//     }
// }
//
// #[macro_export]
// macro_rules! impl_dyn_eq_for_trait {
//     ($trait:ident) => {
//         impl $crate::helper_traits::DynEq for dyn $trait {
//             fn dyn_eq(&self, other: &dyn std::any::Any) -> bool {
//                 if let Some(other) = other.downcast_ref::<Self>() {
//                     self == other
//                 } else {
//                     false
//                 }
//             }
//         }
//     };
// }