use crate::{consts::CAN_ENTER, error::FanError, lock_inner};

#[get("/enter")]
pub fn get_enter() -> Result<String, FanError> {
    let can_enter = lock_inner!(CAN_ENTER)?;

    Ok(can_enter.to_string())
}

#[patch("/enter/<state>")]
pub fn set_enter(state: bool) -> Result<String, FanError> {
    let mut can_enter = lock_inner!(CAN_ENTER)?;
    *can_enter = state;

    Ok(state.to_string())
}
