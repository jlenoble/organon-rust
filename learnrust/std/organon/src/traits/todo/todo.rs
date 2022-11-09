//! Todos
//!
//! ## Intention context
//!
//! Conceptually, a todo is a bit of text, meaningful to me, but not necessarily so to anyone else
//! who lacks the proper context. Worse, I may over time forget, in part or in full, the original context
//! and subsequently may start doubting the actual meaning. Therefore it is crucial in one form or another
//! to preserve as much of the original context as possible.
//!
//! The distinction between a todo and its context cannot be clear-cut: The text of the todo itself in its very
//! phrasing bears some context. It's a matter of preference which piece of information goes where. For example,
//! "wash the dishes" is clear-cut for me at home and I need no more context, but "buy food" is not:
//! My grocery list varies, where I need to go varies. Here the context is grocery list and shopping
//! location. But a similar todo (i.e. which has the same meaning to me) could be "buy bread", which requires
//! no more specifying for me because I have a baker at the corner of my street and as any proper Frenchman,
//! I prefer traditional baguettes.
//!
//! See [IsTodo], [HasContext], [HasText].
//!
//! ## Execution context
//!
//! There is another kind of context. Being some meaningful stuff in my mind is not the same as some recorded
//! piece of information in a database, some scribbles on my screen, some bits in transit between buffers
//! and storages, some transient object passed around from function to function. All these and many more are
//! actual todos, it just depends on that other kind of context, not in the realm of intention, but in the
//! realm of program execution.
//!
//! In the simplest case, a todo just goes from my brain to my screen to oblivion. It is typed, has some
//! representation in RAM, some representation on screen, is executed in reality, then cleared from
//! screen and RAM , then forgotten.
//!
//! The representation of my intention in RAM should be bare-bone (Text and optional Context), but the representation
//! on screen (and its RAM counterpart) may be very complex, with lots of decorations, depending on a view context
//! and user interactions.
//!
//! A more typical case is persisting todos to a text file or a database and reading them back at an ulterior time.
//! There is always a unique Id involved, even if only a line number.
//!
//! See [IsActionableTodo], [HasId].

use crate::traits::id::HasId;

/// Todo trait
trait IsTodo: HasContext + HasText {}

/// Todo context
///
/// It is meaningful only within the context of this module. Other modules may define other context traits
trait IsContext {}

/// Context accessor
trait HasContext {
    type Context: IsContext;

    fn context(&self) -> Option<&Self::Context>;
}

/// Todo text
///
/// It is meaningful only within the context of this module. Other modules may define other text traits
trait IsText {}

/// Text accessor
trait HasText {
    type Text: IsText;

    fn text(&self) -> &Self::Text;
}

/// Bare-bone todo in RAM, in file or in database
trait IsActionableTodo: IsTodo + HasId {}