use std::collections::HashSet;

use crate::qobject::QObjectConfig;
use crate::{Include, QObjectMethod, QObjectProp, QObjectSignal};

pub trait Dependent {
    fn dependencies(&self, includes: &mut HashSet<Include>);
}

impl Dependent for QObjectProp {
    fn dependencies(&self, includes: &mut HashSet<Include>) {
        if let Some(include) = self.type_ref.include() {
            includes.insert(include.clone());
        }
    }
}

impl Dependent for QObjectMethod {
    fn dependencies(&self, includes: &mut HashSet<Include>) {
        if let Some(rtype) = &self.rtype {
            if let Some(include) = rtype.include() {
                includes.insert(include.clone());
            }
        }
        includes.extend(self.args.iter().flat_map(|(_, ty)| ty.include().clone()));
    }
}

impl Dependent for QObjectSignal {
    fn dependencies(&self, includes: &mut HashSet<Include>) {
        includes.extend(self.args.iter().flat_map(|(_, ty)| ty.include().clone()));
    }
}

impl Dependent for QObjectConfig {
    fn dependencies(&self, includes: &mut HashSet<Include>) {
        self.properties
            .iter()
            .for_each(|p| p.dependencies(includes));
        self.methods.iter().for_each(|p| p.dependencies(includes));
        self.signals.iter().for_each(|p| p.dependencies(includes));
        if let Some(include) = self.base_class.include() {
            includes.insert(include.clone());
        }
        includes.insert(Include::System("utility".into())); // required for std::forward
        if self.qml {
            includes.insert(Include::System("QtQml".into()));
        }
    }
}
