library;

pub enum AccessError {
    CannotReinitialize: (),
    NotOwner: (),
}