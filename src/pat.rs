use super::*;
use crate::punctuated::Punctuated;
#[cfg(feature = "extra-traits")]
use crate::tt::TokenStreamHelper;
use proc_macro2::TokenStream;
#[cfg(feature = "extra-traits")]
use std::hash::{Hash, Hasher};

ast_enum_of_structs! {
    /// A pattern in a local binding, function signature, match expression, or
    /// various other places.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    //
    // TODO: change syntax-tree-enum link to an intra rustdoc link, currently
    // blocked on https://github.com/rust-lang/rust/issues/62833
    pub enum Pat #manual_extra_traits {
        /// A box pattern: `box v`.
        Box(PatBox),

        /// A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.
        Ident(PatIdent),

        /// A literal pattern: `0`.
        ///
        /// This holds an `Expr` rather than a `Lit` because negative numbers
        /// are represented as an `Expr::Unary`.
        Lit(PatLit),

        /// A macro in pattern position.
        Macro(PatMacro),

        /// A pattern that matches any one of a set of cases.
        Or(PatOr),

        /// A path pattern like `Color::Red`, optionally qualified with a
        /// self-type.
        ///
        /// Unqualified path patterns can legally refer to variants, structs,
        /// constants or associated constants. Qualified path patterns like
        /// `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
        /// associated constants.
        Path(PatPath),

        /// A range pattern: `1..=2`.
        Range(PatRange),

        /// A reference pattern: `&mut var`.
        Reference(PatReference),

        /// The dots in a tuple or slice pattern: `[0, 1, ..]`
        Rest(PatRest),

        /// A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.
        Slice(PatSlice),

        /// A struct or struct variant pattern: `Variant { x, y, .. }`.
        Struct(PatStruct),

        /// A tuple pattern: `(a, b)`.
        Tuple(PatTuple),

        /// A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
        TupleStruct(PatTupleStruct),

        /// A type ascription pattern: `foo: f64`.
        Type(PatType),

        /// Tokens in pattern position not interpreted by Syn.
        Verbatim(TokenStream),

        /// A pattern that matches any value: `_`.
        Wild(PatWild),

        #[doc(hidden)]
        __Nonexhaustive,
    }
}

ast_struct! {
    /// A box pattern: `box v`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatBox {
        pub attrs: Vec<Attribute>,
        pub box_token: Token![box],
        pub pat: Box<Pat>,
    }
}

ast_struct! {
    /// A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatIdent {
        pub attrs: Vec<Attribute>,
        pub by_ref: Option<Token![ref]>,
        pub mutability: Option<Token![mut]>,
        pub ident: Ident,
        pub subpat: Option<(Token![@], Box<Pat>)>,
    }
}

ast_struct! {
    /// A literal pattern: `0`.
    ///
    /// This holds an `Expr` rather than a `Lit` because negative numbers
    /// are represented as an `Expr::Unary`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatLit {
        pub attrs: Vec<Attribute>,
        pub expr: Box<Expr>,
    }
}

ast_struct! {
    /// A macro in pattern position.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatMacro {
        pub attrs: Vec<Attribute>,
        pub mac: Macro,
    }
}

ast_struct! {
    /// A pattern that matches any one of a set of cases.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatOr {
        pub attrs: Vec<Attribute>,
        pub leading_vert: Option<Token![|]>,
        pub cases: Punctuated<Pat, Token![|]>,
    }
}

ast_struct! {
    /// A path pattern like `Color::Red`, optionally qualified with a
    /// self-type.
    ///
    /// Unqualified path patterns can legally refer to variants, structs,
    /// constants or associated constants. Qualified path patterns like
    /// `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
    /// associated constants.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatPath {
        pub attrs: Vec<Attribute>,
        pub qself: Option<QSelf>,
        pub path: Path,
    }
}

ast_struct! {
    /// A range pattern: `1..=2`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatRange {
        pub attrs: Vec<Attribute>,
        pub lo: Box<Expr>,
        pub limits: RangeLimits,
        pub hi: Box<Expr>,
    }
}

ast_struct! {
    /// A reference pattern: `&mut var`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatReference {
        pub attrs: Vec<Attribute>,
        pub and_token: Token![&],
        pub mutability: Option<Token![mut]>,
        pub pat: Box<Pat>,
    }
}

ast_struct! {
    /// The dots in a tuple or slice pattern: `[0, 1, ..]`
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatRest {
        pub attrs: Vec<Attribute>,
        pub dot2_token: Token![..],
    }
}

ast_struct! {
    /// A dynamically sized slice pattern: `[a, b, ref i @ .., y, z]`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatSlice {
        pub attrs: Vec<Attribute>,
        pub bracket_token: token::Bracket,
        pub elems: Punctuated<Pat, Token![,]>,
    }
}

ast_struct! {
    /// A struct or struct variant pattern: `Variant { x, y, .. }`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatStruct {
        pub attrs: Vec<Attribute>,
        pub path: Path,
        pub brace_token: token::Brace,
        pub fields: Punctuated<FieldPat, Token![,]>,
        pub dot2_token: Option<Token![..]>,
    }
}

ast_struct! {
    /// A tuple pattern: `(a, b)`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatTuple {
        pub attrs: Vec<Attribute>,
        pub paren_token: token::Paren,
        pub elems: Punctuated<Pat, Token![,]>,
    }
}

ast_struct! {
    /// A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatTupleStruct {
        pub attrs: Vec<Attribute>,
        pub path: Path,
        pub pat: PatTuple,
    }
}

ast_struct! {
    /// A type ascription pattern: `foo: f64`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatType {
        pub attrs: Vec<Attribute>,
        pub pat: Box<Pat>,
        pub colon_token: Token![:],
        pub ty: Box<Type>,
    }
}

ast_struct! {
    /// A pattern that matches any value: `_`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct PatWild {
        pub attrs: Vec<Attribute>,
        pub underscore_token: Token![_],
    }
}

ast_struct! {
    /// A single field in a struct pattern.
    ///
    /// Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
    /// the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct FieldPat {
        pub attrs: Vec<Attribute>,
        pub member: Member,
        pub colon_token: Option<Token![:]>,
        pub pat: Box<Pat>,
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for Pat {}

#[cfg(feature = "extra-traits")]
impl PartialEq for Pat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pat::Box(this), Pat::Box(other)) => this == other,
            (Pat::Ident(this), Pat::Ident(other)) => this == other,
            (Pat::Lit(this), Pat::Lit(other)) => this == other,
            (Pat::Macro(this), Pat::Macro(other)) => this == other,
            (Pat::Or(this), Pat::Or(other)) => this == other,
            (Pat::Path(this), Pat::Path(other)) => this == other,
            (Pat::Range(this), Pat::Range(other)) => this == other,
            (Pat::Reference(this), Pat::Reference(other)) => this == other,
            (Pat::Rest(this), Pat::Rest(other)) => this == other,
            (Pat::Slice(this), Pat::Slice(other)) => this == other,
            (Pat::Struct(this), Pat::Struct(other)) => this == other,
            (Pat::Tuple(this), Pat::Tuple(other)) => this == other,
            (Pat::TupleStruct(this), Pat::TupleStruct(other)) => this == other,
            (Pat::Type(this), Pat::Type(other)) => this == other,
            (Pat::Verbatim(this), Pat::Verbatim(other)) => {
                TokenStreamHelper(this) == TokenStreamHelper(other)
            }
            (Pat::Wild(this), Pat::Wild(other)) => this == other,
            _ => false,
        }
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for Pat {
    fn hash<H>(&self, hash: &mut H)
    where
        H: Hasher,
    {
        match self {
            Pat::Box(pat) => {
                hash.write_u8(0);
                pat.hash(hash);
            }
            Pat::Ident(pat) => {
                hash.write_u8(1);
                pat.hash(hash);
            }
            Pat::Lit(pat) => {
                hash.write_u8(2);
                pat.hash(hash);
            }
            Pat::Macro(pat) => {
                hash.write_u8(3);
                pat.hash(hash);
            }
            Pat::Or(pat) => {
                hash.write_u8(4);
                pat.hash(hash);
            }
            Pat::Path(pat) => {
                hash.write_u8(5);
                pat.hash(hash);
            }
            Pat::Range(pat) => {
                hash.write_u8(6);
                pat.hash(hash);
            }
            Pat::Reference(pat) => {
                hash.write_u8(7);
                pat.hash(hash);
            }
            Pat::Rest(pat) => {
                hash.write_u8(8);
                pat.hash(hash);
            }
            Pat::Slice(pat) => {
                hash.write_u8(9);
                pat.hash(hash);
            }
            Pat::Struct(pat) => {
                hash.write_u8(10);
                pat.hash(hash);
            }
            Pat::Tuple(pat) => {
                hash.write_u8(11);
                pat.hash(hash);
            }
            Pat::TupleStruct(pat) => {
                hash.write_u8(12);
                pat.hash(hash);
            }
            Pat::Type(pat) => {
                hash.write_u8(13);
                pat.hash(hash);
            }
            Pat::Verbatim(pat) => {
                hash.write_u8(14);
                TokenStreamHelper(pat).hash(hash);
            }
            Pat::Wild(pat) => {
                hash.write_u8(15);
                pat.hash(hash);
            }
            Pat::__Nonexhaustive => unreachable!(),
        }
    }
}

#[cfg(feature = "parsing")]
mod parsing {
    use super::*;

    use crate::ext::IdentExt;
    use crate::parse::{Parse, ParseStream, Result};
    use crate::path;

    impl Parse for Pat {
        fn parse(input: ParseStream) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident)
                && ({
                    input.peek2(Token![::])
                        || input.peek2(Token![!])
                        || input.peek2(token::Brace)
                        || input.peek2(token::Paren)
                        || input.peek2(Token![..])
                            && !{
                                let ahead = input.fork();
                                ahead.parse::<Ident>()?;
                                ahead.parse::<RangeLimits>()?;
                                ahead.is_empty() || ahead.peek(Token![,])
                            }
                })
                || input.peek(Token![self]) && input.peek2(Token![::])
                || lookahead.peek(Token![::])
                || lookahead.peek(Token![<])
                || input.peek(Token![Self])
                || input.peek(Token![super])
                || input.peek(Token![extern])
                || input.peek(Token![crate])
            {
                pat_path_or_macro_or_struct_or_range(input)
            } else if lookahead.peek(Token![_]) {
                input.call(pat_wild).map(Pat::Wild)
            } else if input.peek(Token![box]) {
                input.call(pat_box).map(Pat::Box)
            } else if input.peek(Token![-]) || lookahead.peek(Lit) {
                pat_lit_or_range(input)
            } else if lookahead.peek(Token![ref])
                || lookahead.peek(Token![mut])
                || input.peek(Token![self])
                || input.peek(Ident)
            {
                input.call(pat_ident).map(Pat::Ident)
            } else if lookahead.peek(Token![&]) {
                input.call(pat_reference).map(Pat::Reference)
            } else if lookahead.peek(token::Paren) {
                input.call(pat_tuple).map(Pat::Tuple)
            } else if lookahead.peek(token::Bracket) {
                input.call(pat_slice).map(Pat::Slice)
            } else if lookahead.peek(Token![..]) && !input.peek(Token![...]) {
                input.call(pat_rest).map(Pat::Rest)
            } else {
                Err(lookahead.error())
            }
        }
    }

    fn pat_path_or_macro_or_struct_or_range(input: ParseStream) -> Result<Pat> {
        let (qself, path) = path::parsing::qpath(input, true)?;

        if input.peek(Token![..]) {
            return pat_range(input, qself, path).map(Pat::Range);
        }

        if qself.is_some() {
            return Ok(Pat::Path(PatPath {
                attrs: Vec::new(),
                qself,
                path,
            }));
        }

        if input.peek(Token![!]) && !input.peek(Token![!=]) {
            let mut contains_arguments = false;
            for segment in &path.segments {
                match segment.arguments {
                    PathArguments::None => {}
                    PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                        contains_arguments = true;
                    }
                }
            }

            if !contains_arguments {
                let bang_token: Token![!] = input.parse()?;
                let (delimiter, tokens) = mac::parse_delimiter(input)?;
                return Ok(Pat::Macro(PatMacro {
                    attrs: Vec::new(),
                    mac: Macro {
                        path,
                        bang_token,
                        delimiter,
                        tokens,
                    },
                }));
            }
        }

        if input.peek(token::Brace) {
            pat_struct(input, path).map(Pat::Struct)
        } else if input.peek(token::Paren) {
            pat_tuple_struct(input, path).map(Pat::TupleStruct)
        } else if input.peek(Token![..]) {
            pat_range(input, qself, path).map(Pat::Range)
        } else {
            Ok(Pat::Path(PatPath {
                attrs: Vec::new(),
                qself,
                path,
            }))
        }
    }

    fn pat_wild(input: ParseStream) -> Result<PatWild> {
        Ok(PatWild {
            attrs: Vec::new(),
            underscore_token: input.parse()?,
        })
    }

    fn pat_box(input: ParseStream) -> Result<PatBox> {
        Ok(PatBox {
            attrs: Vec::new(),
            box_token: input.parse()?,
            pat: input.parse()?,
        })
    }

    fn pat_ident(input: ParseStream) -> Result<PatIdent> {
        Ok(PatIdent {
            attrs: Vec::new(),
            by_ref: input.parse()?,
            mutability: input.parse()?,
            ident: input.call(Ident::parse_any)?,
            subpat: {
                if input.peek(Token![@]) {
                    let at_token: Token![@] = input.parse()?;
                    let subpat: Pat = input.parse()?;
                    Some((at_token, Box::new(subpat)))
                } else {
                    None
                }
            },
        })
    }

    fn pat_tuple_struct(input: ParseStream, path: Path) -> Result<PatTupleStruct> {
        Ok(PatTupleStruct {
            attrs: Vec::new(),
            path,
            pat: input.call(pat_tuple)?,
        })
    }

    fn pat_struct(input: ParseStream, path: Path) -> Result<PatStruct> {
        let content;
        let brace_token = braced!(content in input);

        let mut fields = Punctuated::new();
        while !content.is_empty() && !content.peek(Token![..]) {
            let value = content.call(field_pat)?;
            fields.push_value(value);
            if !content.peek(Token![,]) {
                break;
            }
            let punct: Token![,] = content.parse()?;
            fields.push_punct(punct);
        }

        let dot2_token = if fields.empty_or_trailing() && content.peek(Token![..]) {
            Some(content.parse()?)
        } else {
            None
        };

        Ok(PatStruct {
            attrs: Vec::new(),
            path,
            brace_token,
            fields,
            dot2_token,
        })
    }

    impl Member {
        fn is_unnamed(&self) -> bool {
            match *self {
                Member::Named(_) => false,
                Member::Unnamed(_) => true,
            }
        }
    }

    fn field_pat(input: ParseStream) -> Result<FieldPat> {
        let boxed: Option<Token![box]> = input.parse()?;
        let by_ref: Option<Token![ref]> = input.parse()?;
        let mutability: Option<Token![mut]> = input.parse()?;
        let member: Member = input.parse()?;

        if boxed.is_none() && by_ref.is_none() && mutability.is_none() && input.peek(Token![:])
            || member.is_unnamed()
        {
            return Ok(FieldPat {
                attrs: Vec::new(),
                member,
                colon_token: input.parse()?,
                pat: input.parse()?,
            });
        }

        let ident = match member {
            Member::Named(ident) => ident,
            Member::Unnamed(_) => unreachable!(),
        };

        let mut pat = Pat::Ident(PatIdent {
            attrs: Vec::new(),
            by_ref,
            mutability,
            ident: ident.clone(),
            subpat: None,
        });

        if let Some(boxed) = boxed {
            pat = Pat::Box(PatBox {
                attrs: Vec::new(),
                pat: Box::new(pat),
                box_token: boxed,
            });
        }

        Ok(FieldPat {
            member: Member::Named(ident),
            pat: Box::new(pat),
            attrs: Vec::new(),
            colon_token: None,
        })
    }

    fn pat_range(input: ParseStream, qself: Option<QSelf>, path: Path) -> Result<PatRange> {
        Ok(PatRange {
            attrs: Vec::new(),
            lo: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself,
                path,
            })),
            limits: input.parse()?,
            hi: input.call(pat_lit_expr)?,
        })
    }

    fn pat_tuple(input: ParseStream) -> Result<PatTuple> {
        let content;
        let paren_token = parenthesized!(content in input);

        let mut elems = Punctuated::new();
        while !content.is_empty() {
            let value: Pat = content.parse()?;
            elems.push_value(value);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            elems.push_punct(punct);
        }

        Ok(PatTuple {
            attrs: Vec::new(),
            paren_token,
            elems,
        })
    }

    fn pat_reference(input: ParseStream) -> Result<PatReference> {
        Ok(PatReference {
            attrs: Vec::new(),
            and_token: input.parse()?,
            mutability: input.parse()?,
            pat: input.parse()?,
        })
    }

    fn pat_lit_or_range(input: ParseStream) -> Result<Pat> {
        let lo = input.call(pat_lit_expr)?;
        if input.peek(Token![..]) {
            Ok(Pat::Range(PatRange {
                attrs: Vec::new(),
                lo,
                limits: input.parse()?,
                hi: input.call(pat_lit_expr)?,
            }))
        } else {
            Ok(Pat::Lit(PatLit {
                attrs: Vec::new(),
                expr: lo,
            }))
        }
    }

    fn pat_lit_expr(input: ParseStream) -> Result<Box<Expr>> {
        let neg: Option<Token![-]> = input.parse()?;

        let lookahead = input.lookahead1();
        let expr = if lookahead.peek(Lit) {
            Expr::Lit(input.parse()?)
        } else if lookahead.peek(Ident)
            || lookahead.peek(Token![::])
            || lookahead.peek(Token![<])
            || lookahead.peek(Token![self])
            || lookahead.peek(Token![Self])
            || lookahead.peek(Token![super])
            || lookahead.peek(Token![extern])
            || lookahead.peek(Token![crate])
        {
            Expr::Path(input.parse()?)
        } else {
            return Err(lookahead.error());
        };

        Ok(Box::new(if let Some(neg) = neg {
            Expr::Unary(ExprUnary {
                attrs: Vec::new(),
                op: UnOp::Neg(neg),
                expr: Box::new(expr),
            })
        } else {
            expr
        }))
    }

    fn pat_slice(input: ParseStream) -> Result<PatSlice> {
        let content;
        let bracket_token = bracketed!(content in input);

        let mut elems = Punctuated::new();
        while !content.is_empty() {
            let value: Pat = content.parse()?;
            elems.push_value(value);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            elems.push_punct(punct);
        }

        Ok(PatSlice {
            attrs: Vec::new(),
            bracket_token,
            elems,
        })
    }

    fn pat_rest(input: ParseStream) -> Result<PatRest> {
        Ok(PatRest {
            attrs: Vec::new(),
            dot2_token: input.parse()?,
        })
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;

    use proc_macro2::TokenStream;
    use quote::{ToTokens, TokenStreamExt};

    use crate::attr::FilterAttrs;

    impl ToTokens for PatWild {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.underscore_token.to_tokens(tokens);
        }
    }

    impl ToTokens for PatIdent {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.by_ref.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            if let Some((at_token, subpat)) = &self.subpat {
                at_token.to_tokens(tokens);
                subpat.to_tokens(tokens);
            }
        }
    }

    impl ToTokens for PatStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.path.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                self.fields.to_tokens(tokens);
                // NOTE: We need a comma before the dot2 token if it is present.
                if !self.fields.empty_or_trailing() && self.dot2_token.is_some() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                self.dot2_token.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for PatTupleStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.path.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    impl ToTokens for PatType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.pat.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
        }
    }

    impl ToTokens for PatPath {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            private::print_path(tokens, &self.qself, &self.path);
        }
    }

    impl ToTokens for PatTuple {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.paren_token.surround(tokens, |tokens| {
                self.elems.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for PatBox {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.box_token.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    impl ToTokens for PatReference {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.and_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    impl ToTokens for PatRest {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.dot2_token.to_tokens(tokens);
        }
    }

    impl ToTokens for PatLit {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.expr.to_tokens(tokens);
        }
    }

    impl ToTokens for PatRange {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.lo.to_tokens(tokens);
            match &self.limits {
                RangeLimits::HalfOpen(t) => t.to_tokens(tokens),
                RangeLimits::Closed(t) => t.to_tokens(tokens),
            }
            self.hi.to_tokens(tokens);
        }
    }

    impl ToTokens for PatSlice {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.bracket_token.surround(tokens, |tokens| {
                self.elems.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for PatMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.mac.to_tokens(tokens);
        }
    }

    impl ToTokens for PatOr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            self.leading_vert.to_tokens(tokens);
            self.cases.to_tokens(tokens);
        }
    }

    impl ToTokens for FieldPat {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            if let Some(colon_token) = &self.colon_token {
                self.member.to_tokens(tokens);
                colon_token.to_tokens(tokens);
            }
            self.pat.to_tokens(tokens);
        }
    }
}
