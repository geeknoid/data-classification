//! Exposes a macro to generate data taxonomies.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, parse2};

type SynResult<T> = Result<T, syn::Error>;

struct MacroArgs {
    taxonomy_name: Ident,
    generate_serde: bool,
}

impl MacroArgs {
    pub fn parse(attr_args: TokenStream) -> SynResult<Self> {
        if attr_args.is_empty() {
            Err(syn::Error::new(
                attr_args.span(),
                "taxonomy attribute requires a taxonomy name argument",
            ))
        } else {
            parse2(attr_args)
        }
    }
}

impl Parse for MacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let taxonomy_name: Ident = input.parse()?;

        let generate_serde = if input.peek(syn::token::Comma) {
            _ = input.parse::<syn::token::Comma>()?;
            let ident = input.parse::<Ident>()?;
            if ident != "serde" {
                return Err(syn::Error::new(input.span(), "expected `serde`"));
            }

            _ = input.parse::<syn::token::Eq>()?;
            input.parse::<syn::LitBool>()?.value
        } else {
            true
        };

        Ok(Self {
            taxonomy_name,
            generate_serde,
        })
    }
}

/// Convert `PascalCase` to `snake_case`
fn pascal_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(*ch);
        }
    }

    result
}

/// Determine the path to the `data-classification` crate
#[cfg(not(test))]
fn find_crate(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let found_crate = proc_macro_crate::crate_name("data-classification")
        .map_err(|e| syn::Error::new(input.ident.span(), e))?;

    Ok(match found_crate {
        proc_macro_crate::FoundCrate::Itself => quote!(crate),
        proc_macro_crate::FoundCrate::Name(name) => {
            let ident = quote::format_ident!("{}", name);
            quote!(::#ident)
        }
    })
}

#[allow(clippy::too_many_lines, reason = "Yeah, it's a bit much...")]
fn taxonomy_impl(attr_args: TokenStream, item: TokenStream) -> SynResult<TokenStream> {
    let macro_args = MacroArgs::parse(attr_args)?;
    let input: DeriveInput = parse2(item)?;

    let Data::Enum(enum_data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input,
            "taxonomy attribute can only be applied to enums",
        ));
    };

    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "taxonomy attribute cannot be applied to generic enums",
        ));
    }

    #[cfg(test)]
    let data_classification_path = quote!(crate);

    #[cfg(not(test))]
    let data_classification_path = find_crate(&input)?;

    let enum_name = &input.ident;
    let enum_vis = &input.vis;

    let mut variant_structs = Vec::new();
    let mut match_arms = Vec::new();

    for variant in &enum_data.variants {
        match &variant.fields {
            Fields::Unit => {}
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "taxonomy attribute only supports unit variants",
                ));
            }
        }

        let variant_name = &variant.ident;
        let variant_name_str = variant_name.to_string();
        let snake_case_name = pascal_to_snake_case(&variant_name.to_string());

        let serde_impls = if macro_args.generate_serde {
            quote! {
                impl<'a, T> serde::Deserialize<'a> for #variant_name<T>
                where
                    T: serde::Deserialize<'a>,
                {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'a>,
                    {
                        let payload = T::deserialize(deserializer)?;
                        core::result::Result::Ok(Self::new(payload))
                    }
                }

                impl<T> serde::Serialize for #variant_name<T>
                where
                    T: serde::Serialize,
                {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        self.payload.serialize(serializer)
                    }
                }
            }
        } else {
            quote! {}
        };

        let taxonomy_name = macro_args.taxonomy_name.to_string();
        variant_structs.push(quote! {
            #[doc = concat!("A classified data container for the `", #snake_case_name, "` class of the `", #taxonomy_name, "` taxonomy.")]
            #[derive(Clone, Default, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #enum_vis struct #variant_name<T> {
                payload: T,
            }

            impl<T> #variant_name<T> {
                /// Creates a new instance of the classified data container.
                #[must_use]
                pub fn new(payload: T) -> Self {
                    Self { payload }
                }

                /// Exfiltrates the payload, allowing it to be used outside the classified context.
                ///
                /// Exfiltration should be done with caution, as it may expose sensitive information.
                ///
                /// # Returns
                /// The original payload.
                #[must_use]
                pub fn declassify(self) -> T {
                    self.payload
                }

                /// Returns the data class of the payload.
                #[must_use]
                pub const fn data_class() -> #data_classification_path::DataClass {
                    #data_classification_path::DataClass::new(#taxonomy_name, #snake_case_name)
                }
            }

            impl<T> #data_classification_path::Extract for #variant_name<T>
            where
                T: core::fmt::Display,
            {
                fn extract(&self, extractor: #data_classification_path::Extractor) {
                    extractor.write_str(
                        &Self::data_class(),
                        self.payload.to_string().as_str(),
                    )
                }
            }

            impl<T> #data_classification_path::Classified<T> for #variant_name<T> {
                fn declassify(self) -> T {
                    Self::declassify(self)
                }

                fn visit(&self, operation: impl FnOnce(&T)) {
                    operation(&self.payload);
                }

                fn visit_mut(&mut self, operation: impl FnOnce(&mut T)) {
                    operation(&mut self.payload);
                }

                fn data_class() -> #data_classification_path::DataClass {
                    Self::data_class()
                }
            }

            impl<T> core::fmt::Display for #variant_name<T>
            where
                T: core::fmt::Display,
            {
                #[expect(clippy::string_slice, reason = "No problem with UTF-8 here")]
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    static ASTERISKS: &str = "********************************";

                    let len = self.payload.to_string().len();
                    if len < ASTERISKS.len() {
                        f.write_fmt(::core::format_args!("{0}<{1}>", #variant_name_str, &ASTERISKS[0..len]))
                    } else {
                        f.write_fmt(::core::format_args!("{0}<{1}>", #variant_name_str, "*".repeat(len)))
                    }
                }
            }

            impl<T> core::fmt::Debug for #variant_name<T>
            where
                T: core::fmt::Debug,
            {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_fmt(::core::format_args!("{}(...)", #variant_name_str))
                }
            }

            impl<T> core::convert::From<T> for #variant_name<T> {
                fn from(payload: T) -> Self {
                    Self::new(payload)
                }
            }

            #serde_impls
        });

        match_arms.push(quote! {
            #enum_name::#variant_name => #data_classification_path::DataClass::new(#taxonomy_name, #snake_case_name)
        });
    }

    Ok(quote! {
        #input

        impl #enum_name {
            /// Returns the data class associated with the current variant.
            #[must_use]
            pub fn data_class(&self) -> #data_classification_path::DataClass {
                match self {
                    #(#match_arms),*
                }
            }
        }

        #(#variant_structs)*
    })
}

/// Placeholder docs, real ones in `data_classification` crate.
#[proc_macro_attribute]
pub fn taxonomy(
    attr_args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    taxonomy_impl(attr_args.into(), item.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_to_snake_case() {
        assert_eq!(pascal_to_snake_case("PascalCase"), "pascal_case");
        assert_eq!(pascal_to_snake_case("AnotherExample"), "another_example");
        assert_eq!(pascal_to_snake_case("Simple"), "simple");
        assert_eq!(pascal_to_snake_case("WithNumbers123"), "with_numbers123");
    }

    #[test]
    fn test_taxonomy_impl_empty_args() {
        let input = quote! {
            pub enum MyEnum {
                VariantOne,
                VariantTwo,
            }
        };

        let attr_args = quote! {};
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                .contains("taxonomy attribute requires a taxonomy name argument")
        );
    }

    #[test]
    fn test_taxonomy_impl_invalid_taxonomy_name() {
        let input = quote! {
            pub enum MyEnum {
                VariantOne,
                VariantTwo,
            }
        };

        let attr_args = quote! { "InvalidName" };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!("expected identifier", err.to_string());
    }

    #[test]
    fn test_taxonomy_impl_missing_comma() {
        let input = quote! {
            pub enum MyEnum {
                VariantOne,
                VariantTwo,
            }
        };

        let attr_args = quote! { MyTaxonomy serde = true };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!("unexpected token", err.to_string());
    }

    #[test]
    fn test_taxonomy_impl_unknown_parameter() {
        let input = quote! {
            pub enum MyEnum {
                VariantOne,
                VariantTwo,
            }
        };

        let attr_args = quote! { MyTaxonomy, unknown = true };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!("expected `serde`", err.to_string());
    }

    #[test]
    fn test_taxonomy_impl_non_enum_struct() {
        let input = quote! {
            pub struct MyStruct {
                field: i32,
            }
        };

        let attr_args = quote! { MyTaxonomy };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                .contains("taxonomy attribute can only be applied to enums")
        );
    }

    #[test]
    fn test_taxonomy_impl_generic_enum() {
        let input = quote! {
            pub enum MyEnum<T> {
                VariantOne(T),
                VariantTwo,
            }
        };

        let attr_args = quote! { MyTaxonomy };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                .contains("taxonomy attribute cannot be applied to generic enums")
        );
    }

    #[test]
    fn test_taxonomy_impl_non_unit_variant_named() {
        let input = quote! {
            pub enum MyEnum {
                VariantOne { field: i32 },
                VariantTwo,
            }
        };

        let attr_args = quote! { MyTaxonomy };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("only supports unit variants"));
    }

    #[test]
    fn test_taxonomy_impl_non_unit_variant_unnamed() {
        let input = quote! {
            pub enum MyEnum {
                VariantOne(i32),
                VariantTwo,
            }
        };

        let attr_args = quote! { MyTaxonomy };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("only supports unit variants"));
    }

    #[test]
    fn test_taxonomy_impl_invalid_syn_parse() {
        let input = quote! {
            invalid rust syntax here
        };

        let attr_args = quote! { MyTaxonomy };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            "expected one of: `struct`, `enum`, `union`",
            err.to_string()
        );
    }

    #[test]
    fn test_taxonomy_impl_serde_without_value() {
        let input = quote! {
            pub enum MyEnum {
                VariantOne,
                VariantTwo,
            }
        };

        let attr_args = quote! { MyTaxonomy, serde };
        let result = taxonomy_impl(attr_args, input);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!("expected `=`", err.to_string());
    }
}
