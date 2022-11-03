use syn::{ Error, Lit, LitStr, Result };

use crate::{ Extract, ExtractValue };

impl Extract<LitStr> for Lit {
    fn extract(&self) -> Result<&LitStr> {
        match self {
            Lit::Str(litstr) => { Ok(litstr) }
            _ => { Err(Error::new_spanned(self, "expected LitStr as Lit, got another Lit type")) }
        }
    }
}

impl ExtractValue<String> for Lit {
    fn extract_value(&self) -> Result<String> {
        let litstr: &LitStr = self.extract()?;
        Ok(litstr.value())
    }
}