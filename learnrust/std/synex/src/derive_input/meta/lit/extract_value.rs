use syn::{ Lit, LitStr, Result };

use crate::{ Extract, ExtractValue };

impl ExtractValue<String> for Lit {
    fn extract_value(&self) -> Result<String> {
        let litstr: &LitStr = self.extract()?;
        Ok(litstr.value())
    }
}