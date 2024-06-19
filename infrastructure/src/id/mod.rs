use uuid::Uuid;
#[derive(Default)]
pub struct IdGenerator;

impl application::abstractions::IdProvider for IdGenerator {
    type Id = Uuid;

    fn generate_id(&self) -> Self::Id {
        Uuid::now_v7()
    }
}
