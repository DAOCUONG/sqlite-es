use cqrs_es::persist::PersistedEventStore;
use cqrs_es::{Aggregate, CqrsFramework, Query};

use crate::{SqliteCqrs, SqliteEventRepository};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

/// A convenience method for building a simple connection pool for PostgresDb.
/// A connection pool is needed for both the event and view repositories.
///
/// ```
/// use sqlx::{Pool, Postgres};
/// use postgres_es::default_postgress_pool;
///
/// # async fn configure_pool() {
/// let connection_string = "sqlite://aum-es.db";
/// let pool: Pool<Sqlite> = default_postgress_pool(connection_string).await;
/// # }
/// ```
pub async fn default_sqlites_pool(connection_string: &str) -> Pool<Sqlite> {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect(connection_string)
        .await
        .expect("unable to connect to database")
}

/// A convenience function for creating a CqrsFramework from a database connection pool
/// and queries.
pub fn sqlite_cqrs<A>(
    pool: Pool<Sqlite>,
    query_processor: Vec<Box<dyn Query<A>>>,
    services: A::Services,
) -> SqliteCqrs<A>
where
    A: Aggregate,
{
    let repo = SqliteEventRepository::new(pool);
    let store = PersistedEventStore::new_event_store(repo);
    CqrsFramework::new(store, query_processor, services)
}

/// A convenience function for creating a CqrsFramework using a snapshot store.
pub fn postgres_snapshot_cqrs<A>(
    pool: Pool<Sqlite>,
    query_processor: Vec<Box<dyn Query<A>>>,
    snapshot_size: usize,
    services: A::Services,
) -> SqliteCqrs<A>
where
    A: Aggregate,
{
    let repo = SqliteEventRepository::new(pool);
    let store = PersistedEventStore::new_snapshot_store(repo, snapshot_size);
    CqrsFramework::new(store, query_processor, services)
}

/// A convenience function for creating a CqrsFramework using an aggregate store.
pub fn postgres_aggregate_cqrs<A>(
    pool: Pool<Sqlite>,
    query_processor: Vec<Box<dyn Query<A>>>,
    services: A::Services,
) -> SqliteCqrs<A>
where
    A: Aggregate,
{
    let repo =SqliteEventRepository::new(pool);
    let store = PersistedEventStore::new_aggregate_store(repo);
    CqrsFramework::new(store, query_processor, services)
}

#[cfg(test)]
mod test {
    use crate::testing::tests::{
        TestAggregate, TestQueryRepository, TestServices, TestView, TEST_CONNECTION_STRING,
    };
    use crate::{default_sqlites_pool, sqlite_cqrs, SqliteViewRepository};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_valid_cqrs_framework() {
        let pool = default_sqlites_pool(TEST_CONNECTION_STRING).await;
        let repo =
            SqliteViewRepository::<TestView, TestAggregate>::new("test_view", pool.clone());
        let query = TestQueryRepository::new(Arc::new(repo));
        let _ps = sqlite_cqrs(pool, vec![Box::new(query)], TestServices);
    }
}
