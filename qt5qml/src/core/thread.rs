pub struct QThread(pub(crate) crate::ffi::QTimer);

impl_qobject_ref!(QThread);
