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
        "Expression".to_string()
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
            match v.node.clone() {
                DeclarationSpecifier::Alignment(_) => todo!(),
                DeclarationSpecifier::StorageClass(_) => todo!(),
                DeclarationSpecifier::TypeSpecifier(spec) => {
                    s.push_str(
                        match spec.node {
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
                        }
                        .as_str(),
                    );
                }
                DeclarationSpecifier::TypeQualifier(_) => todo!(),
                DeclarationSpecifier::Function(_) => todo!(),
                DeclarationSpecifier::Extension(_) => todo!(),
            }
            s.push(' ');
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
        "DeclarationSpecifier".to_string()
    }
}

impl WriteString for Declarator {
    fn write_string(&self) -> String {
        "Declarator".to_string()
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

impl WriteString for (&Vec<Node<DeclarationSpecifier>>, &Declarator) {
    fn write_string(&self) -> String {
        format!(
            "{} {}",
            self.0
                .iter()
                .map(|node| node.write_string())
                .collect::<Vec<_>>()
                .join(" "),
            self.1.write_string()
        )
    }
}

impl WriteLine for FunctionDefinition {
    fn write_line(&self, indent: usize, write: &mut dyn Write) -> Result<()> {
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
                stmt.node.then_statement.write_line(indent + 1, write)?;
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
                writeln!(write, "switch ({})", stmt.node.expression.write_string())?;
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
                writeln!(write, "{};", exp.as_ref().write_string())?;
                Ok(())
            }
            Self::Asm(_) => Ok(()),
        }
    }
}
