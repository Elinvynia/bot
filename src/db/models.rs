use crate::db::schema::log_channels;
#[derive(Insertable, Queryable)]
#[table_name = "log_channels"]
pub struct LogChannel {
    pub id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub log_type: i64,
}
