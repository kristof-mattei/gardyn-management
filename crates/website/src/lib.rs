pub mod router;
pub mod routes;
pub mod server;
pub mod state;
pub mod states;
pub mod tasks;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let left = || true;
        let right = || false;

        assert_ne!(left(), right());
    }
}
