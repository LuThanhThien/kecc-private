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
        match self {
            Self::Expression(expr) => expr.write_string(),
            Self::List(items) => format!("{{{}}}", join(items, ", ")),
        }
    }
}

impl WriteString for ExternalDeclaration {
    fn write_string(&self) -> String {
        match self {
            Self::Declaration(decl) => decl.write_string(),
            Self::StaticAssert(_) => panic!("StaticAssert"),
            Self::FunctionDefinition(decl) => decl.write_string(),
        }
    }
}

impl WriteString for Declaration {
    fn write_string(&self) -> String {
        format!(
            "{} {};",
            join(&self.specifiers, " "),
            join(&self.declarators, ", ")
        )
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
    }
}

impl WriteString for DeclarationSpecifier {
    fn write_string(&self) -> String {
        match self {
            Self::StorageClass(spec) => match spec.node {
                StorageClassSpecifier::Typedef => "typedef".to_string(),
                _ => panic!("unsupported storage class"),
            },
            Self::TypeSpecifier(type_specifier) => type_specifier.write_string(),
            Self::TypeQualifier(qualifier) => qualifier.write_string(),
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
            Self::Struct(struct_type) => struct_type.write_string(),
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

        let mut pointers = String::new();
        let mut result = self.kind.write_string();

        // The AST stores pointer levels in source order. Preserve qualifier placement
        // while emitting pointers before the name and suffixes after it.
        for derived in &self.derived {
            if matches!(derived.node, DerivedDeclarator::Pointer(_)) {
                pointers.push_str(&derived.write_string());
            } else {
                result.push_str(&derived.write_string());
            }
        }
        result.insert_str(0, &pointers);

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
            Self::Pointer(qualifiers) => format!("* {} ", join(qualifiers, " ")),
            Self::Array(array) => format!("[{}]", array.node.size.write_string()),
            Self::Function(function) => format!("({})", join(&function.node.parameters, ", ")),
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
            Self::Labeled(stmt) => format!(
                "{}: {}",
                stmt.node.label.write_string(),
                stmt.node.statement.write_string()
            ),
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
            Self::If(stmt) => {
                let stmt = &stmt.node;
                let mut result = format!(
                    "if ({}) {}",
                    stmt.condition.write_string(),
                    stmt.then_statement.write_string()
                );
                if let Some(otherwise) = &stmt.else_statement {
                    result.push_str(&format!(" else {}", otherwise.write_string()));
                }
                result
            }
            Self::Switch(stmt) => format!(
                "switch ({}) {}",
                stmt.node.expression.write_string(),
                stmt.node.statement.write_string()
            ),
            Self::While(stmt) => format!(
                "while ({}) {}",
                stmt.node.expression.write_string(),
                stmt.node.statement.write_string()
            ),
            Self::DoWhile(stmt) => format!(
                "do {} while ({});",
                stmt.node.statement.write_string(),
                stmt.node.expression.write_string()
            ),
            Self::For(stmt) => format!(
                "for ({} {}; {}) {}",
                stmt.node.initializer.write_string(),
                stmt.node.condition.write_string(),
                stmt.node.step.write_string(),
                stmt.node.statement.write_string()
            ),
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
            Self::Declaration(decl) => decl.write_string(),
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
            Self::Member(member) => format!(
                "({}){}{}",
                member.node.expression.write_string(),
                match member.node.operator.node {
                    MemberOperator::Direct => ".",
                    MemberOperator::Indirect => "->",
                },
                member.node.identifier.node.name
            ),
            Self::Call(call) => format!(
                "({})({})",
                call.node.callee.write_string(),
                join(&call.node.arguments, ", ")
            ),
            Self::CompoundLiteral(_) => panic!("Expression::CompoundLiteral"),
            Self::SizeOfTy(ty) => format!("sizeof({})", ty.node.0.write_string()),
            Self::SizeOfVal(value) => format!("sizeof({})", value.node.0.write_string()),
            Self::AlignOf(ty) => format!("_Alignof({})", ty.node.0.write_string()),
            Self::UnaryOperator(unary) => {
                let unary = &unary.node;
                let operand = unary.operand.write_string();
                match unary.operator.node {
                    UnaryOperator::PostIncrement | UnaryOperator::PostDecrement => {
                        format!("(({operand}){})", unary.operator.write_string())
                    }
                    _ => format!("({}({operand}))", unary.operator.write_string()),
                }
            }
            Self::Cast(cast) => format!(
                "(({})({}))",
                cast.node.type_name.write_string(),
                cast.node.expression.write_string()
            ),
            Self::BinaryOperator(binary) => {
                let binary = &binary.node;
                if binary.operator.node == BinaryOperator::Index {
                    format!(
                        "({})[{}]",
                        binary.lhs.write_string(),
                        binary.rhs.write_string()
                    )
                } else {
                    format!(
                        "({} {} {})",
                        binary.lhs.write_string(),
                        binary.operator.write_string(),
                        binary.rhs.write_string()
                    )
                }
            }
            Self::Conditional(expr) => format!(
                "({} ? {} : {})",
                expr.node.condition.write_string(),
                expr.node.then_expression.write_string(),
                expr.node.else_expression.write_string()
            ),
            Self::Comma(exprs) => format!("({})", join(exprs, ", ")),
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
            Self::Float(float) => float.write_string(),
            Self::Character(character) => character.clone(),
        }
    }
}

impl WriteString for Integer {
    fn write_string(&self) -> String {
        let prefix = match self.base {
            IntegerBase::Decimal => "",
            IntegerBase::Octal => "0",
            IntegerBase::Hexadecimal => "0x",
            IntegerBase::Binary => "0b",
        };
        let size = match self.suffix.size {
            IntegerSize::Int => "",
            IntegerSize::Long => "L",
            IntegerSize::LongLong => "LL",
        };
        format!(
            "{prefix}{}{size}{}",
            self.number,
            if self.suffix.unsigned { "U" } else { "" }
        )
    }
}

/// Join AST fragments without changing their order.
fn join<T: WriteString>(items: &[T], separator: &str) -> String {
    items
        .iter()
        .map(WriteString::write_string)
        .collect::<Vec<_>>()
        .join(separator)
}

impl WriteString for InitializerListItem {
    fn write_string(&self) -> String {
        self.initializer.write_string()
    }
}

impl WriteString for InitDeclarator {
    fn write_string(&self) -> String {
        let mut result = self.declarator.write_string();
        if let Some(initializer) = &self.initializer {
            result.push_str(&format!(" = {}", initializer.write_string()));
        }
        result
    }
}

impl WriteString for TypeQualifier {
    fn write_string(&self) -> String {
        match self {
            Self::Const => "const".to_string(),
            _ => panic!("unsupported type qualifier"),
        }
    }
}

impl WriteString for PointerQualifier {
    fn write_string(&self) -> String {
        match self {
            Self::TypeQualifier(qualifier) => qualifier.write_string(),
            _ => panic!("unsupported pointer qualifier"),
        }
    }
}

impl WriteString for ArraySize {
    fn write_string(&self) -> String {
        match self {
            Self::VariableExpression(expr) => expr.write_string(),
            _ => panic!("unsupported array size"),
        }
    }
}

impl WriteString for ParameterDeclaration {
    fn write_string(&self) -> String {
        format!(
            "{} {}",
            join(&self.specifiers, " "),
            self.declarator.write_string()
        )
    }
}

impl WriteString for TypeName {
    fn write_string(&self) -> String {
        format!(
            "{} {}",
            join(&self.specifiers, " "),
            self.declarator.write_string()
        )
    }
}

impl WriteString for SpecifierQualifier {
    fn write_string(&self) -> String {
        match self {
            Self::TypeSpecifier(specifier) => specifier.write_string(),
            Self::TypeQualifier(qualifier) => qualifier.write_string(),
            _ => panic!("unsupported specifier qualifier"),
        }
    }
}

impl WriteString for StructType {
    fn write_string(&self) -> String {
        let name = self.identifier.as_ref().map_or("", |name| &name.node.name);
        match &self.declarations {
            Some(fields) => format!("struct {name} {{\n{}\n}}", join(fields, "\n")),
            None => format!("struct {name}"),
        }
    }
}

impl WriteString for StructDeclaration {
    fn write_string(&self) -> String {
        match self {
            Self::Field(field) => format!(
                "{} {};",
                join(&field.node.specifiers, " "),
                join(&field.node.declarators, ", ")
            ),
            _ => panic!("unsupported struct declaration"),
        }
    }
}

impl WriteString for StructDeclarator {
    fn write_string(&self) -> String {
        self.declarator.write_string()
    }
}

impl WriteString for Label {
    fn write_string(&self) -> String {
        match self {
            Self::Case(expr) => format!("case {}", expr.write_string()),
            Self::Default => "default".to_string(),
            _ => panic!("unsupported label"),
        }
    }
}

impl WriteString for ForInitializer {
    fn write_string(&self) -> String {
        match self {
            Self::Empty => ";".to_string(),
            Self::Expression(expr) => format!("{};", expr.write_string()),
            Self::Declaration(decl) => decl.write_string(),
            _ => panic!("unsupported for initializer"),
        }
    }
}

impl WriteString for Float {
    fn write_string(&self) -> String {
        let prefix = match self.base {
            FloatBase::Decimal => "",
            FloatBase::Hexadecimal => "0x",
        };
        let suffix = match self.suffix.format {
            FloatFormat::Float => "F",
            FloatFormat::Double => "",
            FloatFormat::LongDouble => "L",
            _ => panic!("unsupported floating point format"),
        };
        format!("{prefix}{}{suffix}", self.number)
    }
}

impl WriteString for UnaryOperator {
    fn write_string(&self) -> String {
        match self {
            Self::PostIncrement | Self::PreIncrement => "++",
            Self::PostDecrement | Self::PreDecrement => "--",
            Self::Address => "&",
            Self::Indirection => "*",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Complement => "~",
            Self::Negate => "!",
        }
        .to_string()
    }
}

impl WriteString for BinaryOperator {
    fn write_string(&self) -> String {
        match self {
            Self::Index => panic!("index is printed with brackets"),
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::ShiftLeft => "<<",
            Self::ShiftRight => ">>",
            Self::Less => "<",
            Self::Greater => ">",
            Self::LessOrEqual => "<=",
            Self::GreaterOrEqual => ">=",
            Self::Equals => "==",
            Self::NotEquals => "!=",
            Self::BitwiseAnd => "&",
            Self::BitwiseXor => "^",
            Self::BitwiseOr => "|",
            Self::LogicalAnd => "&&",
            Self::LogicalOr => "||",
            Self::Assign => "=",
            Self::AssignMultiply => "*=",
            Self::AssignDivide => "/=",
            Self::AssignModulo => "%=",
            Self::AssignPlus => "+=",
            Self::AssignMinus => "-=",
            Self::AssignShiftLeft => "<<=",
            Self::AssignShiftRight => ">>=",
            Self::AssignBitwiseAnd => "&=",
            Self::AssignBitwiseXor => "^=",
            Self::AssignBitwiseOr => "|=",
        }
        .to_string()
    }
}
