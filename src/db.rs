use crate::prelude::*;
use crate::problem::*;
use rusqlite::OptionalExtension;
use rusqlite::{named_params, Connection};
use std::sync::LazyLock;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Userboard {
    #[serde(rename = "Success")]
    pub success: Problems,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Problems {
    pub problems: Vec<Option<Score>>,
}

impl Userboard {
    pub fn best_score(&self, id: ProblemId) -> Option<Score> {
        assert!(id > 0);
        let index = (id - 1) as usize;
        self.success.problems[index]
    }

    pub fn new() -> Result<Userboard> {
        Ok(serde_json::from_str(&read_from("userboard.json")?)?)
    }

    pub fn total_score(&self) -> Score {
        self.success.problems.iter().flat_map(|a| a).sum()
    }

    #[allow(dead_code)]
    fn stats(&self) {
        println!("# total: {}", self.total_score());
        println!("# id score");
        for (id, score) in self.success.problems.iter().enumerate() {
            let id = (id + 1) as ProblemId;
            let score = score.unwrap_or(0.0);
            println!("{id} {score}",);
        }
    }
}

// After contests
fn create_table(conn: &Connection) -> Result<usize> {
    Ok(conn.execute(
        "CREATE TABLE IF NOT EXISTS best (
            id    INTEGER PRIMARY KEY,
            score REAL NOT NULL
        ) STRICT",
        (),
    )?)
}

fn db() -> std::sync::MutexGuard<'static, Connection> {
    static CONNECTION: LazyLock<Mutex<Connection>> = LazyLock::new(|| {
        let conn = Connection::open(project_path("db.sqlite")).expect("open?");
        create_table(&conn).expect("creata_table?");
        Mutex::new(conn)
    });
    CONNECTION.lock().unwrap()
}

pub fn score(id: ProblemId) -> Result<Option<Score>> {
    Ok(db()
        .query_one("SELECT score FROM best WHERE id = ?1", (id,), |row| {
            row.get(0)
        })
        .optional()?)
}

pub fn update_score(id: ProblemId, new_score: Score) -> Result<usize> {
    Ok(db().execute(
        "INSERT INTO best (id, score) VALUES(:id, :score)
  ON CONFLICT(id) DO UPDATE SET score = :score;",
        named_params! {
            ":id": id,
            ":score": new_score,
        },
    )?)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn read_userboard() -> Result<()> {
        let userboard = Userboard::new()?;
        // [2023-07-07 Fri 22:49] Lightning. # of Problems = 55
        // Full: Day1  # of Problems = 90
        assert_eq!(userboard.success.problems.len(), 90);
        Ok(())
    }

    #[test]
    fn db_test() -> Result<()> {
        update_score(1, 100.0)?;
        assert_eq!(score(1)?.unwrap(), 100.0);
        assert_eq!(score(10000)?, None);
        Ok(())
    }
}
