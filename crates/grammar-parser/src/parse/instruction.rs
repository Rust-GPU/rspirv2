use crate::parse::{Capability, Extension};
use smallvec::SmallVec;
use std::borrow::Cow;

/// See [`spirv_grammar::meta::InstMeta`]
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct InstMeta<'a> {
    #[serde(borrow)]
    pub opname: Cow<'a, str>,
    /// The name of the [`InstClass`], references `Grammar.instruction_printing_class`
    #[serde(borrow)]
    pub class: Option<Cow<'a, str>>,
    pub opcode: u16,
    #[serde(borrow, default)]
    pub operands: SmallVec<[OperandSpecMeta<'a>; 4]>,
    #[serde(borrow, default)]
    pub capabilities: SmallVec<[Capability<'a>; 2]>,
    #[serde(borrow, default)]
    pub extensions: SmallVec<[Extension<'a>; 1]>,
    #[serde(borrow, default)]
    pub version: Option<Cow<'a, str>>,
    #[serde(borrow, default, rename = "lastVersion")]
    pub last_version: Option<Cow<'a, str>>,
    #[serde(borrow, default)]
    pub aliases: SmallVec<[Cow<'a, str>; 1]>,
    #[serde(default)]
    pub provisional: bool,
}

/// See [`spirv_grammar::meta::OperandSpecMeta`]
#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct OperandSpecMeta<'a> {
    /// The name of the [`OperandKind`], references `Grammar.operand_kinds`
    #[serde(borrow)]
    pub kind: Cow<'a, str>,
    #[serde(borrow, default)]
    pub name: Option<Cow<'a, str>>,
    #[serde(default)]
    pub quantifier: Quantifier,
}

/// How many times to repeat something?
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Ord, PartialOrd, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Quantifier {
    #[default]
    #[serde(rename = "")]
    One,
    #[serde(rename = "?")]
    ZeroOrOne,
    #[serde(rename = "*")]
    ZeroOrMore,
}

/// See [`spirv_grammar::meta::InstClass`]
#[derive(Clone, Debug, serde::Deserialize)]
pub struct InstClass<'a> {
    #[serde(borrow)]
    pub tag: Cow<'a, str>,
    #[serde(borrow)]
    pub heading: Option<Cow<'a, str>>,
}

#[cfg(feature = "codegen")]
pub use codegen::*;

#[cfg(feature = "codegen")]
mod codegen {
    use super::*;
    use crate::codegen::{EmitRef, ident_ok, make_const_ident, ref_ident};
    use crate::parse::OperandKind;
    use convert_case::{Case, Casing};
    use proc_macro2::{Ident, TokenStream};
    use quote::{format_ident, quote};
    use std::collections::HashMap;

    impl InstMeta<'_> {
        pub fn type_ident(opname: &Cow<str>) -> Ident {
            format_ident!("{}", opname)
        }

        pub fn const_ident(opname: &str) -> Ident {
            make_const_ident("", opname)
        }

        pub fn emit_def(&self) -> TokenStream {
            let ident = Self::const_ident(&self.opname);
            let opname = self.opname.emit_ref();
            let class = match &self.class {
                None => quote!(None),
                Some(class) => {
                    let ident = InstClass::const_ident(class);
                    quote!(Some(&#ident))
                }
            };
            let opcode = self.opcode.emit_ref();
            let operands = self.operands.emit_ref();
            let capabilities = self.capabilities.emit_ref();
            let extensions = self.extensions.emit_ref();
            let version = self.version.emit_ref();
            let last_version = self.last_version.emit_ref();
            let aliases = self.aliases.emit_ref();
            let provisional = self.provisional.emit_ref();

            quote! {
                pub const #ident: InstMeta = InstMeta {
                    opname: #opname,
                    class: #class,
                    opcode: #opcode,
                    operands: #operands,
                    capabilities: #capabilities,
                    extensions: #extensions,
                    version: #version,
                    last_version: #last_version,
                    aliases: #aliases,
                    provisional: #provisional,
                };
            }
        }
    }

    impl EmitRef for InstMeta<'_> {
        fn emit_ref(&self) -> TokenStream {
            ref_ident(Self::const_ident(&self.opname))
        }
    }

    #[derive(Clone, Debug)]
    pub struct Operand<'a, 'b> {
        pub meta: &'a OperandSpecMeta<'b>,
        pub name: Ident,
        pub ty: Ident,
    }

    impl<'a> InstMeta<'a> {
        /// Computes a `Vec` of [`Operand`]s with valid member names.
        ///
        /// See [`OperandSpecMeta::member_name_proposal`] for member naming details.
        pub fn compute_operands(&self) -> Vec<Operand<'_, 'a>> {
            let mut proposed_names = self
                .operands
                .iter()
                .map(OperandSpecMeta::member_name_proposal)
                .collect::<Vec<_>>();

            // check for duplicate operand names
            let mut name_cnt = HashMap::new();
            for name in &proposed_names {
                name_cnt
                    .entry(name)
                    .and_modify(|cnt| *cnt += 1)
                    .or_insert(1u32);
            }
            name_cnt.retain(|_, v| *v > 1);
            if !name_cnt.is_empty() {
                // duplicate operand names detected, append `_{i++}`
                // reuse name_cnt for unique ID assignment
                name_cnt.iter_mut().for_each(|(_, v)| *v = 0);
                proposed_names = proposed_names
                    .iter()
                    .map(|name| {
                        if let Some(cnt) = name_cnt.get_mut(name) {
                            let rename = format!("{name}_{}", *cnt);
                            *cnt += 1;
                            rename
                        } else {
                            name.clone()
                        }
                    })
                    .collect();
            }

            self.operands
                .iter()
                .zip(proposed_names)
                .map(|(meta, name)| {
                    let name = format_ident!("{}", name);
                    let ty = OperandKind::type_ident(&meta.kind);
                    Operand { meta, name, ty }
                })
                .collect()
        }
    }

    impl OperandSpecMeta<'_> {
        /// *Proposes* a member name, the actual member name may differ! Use [`InstMeta::compute_operands`] to compute
        /// actual operand / member names.
        ///
        /// Some SPIR-V Instructions (like `OpCopyMemory`) have multiple Operands without names but the same `kind`,
        /// leading to this function returning equivalent Idents for those members. So we need a post pass that ensures
        /// the Idents are unique, to put them in a struct. This makes member naming depend on other members within
        /// the Instruction, making it not trivial to query member names.
        pub fn member_name_proposal(&self) -> String {
            // try to use `name`, but some names are nonsense Idents
            let name = if let Some(name) = &self.name {
                let name = name.to_case(Case::Snake);
                ident_ok(&name).then_some(name)
            } else {
                None
            };
            // otherwise, use `kind` name
            let name = name.unwrap_or_else(|| self.kind.to_case(Case::Snake));
            // keyword replacements
            let name = match name.as_str() {
                "type" => "ty".to_string(),
                "use" => "usage".to_string(),
                _ => name,
            };
            name
        }
    }

    impl EmitRef for OperandSpecMeta<'_> {
        fn emit_ref(&self) -> TokenStream {
            let kind = OperandKind::const_ident(&self.kind);
            let name = self.name.emit_ref();
            let quantifier = self.quantifier.emit_ref();
            quote! {
                OperandSpecMeta {
                    kind: &#kind,
                    name: #name,
                    quantifier: #quantifier,
                }
            }
        }
    }

    impl EmitRef for Quantifier {
        fn emit_ref(&self) -> TokenStream {
            match self {
                Quantifier::One => quote!(Quantifier::One),
                Quantifier::ZeroOrOne => quote!(Quantifier::ZeroOrOne),
                Quantifier::ZeroOrMore => quote!(Quantifier::ZeroOrMore),
            }
        }
    }

    impl InstClass<'_> {
        pub fn const_ident(tag: &str) -> Ident {
            make_const_ident("PRINTING_CLASS_", &tag)
        }

        pub fn emit_def(&self) -> TokenStream {
            let ident = Self::const_ident(&self.tag);
            let tag = self.tag.emit_ref();
            let heading = self.heading.emit_ref();
            quote! {
                pub const #ident: InstClass = InstClass {
                    tag: #tag,
                    heading: #heading,
                };
            }
        }
    }

    impl EmitRef for InstClass<'_> {
        fn emit_ref(&self) -> TokenStream {
            ref_ident(Self::const_ident(&self.tag))
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::parse::{InstMeta, OperandSpecMeta, Quantifier};
        use smallvec::SmallVec;
        use std::borrow::Cow;

        #[test]
        fn test_inst_operand_naming() {
            let test = |names: &[Option<&str>], expected: &[&str]| {
                let inst = InstMeta {
                    operands: SmallVec::from_iter(names.iter().map(|name| OperandSpecMeta {
                        name: name.map(Cow::from),
                        kind: Cow::Borrowed("testkind"),
                        quantifier: Quantifier::One,
                    })),
                    ..Default::default()
                };
                let names = inst
                    .compute_operands()
                    .iter()
                    .map(|i| i.name.to_string())
                    .collect::<Vec<_>>();
                assert_eq!(expected, names);
            };

            test(&[Some("a"), Some("b"), Some("c")], &["a", "b", "c"]);
            test(&[Some("a"), Some("a"), Some("a")], &["a_0", "a_1", "a_2"]);
            test(&[Some("a"), Some("b"), Some("a")], &["a_0", "b", "a_1"]);
            test(&[Some("a"), None, Some("a")], &["a_0", "testkind", "a_1"]);
            test(
                &[Some("a"), None, None, Some("a")],
                &["a_0", "testkind_0", "testkind_1", "a_1"],
            );
        }
    }
}
