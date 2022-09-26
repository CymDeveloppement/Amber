use heraclitus_compiler::prelude::*;
use crate::translate::module::TranslateModule;
use crate::utils::metadata::{ParserMetadata, TranslateMetadata};

#[derive(Debug, Clone)]
pub struct Continue;

impl SyntaxModule<ParserMetadata> for Continue {
    syntax_name!("Continue");

    fn new() -> Self {
        Continue
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let tok = meta.get_current_token();
        token(meta, "continue")?;
        // Detect if the break statement is inside a loop
        if !meta.loop_ctx {
            return error!(meta, tok, "Continue statement can only be used inside a loop")
        }
        Ok(())
    }
}

impl TranslateModule for Continue {
    fn translate(&self, _meta: &mut TranslateMetadata) -> String {
        "continue".to_string()
    }
}
