use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct Dialy {
    date: String,
    timing: String,
    contents: String,
    // pub for summary
    pub calorie: u32,
    is_good: bool,
}

/// Dialy Repository
pub struct DialyRepository {
    /// connection
    conn: Mutex<Connection>,
}

impl DialyRepository {
    pub fn new(path: String) -> Self {
        let conn = rusqlite::Connection::open(path)
            .expect("DialyRepository::new() failed create connection!");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dialies  (
                        date            STRING,
                        timing         STRING,
                        contents     STRING,
                        calorie         NUMBER,
                        is_good       NUMBER,
                        PRIMARY KEY (date, timing)
                    );",
            (),
        )
        .expect("DialyRepository::new() failed create table");
        DialyRepository {
            conn: Mutex::new(conn),
        }
    }

    pub fn insert(&self, record: Dialy) {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO dialies (date, timing, contents, calorie, is_good) VALUES (?1, ?2, ?3, ?4, ?5);",
            (
                record.date,
                record.timing,
                record.contents,
                record.calorie,
                if record.is_good { 1 } else { 0 },
            ),
        )
        .expect("DialyRepository::insert() failed insert");
    }

    pub fn select_all(&self) -> Vec<Dialy> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT * FROM dialies;")
            .expect("DialyRepository::select_all() failed select");
        let dialy_itr = stmt
            .query_map([], |row| {
                Ok(Dialy {
                    date: row.get(0)?,
                    timing: row.get(1)?,
                    contents: row.get(2)?,
                    calorie: row.get(3)?,
                    is_good: row.get(4)?,
                })
            })
            .expect("DialyRepository::select_all() failed parsing");
        let mut vec: Vec<Dialy> = Vec::new();
        for dialy in dialy_itr {
            vec.push(dialy.expect("DialyRepository::select_all() failed unwrap"));
        }
        vec
    }

    pub fn delete(&self, record: Dialy) {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM dialies WHERE date = ?1 AND timing = ?2;",
            (record.date, record.timing),
        )
        .expect("DialyRepository::delete() failed delete");
    }

    pub fn select(&self, date: String) -> Vec<Dialy> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT * FROM dialies WHERE date = :date;")
            .expect("DialyRepository::select() failed select");
        let dialy_itr = stmt
            .query_map(&[(":date", date.as_str())], |row| {
                Ok(Dialy {
                    date: row.get(0)?,
                    timing: row.get(1)?,
                    contents: row.get(2)?,
                    calorie: row.get(3)?,
                    is_good: row.get(4)?,
                })
            })
            .expect("DialyRepository::select_all() failed parsing");
        let mut vec: Vec<Dialy> = Vec::new();
        for dialy in dialy_itr {
            vec.push(dialy.expect("DialyRepository::select_all() failed unwrap"));
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ON_MEMORY: &str = ":memory:";

    #[test]
    fn table_should_be_able_to_delete_record() {
        let db = DialyRepository::new(ON_MEMORY.to_string());

        let record: Dialy = Dialy {
            date: ("2025-10-11".to_string()),
            timing: ("夜ごはん".to_string()),
            contents: ("あんかけ焼きそば".to_string()),
            calorie: (750),
            is_good: (true),
        };
        db.insert(record.clone());

        let dialies: Vec<Dialy> = db.select_all();
        assert_eq!(dialies[0].date, record.date);
        assert_eq!(dialies[0].timing, record.timing);
        assert_eq!(dialies[0].contents, record.contents);
        assert_eq!(dialies[0].calorie, record.calorie);
        assert_eq!(dialies[0].is_good, record.is_good);

        db.delete(record.clone());
        let dialies2 = db.select_all();
        assert_eq!(dialies2.len(), 0);
    }

    #[test]
    fn test_select() {
        let db = DialyRepository::new(ON_MEMORY.to_string());

        let record: Dialy = Dialy {
            date: ("2025-10-11".to_string()),
            timing: ("夜ごはん".to_string()),
            contents: ("あんかけ焼きそば".to_string()),
            calorie: (750),
            is_good: (true),
        };
        let record2: Dialy = Dialy {
            date: ("2025-10-13".to_string()),
            timing: ("おやつ".to_string()),
            contents: ("モンブラン".to_string()),
            calorie: (400),
            is_good: (true),
        };
        db.insert(record.clone());
        db.insert(record2.clone());

        let dialies: Vec<Dialy> = db.select("2025-10-11".to_string());
        assert_eq!(dialies.len(), 1);
        assert_eq!(dialies[0].date, record.date);
        assert_eq!(dialies[0].timing, record.timing);
        assert_eq!(dialies[0].contents, record.contents);
        assert_eq!(dialies[0].calorie, record.calorie);
        assert_eq!(dialies[0].is_good, record.is_good);
    }
}
