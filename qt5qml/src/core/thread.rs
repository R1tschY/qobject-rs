#[repr(C)]
pub struct QThread(pub(crate) crate::ffi::QThread);
impl_ffi_trait!(QThread);
impl_qobject_ref!(QThread);
