use serde_derive_internals::{Ctxt, Derive, ast as serde_ast, attr};
use std::fmt::{self, Debug};

pub struct ContainerDebug<'a>(pub &'a serde_ast::Container<'a>);

impl<'a> Debug for ContainerDebug<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = &self.0;
        f.debug_struct("Container")
            .field("ident", &c.ident)
            .field("attrs", &DebugContainerAttrs(&c.attrs))
            .field("data", &DebugData(&c.data))
            .finish()
    }
}

fn rename_rule_str(rule: attr::RenameRule) -> &'static str {
    match rule {
        attr::RenameRule::None => "none",
        attr::RenameRule::LowerCase => "lowercase",
        attr::RenameRule::UpperCase => "UPPERCASE",
        attr::RenameRule::PascalCase => "PascalCase",
        attr::RenameRule::CamelCase => "camelCase",
        attr::RenameRule::SnakeCase => "snake_case",
        attr::RenameRule::ScreamingSnakeCase => "SCREAMING_SNAKE_CASE",
        attr::RenameRule::KebabCase => "kebab-case",
        attr::RenameRule::ScreamingKebabCase => "SCREAMING-KEBAB-CASE",
    }
}

struct DebugStyle(serde_ast::Style);

impl Debug for DebugStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            serde_ast::Style::Struct => "Struct",
            serde_ast::Style::Tuple => "Tuple",
            serde_ast::Style::Newtype => "Newtype",
            serde_ast::Style::Unit => "Unit",
        })
    }
}

struct DebugTagType<'a>(&'a attr::TagType);

impl<'a> Debug for DebugTagType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            attr::TagType::External => f.write_str("External"),
            attr::TagType::Internal { tag } => {
                f.debug_struct("Internal").field("tag", tag).finish()
            }
            attr::TagType::Adjacent { tag, content } => f
                .debug_struct("Adjacent")
                .field("tag", tag)
                .field("content", content)
                .finish(),
            attr::TagType::None => f.write_str("None"),
        }
    }
}

struct DebugDefault<'a>(&'a attr::Default);

impl<'a> Debug for DebugDefault<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            attr::Default::None => f.write_str("None"),
            attr::Default::Default => f.write_str("Default"),
            attr::Default::Path(path) => write!(f, "Path({:?})", path),
        }
    }
}

struct DebugContainerAttrs<'a>(&'a attr::Container);

impl<'a> Debug for DebugContainerAttrs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.0;
        let name = a.name();
        let rules = a.rename_all_rules();
        let field_rules = a.rename_all_fields_rules();
        f.debug_struct("ContainerAttrs")
            .field("serialize_name", &name.serialize_name())
            .field("deserialize_name", &name.deserialize_name())
            .field("rename_all_serialize", &rename_rule_str(rules.serialize))
            .field(
                "rename_all_deserialize",
                &rename_rule_str(rules.deserialize),
            )
            .field(
                "rename_all_fields_serialize",
                &rename_rule_str(field_rules.serialize),
            )
            .field(
                "rename_all_fields_deserialize",
                &rename_rule_str(field_rules.deserialize),
            )
            .field("transparent", &a.transparent())
            .field("deny_unknown_fields", &a.deny_unknown_fields())
            .field("has_flatten", &a.has_flatten())
            .field("tag", &DebugTagType(a.tag()))
            .field("default", &DebugDefault(a.default()))
            .finish()
    }
}

struct DebugVariantAttrs<'a>(&'a attr::Variant);

impl<'a> Debug for DebugVariantAttrs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.0;
        let name = a.name();
        let rules = a.rename_all_rules();
        f.debug_struct("VariantAttrs")
            .field("serialize_name", &name.serialize_name())
            .field("deserialize_name", &name.deserialize_name())
            .field("rename_all_serialize", &rename_rule_str(rules.serialize))
            .field(
                "rename_all_deserialize",
                &rename_rule_str(rules.deserialize),
            )
            .field("skip_serializing", &a.skip_serializing())
            .field("skip_deserializing", &a.skip_deserializing())
            .field("untagged", &a.untagged())
            .finish()
    }
}

struct DebugFieldAttrs<'a>(&'a attr::Field);

impl<'a> Debug for DebugFieldAttrs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self.0;
        let name = a.name();
        f.debug_struct("FieldAttrs")
            .field("serialize_name", &name.serialize_name())
            .field("deserialize_name", &name.deserialize_name())
            .field("skip_serializing", &a.skip_serializing())
            .field("skip_deserializing", &a.skip_deserializing())
            .field("flatten", &a.flatten())
            .field("transparent", &a.transparent())
            .field("default", &DebugDefault(a.default()))
            .finish()
    }
}

struct DebugField<'a>(&'a serde_ast::Field<'a>);

impl<'a> Debug for DebugField<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field = self.0;
        f.debug_struct("Field")
            .field("member", &field.member)
            .field("ty", field.ty)
            .field("attrs", &DebugFieldAttrs(&field.attrs))
            .finish()
    }
}

struct DebugVariant<'a>(&'a serde_ast::Variant<'a>);

impl<'a> Debug for DebugVariant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.0;
        f.debug_struct("Variant")
            .field("ident", &v.ident)
            .field("style", &DebugStyle(v.style))
            .field("attrs", &DebugVariantAttrs(&v.attrs))
            .field(
                "fields",
                &v.fields.iter().map(DebugField).collect::<Vec<_>>(),
            )
            .finish()
    }
}

struct DebugData<'a>(&'a serde_ast::Data<'a>);

impl<'a> Debug for DebugData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            serde_ast::Data::Enum(variants) => f
                .debug_tuple("Enum")
                .field(&variants.iter().map(DebugVariant).collect::<Vec<_>>())
                .finish(),
            serde_ast::Data::Struct(style, fields) => f
                .debug_tuple("Struct")
                .field(&DebugStyle(*style))
                .field(&fields.iter().map(DebugField).collect::<Vec<_>>())
                .finish(),
        }
    }
}
