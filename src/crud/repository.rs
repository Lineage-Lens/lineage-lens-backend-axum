pub trait Repository<T, E> {
    async fn find_all(&self) -> Vec<T>;
    async fn save(&self, obj: T) -> Result<T, E>;
}