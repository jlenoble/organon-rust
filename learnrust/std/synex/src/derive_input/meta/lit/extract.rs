use syn::{ Error, Lit, LitStr, Result };

use crate::Extract;

impl Extract<LitStr> for Lit {
    fn extract(&self) -> Result<&LitStr> {
        match self {
            Lit::Str(litstr) => { Ok(litstr) }
            _ => { Err(Error::new_spanned(self, "expected LitStr as Lit, got another Lit type")) }
        }
    }
}