pub(crate) fn get_connection() -> sqlite::Connection{
    sqlite::open("sqlite/database.db").unwrap()
}