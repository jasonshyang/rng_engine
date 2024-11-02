use sqlx::sqlite::SqlitePool;
use sqlx::Result;
use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct RngRecordEntry {
    pub record_id: i64,
    pub player_id: i64,
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
            "CREATE TABLE IF NOT EXISTS rng (
                    record_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    player_id INTEGER NOT NULL,
                    roll_result INTEGER NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn insert_roll(&self, player_id: i64, roll_result: i64) -> Result<()> {
        sqlx::query!(
            "INSERT INTO rng (player_id, roll_result) VALUES (?, ?);",
            player_id,
            roll_result,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_roll(&self, player_id: i64, limit: i32) -> Result<Option<Vec<RngRecordEntry>>> {
        let rows = sqlx::query!(
            "SELECT * FROM rng WHERE player_id = ? ORDER BY record_id DESC LIMIT ?;",
            player_id,
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
                record_id: row.record_id,
                player_id: row.player_id,
                created_at: row.created_at,
                roll_result: row.roll_result,
            });
        }

        Ok(Some(rng_entries))
    }
}

