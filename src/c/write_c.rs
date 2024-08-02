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

impl WriteString for BinaryOperatorExpression {
    fn write_string(&self) -> String {
        match self.operator.node {
            BinaryOperator::Index => {
                format!("{}[{}]", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::Multiply => {
                format!("{} * {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::Divide => {
                format!("{} / {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::Modulo => {
                format!("{} % {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::Plus => {
                format!("{} + {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::Minus => {
                format!("{} - {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::ShiftLeft => {
                format!("{} << {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::ShiftRight => {
                format!("{} >> {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::Less => {
                format!("{} < {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::Greater => {
                format!("{} > {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::LessOrEqual => {
                format!("{} <= {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::GreaterOrEqual => {
                format!("{} >= {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::Equals => {
                format!("{} == {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::NotEquals => {
                format!("{} != {}", self.lhs.write_string(), self.rhs.write_string())
            }
            BinaryOperator::BitwiseAnd => {
                format!(
                    "({} & {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::BitwiseXor => {
                format!(
                    "({} ^ {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::BitwiseOr => {
                format!(
                    "({} | {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::LogicalAnd => {
                format!(
                    "({} && {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::LogicalOr => {
                format!(
                    "({} || {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::Assign => {
                format!(
                    "({} = {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignMultiply => {
                format!(
                    "({} *= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignDivide => {
                format!(
                    "({} /= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignModulo => {
                format!(
                    "({} %= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignPlus => {
                format!(
                    "({} += {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignMinus => {
                format!(
                    "({} -= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignShiftLeft => {
                format!(
                    "({} <<= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignShiftRight => {
                format!(
                    "({} >>= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignBitwiseAnd => {
                format!(
                    "({} &= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignBitwiseXor => {
                format!(
                    "({} ^= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
            BinaryOperator::AssignBitwiseOr => {
                format!(
                    "({} |= {})",
                    self.lhs.write_string(),
                    self.rhs.write_string()
                )
            }
        }
    }
}

impl WriteString for UnaryOperatorExpression {
    fn write_string(&self) -> String {
        match self.operator.node {
            UnaryOperator::PostIncrement => {
                format!("{}++", self.operand.write_string())
            }
            UnaryOperator::PostDecrement => {
                format!("{}--", self.operand.write_string())
            }
            UnaryOperator::PreIncrement => {
                format!("++({})", self.operand.write_string())
            }
            UnaryOperator::PreDecrement => {
                format!("--({})", self.operand.write_string())
            }
            UnaryOperator::Address => {
                format!("&({})", self.operand.write_string())
            }
            UnaryOperator::Indirection => {
                format!("*({})", self.operand.write_string())
            }
            UnaryOperator::Plus => {
                format!("+{}", self.operand.write_string())
            }
            UnaryOperator::Minus => {
                format!("-{}", self.operand.write_string())
            }
            UnaryOperator::Complement => {
                format!("~{}", self.operand.write_string())
            }
            UnaryOperator::Negate => {
                format!("!{}", self.operand.write_string())
            }
        }
    }
}

impl WriteString for Expression {
    fn write_string(&self) -> String {
        match self {
            Expression::Identifier(ident) => ident.node.name.clone(),
            Expression::Constant(cst) => match &cst.node {
                Constant::Integer(it) => {
                    let s: String = match &it.base {
                        IntegerBase::Decimal => {
                            let str_slice: &str = &it.number;
                            format!("{}", str_slice)
                        }
                        IntegerBase::Octal => {
                            let str_slice: &str = &it.number;
                            format!("{}", str_slice)
                        }
                        IntegerBase::Hexadecimal => {
                            let str_slice: &str = &it.number;
                            format!("0x{}", str_slice)
                        }
                        IntegerBase::Binary => {
                            let str_slice: &str = &it.number;
                            format!("{}", str_slice)
                        }
                    };
                    match &it.suffix.size {
                        IntegerSize::Int => s,
                        IntegerSize::Long => {
                            format!("{}L", s)
                        }
                        IntegerSize::LongLong => {
                            format!("{}LL", s)
                        }
                    }
                }
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
            Expression::Member(mem) => match mem.node.operator.node {
                MemberOperator::Direct => {
                    format!(
                        "{}.{}",
                        mem.node.expression.node.write_string(),
                        mem.node.identifier.node.name
                    )
                }
                MemberOperator::Indirect => todo!(),
            },
            Expression::Call(exp) => {
                let mut s: Vec<String> = Vec::new();
                for v in &exp.node.arguments {
                    s.push(v.node.write_string());
                }
                format!("{}({})", exp.node.callee.write_string(), s.join(","))
            }
            Expression::CompoundLiteral(_) => todo!(),
            Expression::SizeOfTy(_) => todo!(),
            Expression::SizeOfVal(_) => todo!(),
            Expression::AlignOf(ali) => {
                format!("_Alignof({})", ali.node.0.node.write_string())
            }
            Expression::UnaryOperator(u_op) => u_op.node.write_string(),
            Expression::Cast(cast) => {
                format!(
                    "({}){}",
                    cast.node.type_name.node.write_string(),
                    cast.node.expression.node.write_string()
                )
            }
            Expression::BinaryOperator(b_op) => b_op.node.write_string(),
            Expression::Conditional(cond) => {
                format!(
                    "({} ? {} : {})",
                    cond.node.condition.node.write_string(),
                    cond.node.then_expression.node.write_string(),
                    cond.node.else_expression.node.write_string()
                )
            }
            Expression::Comma(exp) => {
                let mut s: Vec<String> = Vec::new();
                let vec: &Vec<Node<Expression>> = &*exp;
                for p in vec {
                    s.push(p.node.write_string());
                }
                format!("({})", s.join(", "))
            }
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

impl WriteString for SpecifierQualifier {
    fn write_string(&self) -> String {
        match self {
            SpecifierQualifier::TypeSpecifier(spec) => spec.node.write_string(),
            SpecifierQualifier::TypeQualifier(spec) => spec.node.write_string(),
            SpecifierQualifier::Extension(_) => todo!(),
        }
    }
}

impl WriteString for TypeName {
    fn write_string(&self) -> String {
        let mut s: String = String::new();
        for v in &self.specifiers {
            s.push_str(v.node.write_string().as_str());
            s.push(' ');
        }
        s = s.trim().to_string();
        format!("{}{}", s, self.declarator.write_string())
    }
}

impl WriteString for Designator {
    fn write_string(&self) -> String {
        match self {
            Designator::Index(exp) => exp.node.write_string(),
            Designator::Member(ident) => ident.node.name.clone(),
            Designator::Range(range) => {
                format!(
                    "{} {}",
                    range.node.from.node.write_string(),
                    range.node.to.node.write_string()
                )
            }
        }
    }
}

impl WriteString for InitializerListItem {
    fn write_string(&self) -> String {
        let mut s: String = String::new();
        for v in &self.designation {
            s.push_str(v.node.write_string().as_str());
        }
        s.push_str(self.initializer.node.write_string().as_str());
        s
    }
}

impl WriteString for Declaration {
    fn write_string(&self) -> String {
        let mut s: String = String::new();
        for v in &self.specifiers {
            s.push_str(v.node.write_string().as_str());
            s.push(' ');
        }
        s = s.trim().to_string();
        for v in &self.declarators {
            s.push(' ');
            s.push_str(v.node.declarator.write_string().as_str());
            let p = match &v.node.initializer {
                Some(init) => match &init.node {
                    Initializer::Expression(exp) => format!(" = {}", exp.write_string()),
                    Initializer::List(list) => {
                        let mut s: Vec<String> = Vec::new();

                        for v in list {
                            s.push(v.node.write_string());
                        }
                        format!(" = {{{}}}", s.join(", "))
                    }
                },
                None => "".to_string(),
            };
            s.push_str(p.as_str());
        }
        s
    }
}

impl WriteString for StaticAssert {
    fn write_string(&self) -> String {
        let mut s: String = self.expression.node.write_string();
        for v in &self.message.node {
            s.push_str(v);
        }
        s
    }
}

impl WriteString for ForInitializer {
    fn write_string(&self) -> String {
        match self {
            ForInitializer::Empty => "".to_string(),
            ForInitializer::Expression(exp) => {
                format!("{}", exp.write_string())
            }
            ForInitializer::Declaration(exp) => {
                format!("{}", exp.write_string())
            }
            ForInitializer::StaticAssert(exp) => {
                format!("_StaticAssert({})", exp.write_string())
            }
        }
    }
}

impl WriteString for Initializer {
    fn write_string(&self) -> String {
        match self {
            Initializer::Expression(exp) => exp.write_string(),
            Initializer::List(list) => {
                let mut s: String = String::new();
                for p in list {
                    s.push_str(p.node.write_string().as_str());
                }
                s
            }
        }
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
            DeclarationSpecifier::TypeSpecifier(spec) => spec.node.write_string(),
            DeclarationSpecifier::TypeQualifier(qlf) => qlf.node.write_string(),
            DeclarationSpecifier::Function(spec) => match spec.node {
                FunctionSpecifier::Inline => "__inline__".to_string(),
                FunctionSpecifier::Noreturn => "_Noreturn".to_string(),
            },
            DeclarationSpecifier::Extension(_) => todo!(),
        }
    }
}

impl WriteString for TypeSpecifier {
    fn write_string(&self) -> String {
        match self {
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
            TypeSpecifier::Struct(struct_) => {
                // struct todo! how to deal with indent of problem
                let mut s: String = match struct_.node.kind.node {
                    StructKind::Struct => "struct".to_string(),
                    StructKind::Union => "union".to_string(),
                };
                if let Some(ident) = &struct_.node.identifier {
                    s = format!("{} {}", s, ident.node.name);
                }
                if let Some(declaration) = &struct_.node.declarations {
                    for p in declaration {
                        match &p.node {
                            StructDeclaration::Field(field) => {
                                let mut str: String = String::new();
                                for v in &field.node.specifiers {
                                    str.push_str(v.node.write_string().as_str());
                                }
                                for v in &field.node.declarators {
                                    if let Some(declarator) = &v.node.declarator {
                                        str.push_str(declarator.node.write_string().as_str());
                                    }
                                }
                                s = format!("{{{}}}", str);
                            }
                            StructDeclaration::StaticAssert(_) => todo!(),
                        }
                    }
                }
                s
            }
            TypeSpecifier::Enum(_) => "enum".to_string(),
            TypeSpecifier::TypedefName(_) => "typedef name".to_string(),
            TypeSpecifier::TypeOf(_) => "typeof".to_string(),
            TypeSpecifier::TS18661Float(_) => "TS18661 float".to_string(),
        }
    }
}

impl WriteString for TypeQualifier {
    fn write_string(&self) -> String {
        match self {
            TypeQualifier::Const => "const".to_string(),
            TypeQualifier::Restrict => "restrict".to_string(),
            TypeQualifier::Volatile => "volatile".to_string(),
            TypeQualifier::Nonnull => "_Nonnull".to_string(),
            TypeQualifier::NullUnspecified => "_Null_unspecified".to_string(),
            TypeQualifier::Nullable => "_Nullable".to_string(),
            TypeQualifier::Atomic => "_Atomic".to_string(),
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
impl WriteString for PointerQualifier {
    fn write_string(&self) -> String {
        match self {
            PointerQualifier::TypeQualifier(qls) => qls.node.write_string(),
            PointerQualifier::Extension(_) => todo!(),
        }
    }
}

impl WriteString for ArraySize {
    fn write_string(&self) -> String {
        match self {
            ArraySize::Unknown => {
                format!("[]")
            }
            ArraySize::VariableUnknown => {
                format!("[*]")
            }
            ArraySize::VariableExpression(exp) => {
                format!("[{}]", exp.node.write_string())
            }
            ArraySize::StaticExpression(exp) => {
                format!("[static {}]", exp.node.write_string())
            }
        }
    }
}

impl WriteString for Declarator {
    fn write_string(&self) -> String {
        let mut s = String::new();
        let mut str: String = String::new();

        for v in &self.derived {
            match &v.node {
                DerivedDeclarator::Pointer(qlf) => {
                    s.push('*');
                    for p in qlf {
                        str.push_str(p.node.write_string().as_str())
                    }
                }
                DerivedDeclarator::Array(decl) => {
                    let mut v: String = String::new();
                    for p in &decl.node.qualifiers {
                        v.push_str(p.node.write_string().as_str());
                    }
                    str.push_str(&format!("{}{}", v, decl.node.size.write_string()));
                }
                DerivedDeclarator::Function(decl) => {
                    let mut vec: Vec<String> = Vec::new();
                    for p in &decl.node.parameters {
                        let mut v: String = String::new();

                        for vec in &p.node.specifiers {
                            v.push_str(vec.node.write_string().as_str());
                            v.push(' ');
                        }
                        v = v.trim().to_string();
                        vec.push(format!(
                            "{} {}",
                            v,
                            p.node.declarator.write_string().as_str()
                        ));
                    }

                    str.push_str(&format!("({})", vec.join(",")));
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
        }
        s.push_str(self.kind.node.write_string().as_str());
        s.push_str(str.as_str());

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
                if let Some(els) = &stmt.node.else_statement {
                    write_indent(indent, write)?;
                    writeln!(write, "else ")?;
                    els.node.write_line(indent, write)?;
                }

                Ok(())
            }
            Self::Expression(exp) => {
                write_indent(indent, write)?;
                writeln!(write, "{};", exp.as_ref().write_string())?;
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
