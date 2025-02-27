use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;

#[derive(Clone)]
pub struct PoolWrapper(pub Pool<AsyncPgConnection>);
