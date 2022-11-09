//! Todos
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