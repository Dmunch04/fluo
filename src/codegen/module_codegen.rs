use crate::logger::logger::Error;
use crate::typecheck::typecheck;

use inkwell::module;

/// Module object
///
/// There is one module object per file (yes, each file has its own codegen, parser, lexer object).
pub struct CodeGenModule<'a> {
    pub module: module::Module<'a>,
    pub typecheck: typecheck::TypeCheckModule<'a>,
}

impl<'a> CodeGenModule<'a> {
    /// Return new module object.
    ///
    /// Arguments
    /// * `filename` - the filename of the file to read
    pub fn new<'b>(
        module: module::Module<'b>,
        filename: &'b str,
        file_contents: &'b str,
    ) -> CodeGenModule<'b> {
        let typecheck = typecheck::TypeCheckModule::new(filename, file_contents);
        CodeGenModule { module, typecheck }
    }

    pub fn generate(&mut self) -> Result<(), Vec<Error<'a>>> {
        self.typecheck.type_check()?;
        Ok(())
    }
}
