use std::io::{Result, Write};

use lang_c::ast::*;
use lang_c::span::Node;

use crate::write_base::*;

impl<T: WriteLine> WriteLine for Node<T> {
    fn write_line(&self, indent: usize, write: &mut dyn Write) -> Result<()> {
        self.node.write_line(indent, write)
    }
}

impl<T: WriteString> WriteString for Node<T> {
    fn write_string(&self) -> String {
        self.node.write_string()
    }
}

// C AST write line
impl WriteLine for TranslationUnit {
    /// VERY BIG HINT: You should start by understanding the [`writeln!`](https://doc.rust-lang.org/std/macro.writeln.html) macro.
    fn write_line(&self, indent: usize, write: &mut dyn Write) -> Result<()> {
        for declaration in &self.0 {
            writeln!(write, "{}", declaration.write_string())?;
        }

        Ok(())
    }
}

// C AST write string
impl WriteString for Initializer {
    fn write_string(&self) -> String {
        todo!()
    }
}

impl WriteString for ExternalDeclaration {
    fn write_string(&self) -> String {
        match self {
            Self::Declaration(decl) => panic!("Declaration"),
            Self::StaticAssert(decl) => panic!("StaticAssert"),
            Self::FunctionDefinition(decl) => decl.write_string(),
        }
    }
}

impl WriteString for Declaration {
    fn write_string(&self) -> String {
        panic!("Declaration")
        // self.specifiers.assert_supported();
        // self.declarators.assert_supported();
    }
}

impl WriteString for FunctionDefinition {
    fn write_string(&self) -> String {
        // self.specifiers.write_string()
        // self.declarator.write_string()
        // self.declarations.iter()
        //     .map(|item| item.write_string())
        //     .collect::<Vec<_>>()
        //     .join("")
        self.statement.write_string()
    }
}

impl WriteString for Declarator {
    fn write_string(&self) -> String {
        panic!("Declarator")
        // self.kind.assert_supported();
        // self.derived.assert_supported();
        // assert!(self.extensions.is_empty());
    }
}

impl WriteString for Statement {
    fn write_string(&self) -> String {
        match self {
            Self::Labeled(_) => panic!("Statement::Labeled"),
            Self::Compound(items) => items
                .iter()
                .map(|item| item.write_string())
                .collect::<Vec<_>>()
                .join(""),
            Self::Expression(_) => panic!("Statement::Expression"),
            Self::If(_) => panic!("Statement::If"),
            Self::Switch(_) => panic!("Statement::Switch"),
            Self::While(_) => panic!("Statement::While"),
            Self::DoWhile(_) => panic!("Statement::DoWhile"),
            Self::For(_) => panic!("Statement::For"),
            Self::Goto(_) => panic!("Statement::Goto"),
            Self::Continue => panic!("Statement::Continue"),
            Self::Break => panic!("Statement::Break"),
            Self::Return(retrn) => retrn.write_string(),
            Self::Asm(_) => panic!("Statement::Asm"),
        }
    }
}

impl WriteString for BlockItem {
    fn write_string(&self) -> String {
        match self {
            Self::Declaration(decl) => panic!("BlockItem::Declaration"),
            Self::StaticAssert(_) => panic!("BlockItem::StaticAssert"),
            Self::Statement(stmt) => stmt.write_string(),
        }
    }
}

impl WriteString for Expression {
    fn write_string(&self) -> String {
        match self {
            Self::Identifier(identifier) => identifier.node.name.clone(),
            Self::Constant(cnst) => cnst.write_string(),
            Self::StringLiteral(_) => panic!("Expression::StringLiteral"),
            Self::GenericSelection(_) => panic!("Expression::GenericSelection"),
            Self::Member(_) => panic!("Expression::Member"),
            Self::Call(_) => panic!("Expression::Call"),
            Self::CompoundLiteral(_) => panic!("Expression::CompoundLiteral"),
            Self::SizeOfTy(_) => panic!("Expression::SizeOfTy"),
            Self::SizeOfVal(_) => panic!("Expression::SizeOfVal"),
            Self::AlignOf(_) => panic!("Expression::AlignOf"),
            Self::UnaryOperator(_) => panic!("Expression::UnaryOperator"),
            Self::Cast(_) => panic!("Expression::Cast"),
            Self::BinaryOperator(_) => panic!("Expression::BinaryOperator"),
            Self::Conditional(_) => panic!("Expression::Conditional"),
            Self::Comma(_) => panic!("Expression::Comma"),
            Self::OffsetOf(_) => panic!("Expression::OffsetOf"),
            Self::VaArg(_) => panic!("Expression::VaArg"),
            Self::Statement(_) => panic!("Expression::Statement"),
        }
    }
}

impl WriteString for Constant {
    fn write_string(&self) -> String {
        match self {
            Self::Integer(integer) => integer.write_string(),
            Self::Float(float) => panic!("Constant::Float"),
            Self::Character(char) => panic!("Constant::Character"),
        }
    }
}

impl WriteString for Integer {
    fn write_string(&self) -> String {
        self.number.to_string()
    }
}
