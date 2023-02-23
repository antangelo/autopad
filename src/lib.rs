use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{
    braced, parse_macro_input, parse_quote, token, Attribute, Expr, ExprBinary, Field, Generics,
    Ident, ItemStruct, LitInt, Token, Type, Visibility,
};

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

fn generate_padding_name(count: u64) -> String {
    format!("_pad_struct_padding_array{}", count)
}

fn generate_padding(
    count: u64,
    offset: usize,
    last_offset: usize,
    prev_types: &Vec<Type>,
) -> Field {
    let name = generate_padding_name(count);

    // Don't know anything about the type sizes here,
    // but we can at least sanity check this
    assert!(
        offset > last_offset,
        "Requested offset is less than current field position"
    );
    let offset = ExprBinary {
        attrs: vec![],
        left: Box::new(parse_quote!(#offset)),
        op: syn::BinOp::Sub(parse_quote!(-)),
        right: Box::new(parse_quote!(#last_offset)),
    };

    let pad_size = prev_types
        .iter()
        .map(|ty| parse_quote!(core::mem::size_of::<#ty>()))
        .fold(offset, |init, sz| ExprBinary {
            attrs: vec![],
            left: Box::new(Expr::Binary(init)),
            op: syn::BinOp::Sub(parse_quote!(-)),
            right: Box::new(sz),
        });

    let ident = Ident::new(&name, Span::call_site());
    let pad_type = parse_quote!([u8; #pad_size]);
    Field {
        attrs: vec![],
        vis: Visibility::Inherited,
        ident: Some(ident),
        colon_token: parse_quote!(:),
        ty: pad_type,
    }
}

impl PaddedStruct {
    fn into_struct(self) -> ItemStruct {
        let mut fields = Punctuated::<Field, Token![,]>::new();

        let mut pad_field_types: Vec<Type> = Vec::new();
        let mut pad_last_offset: usize = 0x0;
        let mut pad_count = 0;

        for field in self.fields {
            match field {
                PaddedStructField::NoPadding(f) => {
                    pad_field_types.push(f.ty.clone());
                    fields.push(f);
                }
                PaddedStructField::WithPadding(p) => {
                    let offset = p.offset.base10_parse::<usize>().unwrap();
                    let padding =
                        generate_padding(pad_count, offset, pad_last_offset, &pad_field_types);

                    fields.push(padding);
                    pad_count += 1;
                    pad_field_types.clear();
                    pad_field_types.push(p.ty.clone());
                    pad_last_offset = offset;
                    fields.push(p.into_field());
                }
            }
        }

        let fields = syn::FieldsNamed {
            brace_token: self.brace,
            named: fields,
        };
        ItemStruct {
            attrs: self.attrs,
            vis: self.vis,
            struct_token: self.struct_token,
            ident: self.ident,
            generics: self.generics,
            fields: syn::Fields::Named(fields),
            semi_token: self.semi_token,
        }
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
    let parsed = parse_macro_input!(item as PaddedStruct);
    let padded_struct = parsed.into_struct();
    let ts = TokenStream::from(padded_struct.to_token_stream());
    ts
}
