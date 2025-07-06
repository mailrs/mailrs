pub enum DatabaseMode {
    ReadOnly,
    ReadWrite,
}

impl From<DatabaseMode> for notmuch::DatabaseMode {
    fn from(value: DatabaseMode) -> Self {
        match value {
            DatabaseMode::ReadOnly => notmuch::DatabaseMode::ReadOnly,
            DatabaseMode::ReadWrite => notmuch::DatabaseMode::ReadWrite,
        }
    }
}
