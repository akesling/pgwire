use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use super::portal::Portal;
use super::stmt::StoredStatement;

pub trait PortalStore {
    type Statement;

    fn put_statement(&self, name: &str, statement: StoredStatement<Self::Statement>);

    fn rm_statement(&self, name: &str);

    fn get_statement(&self, name: &str) -> Option<Arc<StoredStatement<Self::Statement>>>;

    fn put_portal(&self, name: &str, portal: Portal<Self::Statement>);

    fn rm_portal(&self, name: &str);

    fn get_portal(&self, name: &str) -> Option<Arc<Portal<Self::Statement>>>;
}

#[derive(Debug, Default)]
pub struct MemPortalStore<S> {
    statements: RwLock<BTreeMap<String, Arc<StoredStatement<S>>>>,
    portals: RwLock<BTreeMap<String, Arc<Portal<S>>>>,
}

impl<S: Clone> PortalStore for MemPortalStore<S> {
    type Statement = S;

    fn put_statement(&self, name: &str, statement: StoredStatement<Self::Statement>) {
        let mut guard = self.statements.write().unwrap();
        guard.insert(name.to_owned(), Arc::new(statement));
    }

    fn rm_statement(&self, name: &str) {
        let mut guard = self.statements.write().unwrap();
        guard.remove(name);
    }

    fn get_statement(&self, name: &str) -> Option<Arc<StoredStatement<Self::Statement>>> {
        let guard = self.statements.read().unwrap();
        guard.get(name).cloned()
    }

    fn put_portal(&self, name: &str, portal: Portal<Self::Statement>) {
        let mut guard = self.portals.write().unwrap();
        guard.insert(name.to_owned(), Arc::new(portal));
    }

    fn rm_portal(&self, name: &str) {
        let mut guard = self.portals.write().unwrap();
        guard.remove(name);
    }

    fn get_portal(&self, name: &str) -> Option<Arc<Portal<Self::Statement>>> {
        let mut guard = self.portals.read().unwrap();
        guard.get(name).cloned()
    }
}
