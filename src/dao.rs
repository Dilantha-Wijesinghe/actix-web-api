use rusqlite::Connection;

#[derive(Debug)]
pub struct Dao {
    con: Connection,
}

impl Dao {
    pub fn new() -> Result<Self, rusqlite::Error> {
        let con = Connection::open("data.db")?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL
            );",
            (),
        )?;
        Ok(Self { con })
    }

    pub fn create(&self, title: &str) -> Result<(), rusqlite::Error> {
        self.con
            .execute("INSERT INTO todo (title) VALUES (?1)", [title])?;
        Ok(())
    }

    pub fn read(&self) -> Result<Vec<(u64, String)>, rusqlite::Error> {
        let mut stmt = self.con.prepare("SELECT id, title FROM todo")?;
        let rows = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .map(|x| x.unwrap())
            .collect();
        Ok(rows)
    }

    pub fn remove(&self, id: u64) -> Result<(), rusqlite::Error> {
        self.con.execute("DELETE FROM todo WHERE id = (?1)", [id])?;
        Ok(())
    }
}
