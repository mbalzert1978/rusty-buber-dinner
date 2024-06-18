use std::sync::Arc;

pub trait GetValues {
    type Values;
    fn get_values(&self) -> Arc<Self::Values>;
}
