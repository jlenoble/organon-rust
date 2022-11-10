use super::actionable_todo::IsActionableTodo;

/// Persisted in file or in database, before decoding
///
/// It is not (yet?) a smart object (if direct casting is even possible) but likely to be congruent
/// with a string or a buffer. Therefore, it does not derive from [HasContext] nor [HasText] nor [HasId].
/// It must be parsed to an actionable todo explicitly.
pub trait IsEncodedTodo: HasParse {}

/// Parsing API for encoded todo
pub trait HasParse {
    type Todo: IsActionableTodo;

    fn parse(&self) -> Self::Todo;
}