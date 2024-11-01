use sqlx::sqlite::SqlitePool;
use sqlx::Result;
use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct RngRecordEntry {
    pub id: i64,
    pub created_at: Option<NaiveDateTime>,
    pub roll_result: i64,
}

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    // data model tbc
    pub async fn init(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE rng (
                    id INTEGER PRIMARY KEY,
                    roll_result INTEGER NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn insert_rng(&self, rng_result: i64) -> Result<()> {
        sqlx::query!(
            "INSERT INTO rng (roll_result) VALUES (?);",
            rng_result,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_rng(&self, id: i64, limit: i32) -> Result<Option<Vec<RngRecordEntry>>> {
        let rows = sqlx::query!(
            "SELECT * FROM rng WHERE id = ? ORDER BY created_at DESC LIMIT ?;",
            id,
            limit,
        )
        .fetch_all(&self.pool)
        .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let mut rng_entries = Vec::new();
        for row in rows {
            rng_entries.push(RngRecordEntry {
                id: row.id,
                created_at: row.created_at,
                roll_result: row.roll_result,
            });
        }

        Ok(Some(rng_entries))
    }
}

