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
        let specifiers = self
            .specifiers
            .iter()
            .map(|item| item.write_string())
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "{} {} {}",
            specifiers,
            self.declarator.write_string(),
            self.statement.write_string()
        )
        // self.declarations.iter()
        //     .map(|item| item.write_string())
        //     .collect::<Vec<_>>()
        //     .join("")
    }
}

impl WriteString for DeclarationSpecifier {
    fn write_string(&self) -> String {
        match self {
            Self::StorageClass(_) => panic!("DeclarationSpecifier::StorageClass"),
            Self::TypeSpecifier(type_specifier) => type_specifier.write_string(),
            Self::TypeQualifier(_) => panic!("DeclarationSpecifier::TypeQualifier"),
            Self::Function(_) => panic!("DeclarationSpecifier::Function"),
            Self::Alignment(_) => panic!("DeclarationSpecifier::Alignment"),
            Self::Extension(_) => panic!("DeclarationSpecifier::Extension"),
        }
    }
}

impl WriteString for TypeSpecifier {
    fn write_string(&self) -> String {
        match self {
            Self::Void => "void".to_string(),
            Self::Char => "char".to_string(),
            Self::Short => "short".to_string(),
            Self::Int => "int".to_string(),
            Self::Long => "long".to_string(),
            Self::Float => "float".to_string(),
            Self::Double => "double".to_string(),
            Self::Signed => "signed".to_string(),
            Self::Unsigned => "unsigned".to_string(),
            Self::Bool => "_Bool".to_string(),

            Self::Complex => panic!("TypeSpecifier::Complex"),
            Self::Atomic(_) => panic!("TypeSpecifier::Atomic"),
            Self::Struct(_) => panic!("TypeSpecifier::Struct"),
            Self::Enum(_) => panic!("TypeSpecifier::Enum"),

            Self::TypedefName(name) => name.node.name.clone(),

            Self::TypeOf(_) => panic!("TypeSpecifier::TypeOf"),
            Self::TS18661Float(_) => panic!("TypeSpecifier::TS18661Float"),
        }
    }
}

impl WriteString for Declarator {
    fn write_string(&self) -> String {
        assert!(
            self.extensions.is_empty(),
            "Declarator::extensions not supported"
        );

        let mut result = self.kind.write_string();

        for derived in &self.derived {
            result.push_str(&derived.write_string());
        }

        result
    }
}

impl WriteString for DeclaratorKind {
    fn write_string(&self) -> String {
        match self {
            Self::Abstract => String::new(),
            Self::Identifier(identifier) => identifier.node.name.clone(),
            Self::Declarator(declarator) => {
                format!("({})", declarator.write_string())
            }
        }
    }
}

impl WriteString for DerivedDeclarator {
    fn write_string(&self) -> String {
        match self {
            Self::Pointer(_) => panic!("DerivedDeclarator::Pointer"),
            Self::Array(_) => panic!("DerivedDeclarator::Array"),
            Self::Function(_) => panic!("DerivedDeclarator::Function"),
            Self::KRFunction(identifiers) => {
                let params = identifiers
                    .iter()
                    .map(|identifier| identifier.node.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("({})", params)
            }
            Self::Block(_) => panic!("DerivedDeclarator::Block"),
        }
    }
}

impl WriteString for Statement {
    fn write_string(&self) -> String {
        match self {
            Self::Labeled(_) => panic!("Statement::Labeled"),
            Self::Compound(items) => {
                let body = items
                    .iter()
                    .map(|item| item.write_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                if body.is_empty() {
                    "{}".to_string()
                } else {
                    format!("{{\n{}\n}}", body)
                }
            }
            Self::Expression(expr) => match expr {
                Some(expr) => format!("{};", expr.write_string()),
                None => ";".to_string(),
            },
            Self::If(_) => panic!("Statement::If"),
            Self::Switch(_) => panic!("Statement::Switch"),
            Self::While(_) => panic!("Statement::While"),
            Self::DoWhile(_) => panic!("Statement::DoWhile"),
            Self::For(_) => panic!("Statement::For"),
            Self::Goto(_) => panic!("Statement::Goto"),
            Self::Continue => "continue;".to_string(),
            Self::Break => "break;".to_string(),
            Self::Return(expr) => match expr {
                Some(expr) => format!("return {};", expr.write_string()),
                None => "return;".to_string(),
            },
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
