use lang_c::ast::*;
use lang_c::span::Node;

use core::ops::Deref;
use std::io::{Result, Write};

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

impl<T: WriteString> WriteString for Box<T> {
    fn write_string(&self) -> String {
        self.deref().write_string()
    }
}

impl<T: WriteString> WriteString for &T {
    fn write_string(&self) -> String {
        (*self).write_string()
    }
}

impl<T: WriteString> WriteString for Option<T> {
    fn write_string(&self) -> String {
        if let Some(this) = self {
            this.write_string()
        } else {
            "".to_string()
        }
    }
}

impl WriteString for Expression {
    fn write_string(&self) -> String {
        match self {
            Expression::Identifier(ident) => ident.node.name.clone(),
            Expression::Constant(cst) => match &cst.node {
                Constant::Integer(int) => match &int.base {
                    IntegerBase::Decimal => {
                        let str_slice: &str = &int.number;
                        str_slice.parse::<i64>().expect("error").to_string()
                    }
                    IntegerBase::Octal => {
                        let str_slice: &str = &int.number;
                        format!("{:X}", str_slice.parse::<i64>().expect("error"))
                    }
                    IntegerBase::Hexadecimal => {
                        let str_slice: &str = &int.number;
                        format!("{:o}", str_slice.parse::<i64>().expect("error"))
                    }
                    IntegerBase::Binary => {
                        let str_slice: &str = &int.number;
                        format!("{:b}", str_slice.parse::<i64>().expect("error"))
                    }
                },
                Constant::Float(float) => match &float.base {
                    FloatBase::Decimal => {
                        let str_slice: &str = &float.number;
                        str_slice.parse::<f64>().expect("error").to_string()
                    }
                    FloatBase::Hexadecimal => {
                        let str_slice: &str = &float.number;
                        let float_value = str_slice.parse::<f64>().expect("error");
                        let integer_part = float_value.floor() as i64;
                        let fractional_part = float_value - integer_part as f64;
                        let hex_integer_part = format!("{:X}", integer_part);
                        let fractional_part_hex = (fractional_part * 1_000_000.0).round() as i64;
                        let hex_fractional_part = format!("{:X}", fractional_part_hex);
                        format!("{}.{}", hex_integer_part, hex_fractional_part)
                    }
                },
                Constant::Character(str) => {
                    format!("{}", str.as_str())
                }
            },
            Expression::StringLiteral(_) => todo!(),
            Expression::GenericSelection(_) => todo!(),
            Expression::Member(_) => todo!(),
            Expression::Call(exp) => {
                let mut s: String = String::new();
                s.push_str(exp.node.callee.write_string().as_str());
                for v in &exp.node.arguments {
                    s.push_str(&format!("({})", v.node.write_string().to_string()));
                }
                s
            }
            Expression::CompoundLiteral(_) => todo!(),
            Expression::SizeOfTy(_) => todo!(),
            Expression::SizeOfVal(_) => todo!(),
            Expression::AlignOf(_) => todo!(),
            Expression::UnaryOperator(_) => todo!(),
            Expression::Cast(_) => todo!(),
            Expression::BinaryOperator(b_op) => match b_op.as_ref().node.operator.node {
                BinaryOperator::Assign => {
                    format!(
                        "({} {})",
                        b_op.node.lhs.node.write_string(),
                        b_op.node.rhs.node.write_string()
                    )
                }
                BinaryOperator::Index => todo!(),
                BinaryOperator::Multiply => todo!(),
                BinaryOperator::Divide => todo!(),
                BinaryOperator::Modulo => {
                    format!(
                        "{} % {}",
                        b_op.node.lhs.node.write_string(),
                        b_op.node.rhs.node.write_string()
                    )
                }
                BinaryOperator::Plus => {
                    format!(
                        "{} + {}",
                        b_op.node.lhs.node.write_string(),
                        b_op.node.rhs.node.write_string()
                    )
                }
                BinaryOperator::Minus => {
                    format!(
                        "{} - {}",
                        b_op.node.lhs.node.write_string(),
                        b_op.node.rhs.node.write_string()
                    )
                }
                BinaryOperator::ShiftLeft => todo!(),
                BinaryOperator::ShiftRight => todo!(),
                BinaryOperator::Less => {
                    format!(
                        "{} < {}",
                        b_op.node.lhs.node.write_string(),
                        b_op.node.rhs.node.write_string()
                    )
                }
                BinaryOperator::Greater => todo!(),
                BinaryOperator::LessOrEqual => todo!(),
                BinaryOperator::GreaterOrEqual => todo!(),
                BinaryOperator::Equals => todo!(),
                BinaryOperator::NotEquals => todo!(),
                BinaryOperator::BitwiseAnd => todo!(),
                BinaryOperator::BitwiseXor => todo!(),
                BinaryOperator::BitwiseOr => todo!(),
                BinaryOperator::LogicalAnd => todo!(),
                BinaryOperator::LogicalOr => todo!(),
                BinaryOperator::AssignMultiply => todo!(),
                BinaryOperator::AssignDivide => todo!(),
                BinaryOperator::AssignModulo => todo!(),
                BinaryOperator::AssignPlus => todo!(),
                BinaryOperator::AssignMinus => todo!(),
                BinaryOperator::AssignShiftLeft => todo!(),
                BinaryOperator::AssignShiftRight => todo!(),
                BinaryOperator::AssignBitwiseAnd => todo!(),
                BinaryOperator::AssignBitwiseXor => todo!(),
                BinaryOperator::AssignBitwiseOr => todo!(),
            },
            Expression::Conditional(_) => todo!(),
            Expression::Comma(_) => todo!(),
            Expression::OffsetOf(_) => todo!(),
            Expression::VaArg(_) => todo!(),
            Expression::Statement(_) => todo!(),
        }
    }
}

impl WriteLine for TranslationUnit {
    fn write_line(&self, _indent: usize, _write: &mut dyn Write) -> Result<()> {
        for v in &self.0 {
            v.node.write_line(_indent, _write)?;
            writeln!(_write)?;
        }

        Ok(())
    }
}

impl WriteString for Declaration {
    fn write_string(&self) -> String {
        let mut s: String = String::new();
        for v in &self.specifiers {
            s.push_str(v.node.write_string().as_str());
        }
        for v in &self.declarators {
            s.push(' ');
            s.push_str(v.node.declarator.write_string().as_str());
            let p = match &v.node.initializer {
                Some(init) => match &init.node {
                    Initializer::Expression(exp) => format!(" = {}", exp.write_string()),
                    Initializer::List(_) => todo!(),
                },
                None => "".to_string(),
            };
            s.push_str(p.as_str());
        }
        s
    }
}

impl WriteString for ForInitializer {
    fn write_string(&self) -> String {
        "ForInitializer".to_string()
    }
}

impl WriteString for Initializer {
    fn write_string(&self) -> String {
        "Initializer".to_string()
    }
}

impl WriteString for Label {
    fn write_string(&self) -> String {
        "Label".to_string()
    }
}

impl WriteString for DeclarationSpecifier {
    fn write_string(&self) -> String {
        match self {
            DeclarationSpecifier::Alignment(_) => todo!(),
            DeclarationSpecifier::StorageClass(_) => todo!(),
            DeclarationSpecifier::TypeSpecifier(spec) => match spec.node {
                TypeSpecifier::Void => "void".to_string(),
                TypeSpecifier::Char => "char".to_string(),
                TypeSpecifier::Short => "short".to_string(),
                TypeSpecifier::Int => "int".to_string(),
                TypeSpecifier::Long => "long".to_string(),
                TypeSpecifier::Float => "float".to_string(),
                TypeSpecifier::Double => "double".to_string(),
                TypeSpecifier::Signed => "signed".to_string(),
                TypeSpecifier::Unsigned => "unsigned".to_string(),
                TypeSpecifier::Bool => "_Bool".to_string(),
                TypeSpecifier::Complex => "_Complex".to_string(),
                TypeSpecifier::Atomic(_) => "atomic".to_string(),
                TypeSpecifier::Struct(_) => "struct".to_string(),
                TypeSpecifier::Enum(_) => "enum".to_string(),
                TypeSpecifier::TypedefName(_) => "typedef name".to_string(),
                TypeSpecifier::TypeOf(_) => "typeof".to_string(),
                TypeSpecifier::TS18661Float(_) => "TS18661 float".to_string(),
            },
            DeclarationSpecifier::TypeQualifier(_) => todo!(),
            DeclarationSpecifier::Function(spec) => match spec.node {
                FunctionSpecifier::Inline => "__inline__".to_string(),
                FunctionSpecifier::Noreturn => "_Noreturn".to_string(),
            },
            DeclarationSpecifier::Extension(_) => todo!(),
        }
    }
}

impl WriteString for PointerQualifier {
    fn write_string(&self) -> String {
        match self {
            PointerQualifier::TypeQualifier(qlf) => match qlf.node {
                TypeQualifier::Const => "const".to_string(),
                TypeQualifier::Restrict => "restrict".to_string(),
                TypeQualifier::Volatile => "volatile".to_string(),
                TypeQualifier::Nonnull => "_Nonnull".to_string(),
                TypeQualifier::NullUnspecified => "_Null_unspecified".to_string(),
                TypeQualifier::Nullable => "_Nullable".to_string(),
                TypeQualifier::Atomic => "_Atomic".to_string(),
            },
            PointerQualifier::Extension(_) => todo!(),
        }
    }
}

impl WriteString for DeclaratorKind {
    fn write_string(&self) -> String {
        match self {
            DeclaratorKind::Abstract => "".to_string(),
            DeclaratorKind::Identifier(ident) => ident.node.name.to_string(),
            DeclaratorKind::Declarator(decl) => decl.node.write_string(),
        }
    }
}

impl WriteString for Declarator {
    fn write_string(&self) -> String {
        let mut s = String::new();
        s.push_str(self.kind.node.write_string().as_str());

        for v in &self.derived {
            let mut str: String = String::new();
            match &v.node {
                DerivedDeclarator::Pointer(qlf) => {
                    for p in qlf {
                        str.push_str(p.node.write_string().as_str())
                    }
                }
                DerivedDeclarator::Array(_) => todo!(),
                DerivedDeclarator::Function(decl) => {
                    let mut v: String = String::new();
                    for p in &decl.node.parameters {
                        for vec in &p.node.specifiers {
                            v.push_str(vec.node.write_string().as_str());
                        }
                        v.push(' ');
                        v.push_str(p.node.declarator.write_string().as_str());
                    }
                    str.push_str(&format!("({})", v));
                }
                DerivedDeclarator::KRFunction(ident) => {
                    let mut v: String = String::new();
                    for p in ident {
                        v.push_str(&p.node.name);
                    }
                    str.push_str(&format!("({})", v));
                }
                DerivedDeclarator::Block(_) => todo!(),
            }
            s.push_str(str.as_str());
        }
        s
    }
}
impl WriteString for Identifier {
    fn write_string(&self) -> String {
        self.name.to_string()
    }
}
impl WriteLine for BlockItem {
    fn write_line(&self, indent: usize, write: &mut dyn Write) -> Result<()> {
        match self {
            Self::Declaration(de) => de.write_line(indent, write),
            Self::Statement(fd) => fd.write_line(indent, write),
            Self::StaticAssert(_) => panic!(),
        }
    }
}

impl WriteLine for ExternalDeclaration {
    fn write_line(&self, indent: usize, write: &mut dyn Write) -> Result<()> {
        match self {
            Self::Declaration(de) => de.write_line(indent, write),
            Self::FunctionDefinition(fd) => fd.write_line(indent, write),
            Self::StaticAssert(_) => panic!(),
        }
    }
}

impl WriteLine for Declaration {
    fn write_line(&self, indent: usize, write: &mut dyn Write) -> Result<()> {
        write_indent(indent, write)?;
        writeln!(write, "{};", self.write_string())?;
        Ok(())
    }
}

impl WriteLine for FunctionDefinition {
    fn write_line(&self, indent: usize, write: &mut dyn Write) -> Result<()> {
        let mut return_type: String = String::new();

        for v in &self.specifiers {
            return_type.push_str(v.node.write_string().as_str());
        }

        writeln!(write, "{} {}", return_type, self.declarator.write_string())?;
        self.statement.write_line(indent, write)?;
        write_indent(indent, write)?;
        Ok(())
    }
}

impl WriteLine for Statement {
    fn write_line(&self, indent: usize, write: &mut dyn Write) -> Result<()> {
        match self {
            Self::Labeled(stmt) => {
                write_indent(indent, write)?;
                writeln!(write, "{}", stmt.node.label.write_string())?;
                stmt.node.statement.write_line(indent + 1, write)?;
                Ok(())
            }
            Self::Compound(items) => {
                write_indent(indent, write)?;
                writeln!(write, "{{")?;

                for it in items.iter() {
                    it.write_line(indent + 1, write)?;
                }

                write_indent(indent, write)?;
                writeln!(write, "}}")?;

                Ok(())
            }
            Self::If(stmt) => {
                write_indent(indent, write)?;
                writeln!(write, "if ({})", stmt.node.condition.write_string())?;
                stmt.node.then_statement.write_line(indent, write)?;
                Ok(())
            }
            Self::Expression(exp) => {
                write_indent(indent, write)?;
                writeln!(write, "Expression ({})", exp.as_ref().write_string())?;
                Ok(())
            }
            Self::Switch(stmt) => {
                write_indent(indent, write)?;
                writeln!(write, "switch ({})", stmt.node.expression.write_string())?;
                stmt.node.statement.write_line(indent + 1, write)?;
                Ok(())
            }
            Self::While(stmt) => {
                write_indent(indent, write)?;
                writeln!(write, "while ({})", stmt.node.expression.write_string())?;
                stmt.node.statement.write_line(indent + 1, write)?;
                Ok(())
            }
            Self::DoWhile(stmt) => {
                write_indent(indent, write)?;
                writeln!(write, "switch ({})", stmt.node.expression.write_string())?;
                stmt.node.statement.write_line(indent + 1, write)?;
                Ok(())
            }
            Self::For(stmt) => {
                write_indent(indent, write)?;
                writeln!(
                    write,
                    "for ({};{};{})",
                    stmt.node.initializer.node.write_string(),
                    stmt.node.condition.write_string(),
                    stmt.node.step.write_string(),
                )?;
                stmt.node.statement.write_line(indent + 1, write)?;
                Ok(())
            }
            Self::Goto(iden) => {
                write_indent(indent, write)?;
                writeln!(write, "goto {};", iden.node.write_string())?;
                Ok(())
            }
            Self::Continue => {
                write_indent(indent, write)?;
                writeln!(write, "continue;")?;
                Ok(())
            }
            Self::Break => {
                write_indent(indent, write)?;
                writeln!(write, "break;")?;
                Ok(())
            }
            Self::Return(exp) => {
                write_indent(indent, write)?;
                writeln!(write, "return {};", exp.as_ref().write_string())?;
                Ok(())
            }
            Self::Asm(_) => Ok(()),
        }
    }
}
