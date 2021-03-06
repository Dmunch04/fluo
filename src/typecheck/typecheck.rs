use crate::logger::logger::Error;
use crate::parser::parser;
use crate::parser::parser::Parser;
use crate::typecheck::ast_typecheck;

/// Typecheck object
pub struct TypeCheckModule<'a> {
    pub parser: Parser<'a>,
    pub symtab: ast_typecheck::TypeCheckSymbTab<'a>,
}

impl<'a> TypeCheckModule<'a> {
    /// Return new module object.
    ///
    /// Arguments
    /// * `filename` - the filename of the file to read
    pub fn new(filename: &'a str, file_contents: &'a str) -> TypeCheckModule<'a> {
        let mut p = parser::Parser::new(filename, file_contents);
        p.initialize_expr();
        TypeCheckModule {
            parser: p,
            symtab: ast_typecheck::TypeCheckSymbTab::new(),
        }
    }

    pub fn type_check(&mut self) -> Result<(), Vec<Error<'a>>> {
        self.parser.parse()?;

        let mut errors = Vec::new();

        // Do type checking
        for node in &self.parser.ast.as_ref().unwrap().nodes {
            match (node as &dyn ast_typecheck::TypeCheck).type_check(None, &mut self.symtab) {
                Ok(_) => {}
                Err(e) => {
                    errors.append(&mut e.as_vec());
                }
            }
        }

        if !errors.is_empty() {
            // The different errors to have different priorities
            // We want to show the errors with the highest priority
            // Show all of the errors that have the the same priority
            let max_error_priority = errors.iter().max_by_key(|x| x.1 as usize).unwrap().1 as usize;
            let errors_priority = errors
                .into_iter()
                .filter(|error| error.1 as usize == max_error_priority)
                .map(|error| error.0)
                .collect();
            Err(errors_priority)
        } else {
            Ok(())
        }
    }
}
