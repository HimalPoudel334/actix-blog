use actix_session::{Session, SessionInsertError};

pub async fn set_client_timezone(
    client_tz: String,
    session: Session,
) -> Result<(), SessionInsertError> {
    session.insert("client_tz", client_tz)
}
