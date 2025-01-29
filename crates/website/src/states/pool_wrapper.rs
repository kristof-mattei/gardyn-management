use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

#[derive(Clone)]
pub struct PoolWrapper(pub Pool<AsyncPgConnection>);
