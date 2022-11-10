/// Todo trait
pub trait IsTodo: HasContext + HasText {}

/// Todo context
///
/// It is meaningful only within the context of this module. Other modules may define other context traits
pub trait IsContext {}

/// Context accessor
pub trait HasContext {
    type Context: IsContext;

    fn context(&self) -> Option<&Self::Context>;
}

/// Todo text
///
/// It is meaningful only within the context of this module. Other modules may define other text traits
pub trait IsText {}

/// Text accessor
pub trait HasText {
    type Text: IsText;

    fn text(&self) -> &Self::Text;
}