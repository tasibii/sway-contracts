library;

pub enum ERC721Error {
    NotOwner: (),
    NotMintYet: (),
    Unauthorized: (),
    AlreadyMinted: (),
    NotInitialized: (),
    InvalidOperator: (),
    CannotReinitialize: (),
}
