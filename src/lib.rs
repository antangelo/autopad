use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::ext::IdentExt;
use syn::{braced, token, Field, Ident, Token, Attribute, Visibility, Type, LitInt, ItemStruct, Generics};

#[allow(dead_code)]
struct FieldWithPadding {
    attrs: Vec<Attribute>,
    offset: LitInt,
    equal_sign: Token![=],
    right_arrow: Token![>],
    vis: Visibility,
    ident: Option<Ident>,
    colon_token: Option<Token![:]>,
    ty: Type,
}

impl FieldWithPadding {
    fn into_field(self) -> Field {
        Field {
            attrs: self.attrs,
            vis: self.vis,
            ident: self.ident,
            colon_token: self.colon_token,
            ty: self.ty,
        }
    }
}

impl Parse for FieldWithPadding {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(FieldWithPadding {
            attrs: vec![],
            offset: input.parse()?,
            equal_sign: input.parse()?,
            right_arrow: input.parse()?,
            vis: input.parse()?,
            ident: Some(if input.peek(Token![_]) {
                input.call(Ident::parse_any)
            } else {
                input.parse()
            }?),
            colon_token: Some(input.parse()?),
            ty: input.parse()?,
        })
    }
}

enum PaddedStructField {
    NoPadding(Field),
    WithPadding(FieldWithPadding),
}

impl Parse for PaddedStructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let lookahead = input.lookahead1();
        if lookahead.peek(LitInt) {
            let mut field: FieldWithPadding = input.parse()?;
            field.attrs = attrs;
            Ok(PaddedStructField::WithPadding(field))
        } else {
            let mut field = Field::parse_named(input)?;
            field.attrs = attrs;
            Ok(PaddedStructField::NoPadding(field))
        }
    }
}

#[allow(dead_code)]
struct PaddedStruct {
    attrs: Vec<Attribute>,
    vis: Visibility,
    struct_token: Token![struct],
    ident: Ident,
    generics: Generics,
    brace: token::Brace,
    fields: Punctuated<PaddedStructField, Token![,]>,
    semi_token: Option<Token![;]>,
}

impl PaddedStruct {
    fn into_struct(self) -> ItemStruct {
        let mut fields = Punctuated::<Field, Token![,]>::new();

        for field in self.fields {
            match field {
                PaddedStructField::NoPadding(f) => fields.push(f),
                PaddedStructField::WithPadding(p) => {
                    fields.push(p.into_field());
                },
            }
        }

        let fields = syn::FieldsNamed { brace_token: self.brace, named: fields };
        ItemStruct { attrs: self.attrs, vis: self.vis, struct_token: self.struct_token, ident: self.ident, generics: self.generics, fields: syn::Fields::Named(fields), semi_token: self.semi_token }
    }
}

impl Parse for PaddedStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(PaddedStruct {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            ident: input.parse()?,
            generics: input.parse()?,
            brace: braced!(content in input),
            fields: content.parse_terminated(PaddedStructField::parse)?,
            semi_token: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn pad_struct(item: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(item as PaddedStruct);
    let padded_struct = parsed.into_struct();
    TokenStream::from(padded_struct.to_token_stream())
}
