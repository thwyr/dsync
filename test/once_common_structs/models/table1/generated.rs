/* @generated and managed by dsync */

use crate::diesel::*;
use crate::schema::*;
use diesel::QueryResult;
use crate::models::common::*;

pub type ConnectionType = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

/// Struct representing a row in table `table1`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Queryable, Selectable, QueryableByName)]
#[diesel(table_name=table1, primary_key(id))]
pub struct Table1 {
    /// Field representing column `id`
    pub id: i32,
}

impl Table1 {
    /// Insert a new row into `table1` with all default values
    pub fn create(db: &mut ConnectionType) -> QueryResult<Self> {
        use crate::schema::table1::dsl::*;

        insert_into(table1).default_values().get_result::<Self>(db)
    }

    /// Get a row from `table1`, identified by the primary key
    pub fn read(db: &mut ConnectionType, param_id: i32) -> QueryResult<Self> {
        use crate::schema::table1::dsl::*;

        table1.filter(id.eq(param_id)).first::<Self>(db)
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub fn paginate(db: &mut ConnectionType, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::table1::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = table1.count().get_result(db)?;
        let items = table1.limit(page_size).offset(page * page_size).load::<Self>(db)?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    /// Delete a row in `table1`, identified by the primary key
    pub fn delete(db: &mut ConnectionType, param_id: i32) -> QueryResult<usize> {
        use crate::schema::table1::dsl::*;

        diesel::delete(table1.filter(id.eq(param_id))).execute(db)
    }
}
