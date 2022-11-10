use crate::traits::id::HasId;
use super::{ encoded_todo::IsEncodedTodo, todo::IsTodo };

/// Bare-bone todo in RAM, after decoding
///
/// It may be encoded for persistent storage.
pub trait IsActionableTodo: IsTodo + HasId + HasEncode {}

/// Encoding API for actionable (i.e. in-memory, or "stored") todo
pub trait HasEncode {
    type Todo: IsEncodedTodo;

    fn encode(&self) -> Self::Todo;
}