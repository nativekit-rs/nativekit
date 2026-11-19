#![allow(non_snake_case)]
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum, ItemFn, ReturnType, Type};

#[proc_macro_attribute]
pub fn prop(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_prop_enum(attr, item)
}

#[proc_macro_attribute]
pub fn props(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_prop_enum(attr, item)
}

fn expand_prop_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_enum = parse_macro_input!(item as ItemEnum);
    let enum_name = &input_enum.ident;
    let vis = &input_enum.vis;
    let attrs = &input_enum.attrs;
    let opts_struct_ident = Ident::new(&format!("__NkPropOpts_{}", enum_name), enum_name.span());

    let mut clean_variants = Vec::new();
    let mut opt_fields = Vec::new();
    let mut opt_inits = Vec::new();
    let mut opt_methods = Vec::new();
    let mut from_str_arms = Vec::new();
    let mut as_str_arms = Vec::new();
    let mut extra_from_impls = Vec::new();
    let mut default_expr: Option<TokenStream2> = None;

    for v in &input_enum.variants {
        let v_ident = &v.ident;
        let v_str = v_ident.to_string();
        if !is_valid_snake_case_ident(&v_str) {
            let err_msg = format!(
                "#[prop] enum variant `{}` on `{}` must be snake_case (e.g. `{}`).",
                v_str,
                enum_name,
                v_str.to_ascii_lowercase()
            );
            return TokenStream::from(
                quote::quote_spanned! { v_ident.span() => compile_error!(#err_msg); },
            );
        }
        let is_default_attr = v.attrs.iter().any(|a| a.path().is_ident("default"));

        match &v.fields {
            Fields::Unit => {
                if is_default_attr || default_expr.is_none() {
                    default_expr = Some(quote! { Self::#v_ident });
                }
                clean_variants.push(quote! { #v_ident });
                opt_fields.push(quote! { pub #v_ident: #enum_name });
                opt_inits.push(quote! { #v_ident: #enum_name::#v_ident });
                from_str_arms.push(quote! { #v_str => #enum_name::#v_ident, });
                as_str_arms.push(quote! { #enum_name::#v_ident => #v_str.to_string(), });
            }
            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                let raw_inner_ty = unnamed.unnamed.first().unwrap().ty.clone();
                let inner_ty = normalize_friendly_type(raw_inner_ty);
                if is_default_attr || default_expr.is_none() {
                    default_expr = Some(quote! { Self::#v_ident(::std::default::Default::default()) });
                }
                clean_variants.push(quote! { #v_ident(#inner_ty) });
                opt_methods.push(quote! {
                    pub fn #v_ident(self, val: impl ::std::convert::Into<#inner_ty>) -> #enum_name {
                        #enum_name::#v_ident(val.into())
                    }
                });
                as_str_arms.push(quote! { #enum_name::#v_ident(ref v) => format!("{:?}", v), });

                let custom_ty_name = if let Type::Path(tp) = &inner_ty {
                    tp.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
                } else {
                    String::new()
                };

                match custom_ty_name.as_str() {
                    "u32" => {
                        extra_from_impls.push(quote! {
                            impl ::std::convert::From<u32> for #enum_name {
                                fn from(v: u32) -> Self { Self::#v_ident(v) }
                            }
                            impl ::std::convert::From<i32> for #enum_name {
                                fn from(v: i32) -> Self { Self::#v_ident(v.max(0) as u32) }
                            }
                            impl ::std::convert::From<usize> for #enum_name {
                                fn from(v: usize) -> Self { Self::#v_ident(v as u32) }
                            }
                        });
                    }
                    "i32" => {
                        extra_from_impls.push(quote! {
                            impl ::std::convert::From<i32> for #enum_name {
                                fn from(v: i32) -> Self { Self::#v_ident(v) }
                            }
                            impl ::std::convert::From<u32> for #enum_name {
                                fn from(v: u32) -> Self { Self::#v_ident(v as i32) }
                            }
                            impl ::std::convert::From<usize> for #enum_name {
                                fn from(v: usize) -> Self { Self::#v_ident(v as i32) }
                            }
                        });
                    }
                    "f32" => {
                        extra_from_impls.push(quote! {
                            impl ::std::convert::From<f32> for #enum_name {
                                fn from(v: f32) -> Self { Self::#v_ident(v) }
                            }
                            impl ::std::convert::From<f64> for #enum_name {
                                fn from(v: f64) -> Self { Self::#v_ident(v as f32) }
                            }
                            impl ::std::convert::From<i32> for #enum_name {
                                fn from(v: i32) -> Self { Self::#v_ident(v as f32) }
                            }
                        });
                    }
                    "String" => {}
                    _ => {
                        extra_from_impls.push(quote! {
                            impl ::std::convert::From<#inner_ty> for #enum_name {
                                fn from(v: #inner_ty) -> Self { Self::#v_ident(v) }
                            }
                        });
                    }
                }
            }
            _ => {
                return TokenStream::from(
                    quote::quote_spanned! { v_ident.span() => compile_error!("#[prop] enum variants must be unit variants or 1-tuple variants like `custom(number)`"); },
                );
            }
        }
    }

    let def_body = default_expr.unwrap_or_else(|| quote! { unreachable!() });

    TokenStream::from(quote! {
        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        #[derive(Clone, Debug, PartialEq)]
        #( #attrs )*
        #vis enum #enum_name {
            #( #clean_variants ),*
        }

        impl ::std::default::Default for #enum_name {
            fn default() -> Self {
                #def_body
            }
        }

        impl ::std::convert::From<&str> for #enum_name {
            fn from(s: &str) -> Self {
                match s {
                    #( #from_str_arms )*
                    _ => ::std::default::Default::default(),
                }
            }
        }

        impl ::std::convert::From<::std::string::String> for #enum_name {
            fn from(s: ::std::string::String) -> Self {
                Self::from(s.as_str())
            }
        }

        impl ::std::convert::From<#enum_name> for ::std::string::String {
            fn from(v: #enum_name) -> ::std::string::String {
                match v {
                    #( #as_str_arms )*
                }
            }
        }

        #( #extra_from_impls )*

        #[doc(hidden)]
        #[allow(non_camel_case_types, non_snake_case, dead_code)]
        #vis struct #opts_struct_ident {
            #( #opt_fields ),*
        }

        impl #opts_struct_ident {
            #( #opt_methods )*
        }

        impl ::std::convert::From<#opts_struct_ident> for #enum_name {
            fn from(_: #opts_struct_ident) -> Self {
                ::std::default::Default::default()
            }
        }

        impl #enum_name {
            #[doc(hidden)]
            #[allow(non_snake_case, dead_code)]
            pub const fn __nk_enum_opts() -> #opts_struct_ident {
                #opts_struct_ident {
                    #( #opt_inits ),*
                }
            }
        }
    })
}

fn is_bool_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if let Some(last_seg) = tp.path.segments.last() {
            return last_seg.ident == "bool";
        }
    }
    false
}

fn is_action_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if let Some(last_seg) = tp.path.segments.last() {
            return last_seg.ident == "Action";
        }
    }
    false
}

fn is_node_slot_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if let Some(last_seg) = tp.path.segments.last() {
            return matches!(last_seg.ident.to_string().as_str(), "Node" | "View" | "Slot");
        }
    }
    false
}

fn is_vec_of_node_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if let Some(last_seg) = tp.path.segments.last() {
            if last_seg.ident == "Children" {
                return true;
            }
            if last_seg.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return is_node_slot_type(inner_ty);
                    }
                }
            }
        }
    }
    false
}

fn extract_vec_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(tp) = ty {
        if let Some(last_seg) = tp.path.segments.last() {
            if last_seg.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

fn normalize_friendly_type(ty: Type) -> Type {
    match ty {
        Type::Reference(r) => {
            if let Type::Path(tp) = &*r.elem {
                if tp.path.is_ident("str") {
                    return syn::parse2(quote! { String }).unwrap();
                }
            }
            Type::Reference(r)
        }
        Type::Slice(s) => {
            let inner = normalize_friendly_type(*s.elem);
            syn::parse2(quote! { ::std::vec::Vec<#inner> }).unwrap()
        }
        Type::Array(a) => {
            let inner = normalize_friendly_type(*a.elem);
            syn::parse2(quote! { ::std::vec::Vec<#inner> }).unwrap()
        }
        Type::Path(mut tp) => {
            if let Some(last_seg) = tp.path.segments.last_mut() {
                let id_str = last_seg.ident.to_string();
                match id_str.as_str() {
                    "str" | "string" => {
                        return syn::parse2(quote! { String }).unwrap();
                    }
                    "number" | "uint" => {
                        return syn::parse2(quote! { u32 }).unwrap();
                    }
                    "int" => {
                        return syn::parse2(quote! { i32 }).unwrap();
                    }
                    "float" => {
                        return syn::parse2(quote! { f32 }).unwrap();
                    }
                    "boolean" => {
                        return syn::parse2(quote! { bool }).unwrap();
                    }
                    "View" | "Slot" => {
                        return syn::parse2(quote! { ::native_kit::Node }).unwrap();
                    }
                    "Color" => {
                        return syn::parse2(quote! { ::native_kit::Color }).unwrap();
                    }
                    "Font" => {
                        return syn::parse2(quote! { ::native_kit::Font }).unwrap();
                    }
                    "Icon" | "IconName" => {
                        return syn::parse2(quote! { ::native_kit::IconName }).unwrap();
                    }
                    "Action" => {
                        return syn::parse2(quote! { ::native_kit::Action }).unwrap();
                    }
                    "Children" => {
                        return syn::parse2(quote! { ::native_kit::Children }).unwrap();
                    }
                    "Option" | "Vec" => {
                        if let syn::PathArguments::AngleBracketed(ref mut args) = last_seg.arguments
                        {
                            for arg in args.args.iter_mut() {
                                if let syn::GenericArgument::Type(inner_ty) = arg {
                                    *inner_ty = normalize_friendly_type(inner_ty.clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Type::Path(tp)
        }
        other => other,
    }
}

fn rewrite_match_dot_patterns_with_params(
    tokens: &[TokenTree],
    params: &[ParsedPropParam],
    comp_name: &Ident,
) -> Vec<TokenTree> {
    let mut out = Vec::new();
    let mut idx = 0;

    while idx < tokens.len() {
        if let TokenTree::Ident(kw) = &tokens[idx] {
            if kw == "match" {
                out.push(tokens[idx].clone());
                idx += 1;

                let mut scrutinee_tokens = Vec::new();
                while idx < tokens.len() {
                    if let TokenTree::Group(g) = &tokens[idx] {
                        if g.delimiter() == Delimiter::Brace {
                            break;
                        }
                    }
                    scrutinee_tokens.push(tokens[idx].clone());
                    out.push(tokens[idx].clone());
                    idx += 1;
                }

                let enum_ty: Option<TokenStream2> = if scrutinee_tokens.len() == 1 {
                    if let TokenTree::Ident(scrut_ident) = &scrutinee_tokens[0] {
                        let scrut_name = scrut_ident.to_string();
                        params
                            .iter()
                            .find(|p| p.ident.to_string() == scrut_name)
                            .and_then(|p| {
                                if !p.options.is_empty() || p.custom_union_ty.is_some() {
                                    let gen_enum = Ident::new(
                                        &format!("__NkPropEnum_{}_{}", comp_name, p.ident),
                                        p.ident.span(),
                                    );
                                    Some(quote! { #gen_enum })
                                } else {
                                    p.ty.as_ref().map(|ty| {
                                        let inner = extract_option_inner_type(ty).unwrap_or(ty);
                                        quote! { #inner }
                                    })
                                }
                            })
                    } else {
                        None
                    }
                } else {
                    None
                };

                if idx < tokens.len() {
                    if let TokenTree::Group(g) = &tokens[idx] {
                        if g.delimiter() == Delimiter::Brace {
                            let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                            let rewritten_arms = rewrite_match_arms_for_param(
                                &inner,
                                enum_ty.as_ref(),
                                params,
                                comp_name,
                            );
                            let mut ng = Group::new(
                                Delimiter::Brace,
                                TokenStream2::from_iter(rewritten_arms),
                            );
                            ng.set_span(g.span());
                            out.push(TokenTree::Group(ng));
                            idx += 1;
                            continue;
                        }
                    }
                }
                continue;
            }
        }

        if let TokenTree::Group(g) = &tokens[idx] {
            let inner: Vec<TokenTree> = g.stream().into_iter().collect();
            let rewritten = rewrite_match_dot_patterns_with_params(&inner, params, comp_name);
            let mut ng = Group::new(g.delimiter(), TokenStream2::from_iter(rewritten));
            ng.set_span(g.span());
            out.push(TokenTree::Group(ng));
            idx += 1;
            continue;
        }

        out.push(tokens[idx].clone());
        idx += 1;
    }

    out
}

fn rewrite_rhs_leading_dot(tokens: &[TokenTree]) -> Vec<TokenTree> {
    if tokens.len() == 2 {
        if let (TokenTree::Punct(dot), TokenTree::Ident(tok_id)) = (&tokens[0], &tokens[1]) {
            if dot.as_char() == '.' {
                let s = tok_id.to_string();
                return quote! { #s }.into_iter().collect();
            }
        }
    }
    tokens.to_vec()
}

fn rewrite_match_arms_for_param(
    tokens: &[TokenTree],
    enum_ty: Option<&TokenStream2>,
    params: &[ParsedPropParam],
    comp_name: &Ident,
) -> Vec<TokenTree> {
    let mut out = Vec::new();
    let mut idx = 0;
    let mut at_arm_start = true;

    while idx < tokens.len() {
        if idx + 1 < tokens.len() {
            if let (TokenTree::Punct(p1), TokenTree::Punct(p2)) = (&tokens[idx], &tokens[idx + 1]) {
                if p1.as_char() == '=' && p2.as_char() == '>' {
                    out.push(tokens[idx].clone());
                    out.push(tokens[idx + 1].clone());
                    idx += 2;

                    if idx < tokens.len() {
                        if let TokenTree::Group(g) = &tokens[idx] {
                            if g.delimiter() == Delimiter::Brace {
                                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                                let rewritten = rewrite_match_dot_patterns_with_params(
                                    &inner, params, comp_name,
                                );
                                let mut ng = Group::new(
                                    Delimiter::Brace,
                                    TokenStream2::from_iter(rewritten),
                                );
                                ng.set_span(g.span());
                                out.push(TokenTree::Group(ng));
                                idx += 1;
                                if idx < tokens.len() {
                                    if let TokenTree::Punct(comma) = &tokens[idx] {
                                        if comma.as_char() == ',' {
                                            out.push(tokens[idx].clone());
                                            idx += 1;
                                        }
                                    }
                                }
                                at_arm_start = true;
                                continue;
                            }
                        }
                    }

                    let mut arm_rhs = Vec::new();
                    while idx < tokens.len() {
                        if let TokenTree::Punct(p) = &tokens[idx] {
                            if p.as_char() == ',' {
                                break;
                            }
                        }
                        if let TokenTree::Group(g) = &tokens[idx] {
                            let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                            let rewritten =
                                rewrite_match_dot_patterns_with_params(&inner, params, comp_name);
                            let mut ng =
                                Group::new(g.delimiter(), TokenStream2::from_iter(rewritten));
                            ng.set_span(g.span());
                            arm_rhs.push(TokenTree::Group(ng));
                        } else {
                            arm_rhs.push(tokens[idx].clone());
                        }
                        idx += 1;
                    }
                    out.extend(rewrite_rhs_leading_dot(&arm_rhs));
                    if idx < tokens.len() {
                        out.push(tokens[idx].clone());
                        idx += 1;
                    }
                    at_arm_start = true;
                    continue;
                }
            }
        }

        if at_arm_start {
            if let TokenTree::Punct(p) = &tokens[idx] {
                if p.as_char() == '.' {
                    if let Some(TokenTree::Ident(var_id)) = tokens.get(idx + 1) {
                        if let Some(ty_ts) = enum_ty {
                            let clean_var_id = Ident::new(&var_id.to_string(), Span::call_site());
                            let replacement = quote! { #ty_ts::#clean_var_id };
                            out.extend(replacement);
                            idx += 2;
                            continue;
                        }
                    }
                }
            }
        }

        out.push(tokens[idx].clone());
        idx += 1;
    }

    out
}

fn expand_component_body(
    raw_body_tokens: &[TokenTree],
    params: &[ParsedPropParam],
    comp_name: &Ident,
) -> TokenStream2 {
    if raw_body_tokens.is_empty() {
        return quote! { ::native_kit::VStack::new() };
    }
    let pre_rewritten = rewrite_match_dot_patterns_with_params(raw_body_tokens, params, comp_name);
    let has_return_macro = pre_rewritten.windows(2).any(|w| {
        if let (TokenTree::Ident(id), TokenTree::Punct(p)) = (&w[0], &w[1]) {
            p.as_char() == '!'
                && matches!(
                    id.to_string().as_str(),
                    "Return" | "View" | "view" | "render" | "ui"
                )
        } else {
            false
        }
    });
    if has_return_macro {
        return transform_rust_tokens(&pre_rewritten);
    }
    let view_ts = TokenStream2::from(expand_view_macro(TokenStream::from(
        TokenStream2::from_iter(pre_rewritten),
    )));
    let view_tokens: Vec<TokenTree> = view_ts.into_iter().collect();
    let rewritten = rewrite_match_dot_patterns_with_params(&view_tokens, params, comp_name);
    TokenStream2::from_iter(rewritten)
}

fn extract_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(tp) = ty {
        if let Some(last_seg) = tp.path.segments.last() {
            if last_seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &last_seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

struct ParsedPropParam {
    mutability: Option<TokenTree>,
    ident: Ident,
    is_optional_mark: bool,
    ty: Option<Type>,
    options: Vec<String>,
    custom_union_ty: Option<Type>,
    default_dot_option: Option<Ident>,
    default_expr: Option<TokenStream2>,
}

fn strip_string_quotes(lit_str: &str) -> Option<String> {
    if lit_str.starts_with('"') && lit_str.ends_with('"') && lit_str.len() >= 2 {
        Some(lit_str[1..lit_str.len() - 1].to_string())
    } else {
        None
    }
}

fn parse_union_segment_type(seg: &[TokenTree]) -> Option<Type> {
    if seg.is_empty() {
        return None;
    }
    let ts = TokenStream2::from_iter(seg.iter().cloned());
    syn::parse2::<Type>(ts).ok().map(normalize_friendly_type)
}

fn parse_component_params(param_group: &Group) -> Vec<ParsedPropParam> {
    let all_tokens: Vec<TokenTree> = param_group.stream().into_iter().collect();
    let mut chunks = Vec::new();
    let mut cur = Vec::new();
    let mut angle_depth: usize = 0;
    for tok in all_tokens {
        match &tok {
            TokenTree::Punct(p) if p.as_char() == '<' => {
                angle_depth += 1;
                cur.push(tok);
            }
            TokenTree::Punct(p) if p.as_char() == '>' && angle_depth > 0 => {
                angle_depth -= 1;
                cur.push(tok);
            }
            TokenTree::Punct(p) if p.as_char() == ',' && angle_depth == 0 => {
                if !cur.is_empty() {
                    chunks.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(tok),
        }
    }
    if !cur.is_empty() {
        chunks.push(cur);
    }

    let mut params = Vec::new();
    for chunk in chunks {
        let mut idx = 0;
        let mut mutability = None;
        if let Some(TokenTree::Ident(id)) = chunk.get(idx) {
            if id == "mut" {
                mutability = Some(chunk[idx].clone());
                idx += 1;
            }
        }
        let Some(TokenTree::Ident(param_ident)) = chunk.get(idx).cloned() else {
            continue;
        };
        idx += 1;

        let mut is_optional_mark = false;
        if let Some(TokenTree::Punct(q)) = chunk.get(idx) {
            if q.as_char() == '?' {
                is_optional_mark = true;
                idx += 1;
            }
        }

        // Find top-level `=` for default value
        let mut eq_pos = None;
        let mut ad = 0usize;
        for (i, t) in chunk.iter().enumerate().skip(idx) {
            match t {
                TokenTree::Punct(p) if p.as_char() == '<' => ad += 1,
                TokenTree::Punct(p) if p.as_char() == '>' && ad > 0 => ad -= 1,
                TokenTree::Punct(p) if p.as_char() == '=' && ad == 0 => {
                    eq_pos = Some(i);
                    break;
                }
                _ => {}
            }
        }

        let type_slice = match eq_pos {
            Some(ep) => &chunk[idx..ep],
            None => &chunk[idx..],
        };
        let default_slice = match eq_pos {
            Some(ep) => &chunk[ep + 1..],
            None => &[],
        };

        // Parse `: Type` or `: .opt1 | .opt2` or `: .large | .medium | number`
        let mut ty: Option<Type> = None;
        let mut options: Vec<String> = Vec::new();
        let mut custom_union_ty: Option<Type> = None;

        if !type_slice.is_empty() {
            let raw_ty = if matches!(type_slice.first(), Some(TokenTree::Punct(c)) if c.as_char() == ':') {
                &type_slice[1..]
            } else {
                type_slice
            };
            let has_pipe = raw_ty
                .iter()
                .any(|t| matches!(t, TokenTree::Punct(p) if p.as_char() == '|'));
            let starts_with_dot =
                matches!(raw_ty.first(), Some(TokenTree::Punct(p)) if p.as_char() == '.');

            if has_pipe || starts_with_dot {
                let mut segs: Vec<Vec<TokenTree>> = Vec::new();
                let mut cur_seg = Vec::new();
                for tok in raw_ty {
                    if let TokenTree::Punct(p) = tok {
                        if p.as_char() == '|' {
                            if !cur_seg.is_empty() {
                                segs.push(std::mem::take(&mut cur_seg));
                            }
                            continue;
                        }
                    }
                    cur_seg.push(tok.clone());
                }
                if !cur_seg.is_empty() {
                    segs.push(cur_seg);
                }

                for seg in segs {
                    if seg.len() == 2 {
                        if let (TokenTree::Punct(p), TokenTree::Ident(id)) = (&seg[0], &seg[1]) {
                            if p.as_char() == '.' {
                                options.push(id.to_string());
                                continue;
                            }
                        }
                    }
                    if seg.len() == 1 {
                        if let TokenTree::Literal(lit) = &seg[0] {
                            if let Some(s) = strip_string_quotes(&lit.to_string()) {
                                options.push(s);
                                continue;
                            }
                        }
                    }
                    if let Some(parsed_custom_ty) = parse_union_segment_type(&seg) {
                        custom_union_ty = Some(parsed_custom_ty);
                    }
                }
            } else {
                let ts = TokenStream2::from_iter(raw_ty.iter().cloned());
                ty = syn::parse2::<Type>(ts).ok().map(normalize_friendly_type);
            }
        }

        // Parse `= default` expression
        let mut default_expr: Option<TokenStream2> = None;
        let mut default_dot_option: Option<Ident> = None;
        if !default_slice.is_empty() {
            if default_slice.len() == 2 {
                if let (TokenTree::Punct(dot), TokenTree::Ident(opt_id)) =
                    (&default_slice[0], &default_slice[1])
                {
                    if dot.as_char() == '.' {
                        default_dot_option = Some(Ident::new(&opt_id.to_string(), Span::call_site()));
                        let s = opt_id.to_string();
                        default_expr = Some(quote! { (#s).into() });
                    }
                }
            }
            if default_expr.is_none() {
                if default_slice.len() == 1 {
                    if let TokenTree::Ident(id) = &default_slice[0] {
                        if id == "None" {
                            default_expr = Some(quote! { ::std::option::Option::None });
                        }
                    }
                }
            }
            if default_expr.is_none() {
                let d_tokens = transform_rust_tokens(default_slice);
                default_expr = Some(quote! { (#d_tokens).into() });
            }

            // Infer type from default value if `: Type` was omitted
            if ty.is_none() && options.is_empty() && custom_union_ty.is_none() {
                if let Some(TokenTree::Literal(lit)) = default_slice.first() {
                    let s = lit.to_string();
                    if s.starts_with('"') {
                        ty = syn::parse2::<Type>(quote! { String }).ok();
                    } else if s.contains('.') {
                        ty = syn::parse2::<Type>(quote! { f32 }).ok();
                    } else if s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                        ty = syn::parse2::<Type>(quote! { i32 }).ok();
                    }
                } else if let Some(TokenTree::Ident(id)) = default_slice.first() {
                    if id == "true" || id == "false" {
                        ty = syn::parse2::<Type>(quote! { bool }).ok();
                    } else if id == "None" {
                        ty = syn::parse2::<Type>(quote! { Option<String> }).ok();
                    }
                } else if let Some(TokenTree::Punct(p)) = default_slice.first() {
                    if p.as_char() == '.' {
                        ty = syn::parse2::<Type>(quote! { String }).ok();
                    }
                }
            }
        }

        if is_optional_mark && default_expr.is_none() {
            if let Some(base_ty) = ty.take() {
                if extract_option_inner_type(&base_ty).is_some() {
                    ty = Some(base_ty);
                } else {
                    ty = syn::parse2::<Type>(quote! { ::std::option::Option<#base_ty> }).ok();
                }
            } else if options.is_empty() && custom_union_ty.is_none() {
                ty = syn::parse2::<Type>(quote! { ::std::option::Option<String> }).ok();
            }
        }

        params.push(ParsedPropParam {
            mutability,
            ident: param_ident,
            is_optional_mark,
            ty,
            options,
            custom_union_ty,
            default_dot_option,
            default_expr,
        });
    }
    params
}

fn is_valid_snake_case_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_component(attr, item)
}

fn expand_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2 = TokenStream2::from(item);
    let tokens: Vec<TokenTree> = item2.into_iter().collect();
    let attr_tokens: Vec<TokenTree> = TokenStream2::from(attr).into_iter().collect();

    // Find the parameter Parenthesis group `( ... )` in the header
    let Some(paren_idx) = tokens
        .iter()
        .position(|t| matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis))
    else {
        return TokenStream::from(TokenStream2::from_iter(tokens));
    };

    let param_group = match &tokens[paren_idx] {
        TokenTree::Group(g) => g.clone(),
        _ => unreachable!(),
    };

    let prefix_tokens = &tokens[..paren_idx];
    let after_paren = &tokens[paren_idx + 1..];

    // Support both Layout B (`View! { Card(...) let ... VStack { ... } }`)
    // and traditional (`View! { Card(...) { ... } }` or `#[component] pub fn Card(...) { ... }`)
    let (ret_sig_tokens, raw_body_tokens): (Vec<TokenTree>, Vec<TokenTree>) =
        if after_paren.len() == 1
            && matches!(after_paren.first(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace)
        {
            if let TokenTree::Group(g) = &after_paren[0] {
                (Vec::new(), g.stream().into_iter().collect())
            } else {
                unreachable!()
            }
        } else if after_paren.len() >= 3
            && matches!(after_paren.first(), Some(TokenTree::Punct(p)) if p.as_char() == '-')
            && matches!(after_paren.get(1), Some(TokenTree::Punct(p)) if p.as_char() == '>')
            && matches!(after_paren.last(), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace)
        {
            let last_i = after_paren.len() - 1;
            let body = if let TokenTree::Group(g) = &after_paren[last_i] {
                g.stream().into_iter().collect()
            } else {
                Vec::new()
            };
            (after_paren[..last_i].to_vec(), body)
        } else {
            (Vec::new(), after_paren.to_vec())
        };

    // Build a clean Rust signature without custom `= default` or `.a | .b` inside `(...)` so `syn` always parses it
    let mut clean_sig_tokens = TokenStream2::new();
    let has_fn_kw = prefix_tokens
        .iter()
        .any(|t| matches!(t, TokenTree::Ident(id) if id == "fn"));
    if has_fn_kw {
        clean_sig_tokens.extend(prefix_tokens.iter().cloned());
    } else {
        clean_sig_tokens.extend(quote! { pub fn });
        clean_sig_tokens.extend(prefix_tokens.iter().cloned());
    }
    clean_sig_tokens.extend(quote! { () });
    clean_sig_tokens.extend(ret_sig_tokens.into_iter());
    clean_sig_tokens.extend(quote! { {} });

    let Ok(sig_fn) = syn::parse2::<ItemFn>(clean_sig_tokens) else {
        return TokenStream::from(TokenStream2::from_iter(tokens));
    };

    let fn_name = &sig_fn.sig.ident;
    let fn_vis = &sig_fn.vis;
    let fn_generics = &sig_fn.sig.generics;

    let fn_name_str = fn_name.to_string();
    if !fn_name_str
        .chars()
        .next()
        .map(|c| c.is_ascii_uppercase())
        .unwrap_or(false)
    {
        let err_msg = format!(
            "#[component] name `{}` must be PascalCase (e.g. `{}{}`)",
            fn_name_str,
            fn_name_str.chars().next().unwrap_or('C').to_ascii_uppercase(),
            &fn_name_str[1..]
        );
        return TokenStream::from(quote::quote_spanned! { fn_name.span() => compile_error!(#err_msg); });
    }

    let mut parsed_params = parse_component_params(&param_group);
    if !attr_tokens.is_empty() {
        let attr_group = Group::new(Delimiter::Parenthesis, TokenStream2::from_iter(attr_tokens));
        for ap in parse_component_params(&attr_group) {
            if let Some(existing) = parsed_params.iter_mut().find(|p| p.ident == ap.ident) {
                if !ap.options.is_empty() {
                    existing.options = ap.options;
                }
                if ap.custom_union_ty.is_some() {
                    existing.custom_union_ty = ap.custom_union_ty;
                }
                if ap.default_dot_option.is_some() {
                    existing.default_dot_option = ap.default_dot_option;
                }
                if ap.default_expr.is_some() {
                    existing.default_expr = ap.default_expr;
                }
                if existing.ty.is_none() && ap.ty.is_some() {
                    existing.ty = ap.ty;
                }
            } else {
                parsed_params.push(ap);
            }
        }
    }

    // Enforce snake_case on all prop names and inline union options
    for p in &parsed_params {
        let p_str = p.ident.to_string();
        if !is_valid_snake_case_ident(&p_str) {
            let err_msg = format!(
                "#[component] prop `{}` on `{}` must be snake_case.",
                p_str, fn_name_str
            );
            return TokenStream::from(
                quote::quote_spanned! { p.ident.span() => compile_error!(#err_msg); },
            );
        }
        for opt in &p.options {
            let clean: String = opt
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();
            if !is_valid_snake_case_ident(&clean) {
                let err_msg = format!(
                    "#[component] option `.{}` on `{}.{}` must be snake_case.",
                    opt, fn_name_str, p_str
                );
                return TokenStream::from(
                    quote::quote_spanned! { p.ident.span() => compile_error!(#err_msg); },
                );
            }
        }
    }

    let transformed_body = expand_component_body(&raw_body_tokens, &parsed_params, fn_name);

    let mut field_defs = Vec::new();
    let mut default_inits = Vec::new();
    let mut builder_methods = Vec::new();
    let mut local_bindings = Vec::new();
    let mut prop_option_helpers = Vec::new();
    let mut has_children_param = false;

    for param in &parsed_params {
        let param_ident = &param.ident;
        let param_mut = &param.mutability;
        let param_name_str = param_ident.to_string();
        let fallback_ty: Type = syn::parse2(quote! { String }).unwrap();

        let is_union_prop = !param.options.is_empty() || param.custom_union_ty.is_some();
        let union_enum_ident = Ident::new(
            &format!("__NkPropEnum_{}_{}", fn_name, param_ident),
            param_ident.span(),
        );
        let union_ty: Type = syn::parse2(quote! { #union_enum_ident }).unwrap();
        let effective_base_ty: &Type = if is_union_prop {
            &union_ty
        } else {
            param.ty.as_ref().unwrap_or(&fallback_ty)
        };

        let final_ty: Type = if is_union_prop && param.is_optional_mark && param.default_expr.is_none()
        {
            syn::parse2(quote! { ::std::option::Option<#union_enum_ident> }).unwrap()
        } else {
            effective_base_ty.clone()
        };
        let param_ty = &final_ty;

        let is_children = param_name_str == "children" || is_vec_of_node_type(param_ty);

        if is_children && param_name_str == "children" {
            has_children_param = true;
            field_defs.push(quote! {
                #param_ident: ::std::vec::Vec<::native_kit::Node>
            });
            default_inits.push(quote! {
                #param_ident: ::std::vec::Vec::new()
            });
            local_bindings.push(quote! {
                let #param_mut #param_ident = self.#param_ident;
            });
            continue;
        }

        // Generate real Rust enum + autocomplete helper struct for inline union props (`.a | .b` or `.a | .b | i32`)
        if is_union_prop {
            let opts_struct_ident = Ident::new(
                &format!("__NkPropOpts_{}_{}", fn_name, param_ident),
                param_ident.span(),
            );
            let mut enum_variants = Vec::new();
            let mut opt_fields = Vec::new();
            let mut opt_inits = Vec::new();
            let mut opt_methods = Vec::new();
            let mut from_str_arms = Vec::new();
            let mut as_str_arms = Vec::new();
            let mut extra_from_impls = Vec::new();
            let mut first_unit_ident: Option<Ident> = None;

            for o in &param.options {
                let clean: String = o
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                    .collect();
                if clean.is_empty() {
                    continue;
                }
                let id = Ident::new(&clean, Span::call_site());
                if first_unit_ident.is_none() {
                    first_unit_ident = Some(id.clone());
                }
                enum_variants.push(quote! { #id });
                opt_fields.push(quote! { pub #id: #union_enum_ident });
                opt_inits.push(quote! { #id: #union_enum_ident::#id });
                from_str_arms.push(quote! { #o | #clean => #union_enum_ident::#id, });
                as_str_arms.push(quote! { #union_enum_ident::#id => #o.to_string(), });
            }

            if let Some(ref custom_ty) = param.custom_union_ty {
                enum_variants.push(quote! { custom(#custom_ty) });
                opt_methods.push(quote! {
                    pub fn custom(self, val: impl ::std::convert::Into<#custom_ty>) -> #union_enum_ident {
                        #union_enum_ident::custom(val.into())
                    }
                });
                as_str_arms.push(quote! { #union_enum_ident::custom(ref v) => format!("{:?}", v), });

                let custom_ty_name = if let Type::Path(tp) = custom_ty {
                    tp.path
                        .segments
                        .last()
                        .map(|s| s.ident.to_string())
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                match custom_ty_name.as_str() {
                    "i32" => {
                        extra_from_impls.push(quote! {
                            impl ::std::convert::From<i32> for #union_enum_ident {
                                fn from(v: i32) -> Self { Self::custom(v) }
                            }
                            impl ::std::convert::From<u32> for #union_enum_ident {
                                fn from(v: u32) -> Self { Self::custom(v as i32) }
                            }
                            impl ::std::convert::From<usize> for #union_enum_ident {
                                fn from(v: usize) -> Self { Self::custom(v as i32) }
                            }
                        });
                    }
                    "u32" => {
                        extra_from_impls.push(quote! {
                            impl ::std::convert::From<u32> for #union_enum_ident {
                                fn from(v: u32) -> Self { Self::custom(v) }
                            }
                            impl ::std::convert::From<i32> for #union_enum_ident {
                                fn from(v: i32) -> Self { Self::custom(v.max(0) as u32) }
                            }
                            impl ::std::convert::From<usize> for #union_enum_ident {
                                fn from(v: usize) -> Self { Self::custom(v as u32) }
                            }
                        });
                    }
                    "f32" => {
                        extra_from_impls.push(quote! {
                            impl ::std::convert::From<f32> for #union_enum_ident {
                                fn from(v: f32) -> Self { Self::custom(v) }
                            }
                            impl ::std::convert::From<f64> for #union_enum_ident {
                                fn from(v: f64) -> Self { Self::custom(v as f32) }
                            }
                            impl ::std::convert::From<i32> for #union_enum_ident {
                                fn from(v: i32) -> Self { Self::custom(v as f32) }
                            }
                        });
                    }
                    "String" => {}
                    _ => {
                        extra_from_impls.push(quote! {
                            impl ::std::convert::From<#custom_ty> for #union_enum_ident {
                                fn from(v: #custom_ty) -> Self { Self::custom(v) }
                            }
                        });
                    }
                }
            }

            let default_variant_expr = if let Some(ref def_opt) = param.default_dot_option {
                quote! { Self::#def_opt }
            } else if let Some(ref first_id) = first_unit_ident {
                quote! { Self::#first_id }
            } else if param.custom_union_ty.is_some() {
                quote! { Self::custom(::std::default::Default::default()) }
            } else {
                quote! { unreachable!() }
            };

            let str_fallback = if let Some(ref custom_ty) = param.custom_union_ty {
                let is_str_custom = matches!(custom_ty, Type::Path(tp) if tp.path.segments.last().map(|s| s.ident == "String").unwrap_or(false));
                if is_str_custom {
                    quote! { other => Self::custom(other.to_string()), }
                } else {
                    quote! { _ => ::std::default::Default::default(), }
                }
            } else {
                quote! { _ => ::std::default::Default::default(), }
            };

            let helper_fn_ident = Ident::new(
                &format!("__nk_prop_opts_{}", param_ident),
                param_ident.span(),
            );

            prop_option_helpers.push(quote! {
                #[allow(non_camel_case_types, non_snake_case, dead_code)]
                #[derive(Clone, Debug, PartialEq)]
                #fn_vis enum #union_enum_ident {
                    #( #enum_variants ),*
                }

                impl ::std::default::Default for #union_enum_ident {
                    fn default() -> Self {
                        #default_variant_expr
                    }
                }

                impl ::std::convert::From<&str> for #union_enum_ident {
                    fn from(s: &str) -> Self {
                        match s {
                            #( #from_str_arms )*
                            #str_fallback
                        }
                    }
                }

                impl ::std::convert::From<::std::string::String> for #union_enum_ident {
                    fn from(s: ::std::string::String) -> Self {
                        Self::from(s.as_str())
                    }
                }

                impl ::std::convert::From<#union_enum_ident> for ::std::string::String {
                    fn from(v: #union_enum_ident) -> ::std::string::String {
                        match v {
                            #( #as_str_arms )*
                        }
                    }
                }

                #( #extra_from_impls )*

                #[allow(non_camel_case_types, non_snake_case, dead_code)]
                #fn_vis struct #opts_struct_ident {
                    #( #opt_fields ),*
                }

                impl #opts_struct_ident {
                    #( #opt_methods )*
                }

                impl ::std::convert::From<#opts_struct_ident> for #union_enum_ident {
                    fn from(_: #opts_struct_ident) -> Self {
                        ::std::default::Default::default()
                    }
                }

                impl #fn_generics #fn_name #fn_generics {
                    #[doc(hidden)]
                    #[allow(non_snake_case, dead_code)]
                    pub const fn #helper_fn_ident() -> #opts_struct_ident {
                        #opts_struct_ident {
                            #( #opt_inits ),*
                        }
                    }
                }
            });
        } else {
            let helper_ty = extract_option_inner_type(param_ty).unwrap_or(param_ty);
            let helper_fn_ident = Ident::new(
                &format!("__nk_prop_opts_{}", param_ident),
                param_ident.span(),
            );
            if let Type::Path(tp) = helper_ty {
                if let Some(last_seg) = tp.path.segments.last() {
                    let seg_name = last_seg.ident.to_string();
                    let props_ty = match seg_name.as_str() {
                        "Color" => Some(quote! { ::native_kit::__props::ColorProps }),
                        "Font" => Some(quote! { ::native_kit::__props::FontProps }),
                        "IconName" => Some(quote! { ::native_kit::__props::IconNameProps }),
                        "String" | "u32" | "i32" | "f32" | "f64" | "usize" | "isize" | "bool"
                        | "Node" | "View" | "Slot" | "Children" | "Action" | "Vec" => None,
                        custom_enum => {
                            let opts_id = Ident::new(
                                &format!("__NkPropOpts_{}", custom_enum),
                                last_seg.ident.span(),
                            );
                            Some(quote! { #opts_id })
                        }
                    };
                    if let Some(p_ty) = props_ty {
                        prop_option_helpers.push(quote! {
                            impl #fn_generics #fn_name #fn_generics {
                                #[doc(hidden)]
                                #[allow(non_snake_case, dead_code)]
                                pub const fn #helper_fn_ident() -> #p_ty {
                                    <#helper_ty>::__nk_enum_opts()
                                }
                            }
                        });
                    }
                }
            }
        }

        field_defs.push(quote! {
            #param_ident: #param_ty
        });

        if is_union_prop {
            if let Some(ref def_opt) = param.default_dot_option {
                default_inits.push(quote! {
                    #param_ident: #union_enum_ident::#def_opt
                });
            } else if let Some(ref def_expr) = param.default_expr {
                default_inits.push(quote! {
                    #param_ident: #def_expr
                });
            } else {
                default_inits.push(quote! {
                    #param_ident: ::std::default::Default::default()
                });
            }
        } else if let Some(ref def_expr) = param.default_expr {
            default_inits.push(quote! {
                #param_ident: #def_expr
            });
        } else {
            default_inits.push(quote! {
                #param_ident: ::std::default::Default::default()
            });
        }

        let doc_comment = if !param.options.is_empty() {
            format!("Valid options: {}", param.options.join(", "))
        } else {
            String::new()
        };
        let doc_attr = if !doc_comment.is_empty() {
            quote! { #[doc = #doc_comment] }
        } else {
            TokenStream2::new()
        };

        if let Some(inner_ty) = extract_option_inner_type(param_ty) {
            if is_bool_type(inner_ty) {
                builder_methods.push(quote! {
                    #doc_attr
                    pub fn #param_ident(mut self, val: impl ::native_kit::IntoBoolProp) -> Self {
                        self.#param_ident = ::std::option::Option::Some(val.into_bool_prop());
                        self
                    }
                });
            } else if is_action_type(inner_ty) {
                builder_methods.push(quote! {
                    #doc_attr
                    pub fn #param_ident<__NkH, __NkArgs, __NkS>(mut self, handler: __NkH) -> Self
                    where
                        __NkH: ::native_kit::ActionHandler<__NkArgs, __NkS>,
                        __NkS: 'static,
                    {
                        self.#param_ident = ::std::option::Option::Some(::native_kit::Action::new(handler));
                        self
                    }
                });
            } else if is_node_slot_type(inner_ty) {
                builder_methods.push(quote! {
                    #doc_attr
                    pub fn #param_ident(mut self, val: impl ::native_kit::IntoNode) -> Self {
                        self.#param_ident = ::std::option::Option::Some(val.into_node());
                        self
                    }
                });
            } else {
                builder_methods.push(quote! {
                    #doc_attr
                    pub fn #param_ident(mut self, val: impl ::native_kit::IntoOptionalProp<#inner_ty>) -> Self {
                        self.#param_ident = val.into_optional_prop();
                        self
                    }
                });
            }
        } else if is_bool_type(param_ty) {
            builder_methods.push(quote! {
                #doc_attr
                pub fn #param_ident(mut self, val: impl ::native_kit::IntoBoolProp) -> Self {
                    self.#param_ident = val.into_bool_prop();
                    self
                }
            });
        } else if is_action_type(param_ty) {
            builder_methods.push(quote! {
                #doc_attr
                pub fn #param_ident<__NkH, __NkArgs, __NkS>(mut self, handler: __NkH) -> Self
                where
                    __NkH: ::native_kit::ActionHandler<__NkArgs, __NkS>,
                    __NkS: 'static,
                {
                    self.#param_ident = ::native_kit::Action::new(handler);
                    self
                }
            });
        } else if is_node_slot_type(param_ty) {
            builder_methods.push(quote! {
                #doc_attr
                pub fn #param_ident(mut self, val: impl ::native_kit::IntoNode) -> Self {
                    self.#param_ident = val.into_node();
                    self
                }
            });
        } else if is_vec_of_node_type(param_ty) {
            builder_methods.push(quote! {
                #doc_attr
                pub fn #param_ident(mut self, val: impl ::native_kit::IntoChildren) -> Self {
                    self.#param_ident = val.into_children();
                    self
                }
            });
        } else if let Some(vec_inner) = extract_vec_inner_type(param_ty) {
            builder_methods.push(quote! {
                #doc_attr
                pub fn #param_ident<__NkI, __NkItem>(mut self, items: __NkI) -> Self
                where
                    __NkI: ::std::iter::IntoIterator<Item = __NkItem>,
                    __NkItem: ::std::convert::Into<#vec_inner>,
                {
                    self.#param_ident = items.into_iter().map(::std::convert::Into::into).collect();
                    self
                }
            });
        } else {
            builder_methods.push(quote! {
                #doc_attr
                pub fn #param_ident(mut self, val: impl ::std::convert::Into<#param_ty>) -> Self {
                    self.#param_ident = val.into();
                    self
                }
            });
        }
        local_bindings.push(quote! {
            let #param_mut #param_ident = self.#param_ident;
        });
    }

    if !has_children_param {
        field_defs.push(quote! {
            __children: ::std::vec::Vec<::native_kit::Node>
        });
        default_inits.push(quote! {
            __children: ::std::vec::Vec::new()
        });
    }

    let children_field_ident = if has_children_param {
        Ident::new("children", Span::call_site())
    } else {
        Ident::new("__children", Span::call_site())
    };

    let ret_ty = match &sig_fn.sig.output {
        ReturnType::Default => quote! { impl ::native_kit::IntoNode },
        ReturnType::Type(_, ty) => quote! { #ty },
    };

    let render_fn_ident = Ident::new(&format!("__nk_render_{}", fn_name), fn_name.span());
    let new_fn_ident = Ident::new("new", fn_name.span());

    let expanded = quote! {
        #[derive(Clone)]
        #fn_vis struct #fn_name #fn_generics {
            #( #field_defs ),*
        }

        impl #fn_generics ::std::default::Default for #fn_name #fn_generics {
            fn default() -> Self {
                Self::#new_fn_ident()
            }
        }

        #[allow(non_snake_case, dead_code)]
        #fn_vis fn #fn_name #fn_generics () -> #fn_name #fn_generics {
            #fn_name::#new_fn_ident()
        }

        impl #fn_generics #fn_name #fn_generics {
            pub fn #new_fn_ident() -> Self {
                Self {
                    #( #default_inits ),*
                }
            }

            #( #builder_methods )*

            pub fn child(mut self, child: impl ::native_kit::IntoNode) -> Self {
                self.#children_field_ident.push(child.into_node());
                self
            }

            pub fn children(mut self, children: impl ::native_kit::IntoChildren) -> Self {
                self.#children_field_ident.extend(children.into_children());
                self
            }

            fn #render_fn_ident(self) -> #ret_ty {
                #[allow(unused_imports)]
                use ::native_kit::*;
                #[allow(unused_imports)]
                use crate::__nk_auto::*;
                #[allow(unused_imports)]
                use crate::app::*;
                #( #local_bindings )*
                #transformed_body
            }
        }

        impl #fn_generics ::native_kit::IntoNode for #fn_name #fn_generics {
            fn into_node(self) -> ::native_kit::Node {
                ::native_kit::IntoNode::into_node(self.#render_fn_ident())
            }
        }

        #( #prop_option_helpers )*
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn server(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_async = &input_fn.sig.asyncness;
    let fn_return = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    let expanded = quote! {
        #fn_vis #fn_async fn #fn_name() #fn_return {
            #[cfg(any(target_os = "ios", target_os = "android", feature = "client"))]
            {
                let url = format!("{}/api/{}", native_kit::engine::get_server_url(), stringify!(#fn_name));
                let client = reqwest::Client::builder()
                    .build()
                    .unwrap_or_default();

                match client.post(&url).send().await {
                    Ok(res) => {
                        match res.text().await {
                            Ok(text) => {
                                use base64::{Engine as _, engine::general_purpose};
                                let text = text.trim().to_string();
                                if general_purpose::STANDARD.decode(&text).is_ok() {
                                    return Ok(native_kit::engine::ast::Node::RawBinary(text));
                                } else {
                                    let preview = if text.len() > 100 { format!("{}...", &text[..100]) } else { text.clone() };
                                    return Err(format!("Invalid OTA binary. Server returned: {}", preview));
                                }
                            }
                            Err(e) => return Err(format!("Body Read Error: {}", e)),
                        }
                    }
                    Err(e) => {
                        let mut msg = format!("Network Error: {}", e);
                        let mut current: &dyn std::error::Error = &e;
                        while let Some(source) = current.source() {
                            msg.push_str(&format!("\nCaused by: {}", source));
                            current = source;
                        }
                        return Err(msg);
                    }
                }
            }
            #[cfg(not(any(target_os = "ios", target_os = "android", feature = "client")))]
            {
                #fn_block
            }
        }

        #[cfg(not(any(target_os = "ios", target_os = "android", feature = "client")))]
        native_kit::engine::dev_server::inventory::submit! {
            native_kit::engine::dev_server::ServerRoute {
                path: concat!("/api/", stringify!(#fn_name)),
                handler: || {
                    let result = native_kit::engine::runtime::block_on(async { #fn_name().await });
                    native_kit::engine::dev_server::IntoRouteResponse::into_response(result)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Declarative UI Macros (`Page!`, `View!`, `VStack!`, `HStack!`, `Section!`, etc.)
// ============================================================================

#[proc_macro]
pub fn Page(input: TokenStream) -> TokenStream {
    expand_container_macro("Page", input)
}

fn is_view_component_definition(tokens: &[TokenTree]) -> bool {
    let mut idx = 0;
    while idx < tokens.len() {
        if matches!(&tokens[idx], TokenTree::Punct(p) if p.as_char() == '#') {
            idx += 2;
            continue;
        }
        break;
    }
    if matches!(tokens.get(idx), Some(TokenTree::Ident(id)) if id == "pub") {
        idx += 1;
        if matches!(tokens.get(idx), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis) {
            idx += 1;
        }
    }
    let has_fn = if matches!(tokens.get(idx), Some(TokenTree::Ident(id)) if id == "fn") {
        idx += 1;
        true
    } else {
        false
    };
    let Some(TokenTree::Ident(name_id)) = tokens.get(idx) else {
        return false;
    };
    let name_str = name_id.to_string();
    let Some(TokenTree::Group(g)) = tokens.get(idx + 1) else {
        return false;
    };
    if g.delimiter() != Delimiter::Parenthesis {
        return false;
    }
    if has_fn {
        return true;
    }
    is_custom_component_ident(&name_str) && tokens.len() > idx + 2
}

#[proc_macro]
pub fn View(input: TokenStream) -> TokenStream {
    let ts2 = TokenStream2::from(input.clone());
    let tokens: Vec<TokenTree> = ts2.into_iter().collect();
    if is_view_component_definition(&tokens) {
        expand_component(TokenStream::new(), input)
    } else {
        expand_view_macro(input)
    }
}

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    expand_view_macro(input)
}

#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    expand_view_macro(input)
}

#[proc_macro]
pub fn ui(input: TokenStream) -> TokenStream {
    expand_view_macro(input)
}

#[proc_macro]
pub fn Return(input: TokenStream) -> TokenStream {
    expand_view_macro(input)
}

fn expand_view_macro(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = TokenStream2::from(input).into_iter().collect();

    // Allow leading `let ... = ...;` statements at the top of `Return! { ... }`
    let mut idx = 0;
    let mut prefix_stmts = TokenStream2::new();
    while idx < tokens.len() {
        if let TokenTree::Ident(id) = &tokens[idx] {
            if id == "let" {
                let mut stmt_tokens = Vec::new();
                let mut brace_depth = 0usize;
                while idx < tokens.len() {
                    let tok = tokens[idx].clone();
                    idx += 1;
                    if let TokenTree::Group(g) = &tok {
                        if g.delimiter() == Delimiter::Brace {
                            brace_depth += 1;
                            stmt_tokens.push(tok);
                            brace_depth -= 1;
                            continue;
                        }
                    }
                    let is_semi =
                        brace_depth == 0 && matches!(&tok, TokenTree::Punct(p) if p.as_char() == ';');
                    stmt_tokens.push(tok);
                    if is_semi {
                        break;
                    }
                }
                prefix_stmts.extend(transform_rust_tokens(&stmt_tokens));
                continue;
            }
        }
        break;
    }

    let children_tokens = if idx + 1 < tokens.len()
        && matches!(&tokens[idx], TokenTree::Ident(id) if id == "return")
        && matches!(&tokens[idx + 1], TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis)
    {
        if let TokenTree::Group(g) = &tokens[idx + 1] {
            g.stream().into_iter().collect::<Vec<TokenTree>>()
        } else {
            tokens[idx..].to_vec()
        }
    } else {
        tokens[idx..].to_vec()
    };
    let children = parse_children_tokens(&children_tokens);
    if children.len() == 1 {
        let single = &children[0];
        return TokenStream::from(quote! {
            {
                #[allow(unused_imports)]
                use crate::__nk_auto::*;
                #prefix_stmts
                #single
            }
        });
    }
    let mut out = quote! { ::native_kit::VStack::new() };
    for child in children {
        out.extend(quote! { .children(#child) });
    }
    TokenStream::from(quote! {
        {
            #[allow(unused_imports)]
            use ::native_kit::*;
            #[allow(unused_imports)]
            use crate::__nk_auto::*;
            #prefix_stmts
            #out
        }
    })
}

#[proc_macro]
pub fn VStack(input: TokenStream) -> TokenStream {
    expand_container_macro("VStack", input)
}

#[proc_macro]
pub fn HStack(input: TokenStream) -> TokenStream {
    expand_container_macro("HStack", input)
}

#[proc_macro]
pub fn Section(input: TokenStream) -> TokenStream {
    expand_container_macro("Section", input)
}

#[proc_macro]
pub fn Grid(input: TokenStream) -> TokenStream {
    expand_container_macro("Grid", input)
}

#[proc_macro]
pub fn ToolbarTop(input: TokenStream) -> TokenStream {
    expand_container_macro("ToolbarTop", input)
}

#[proc_macro]
pub fn NavigationStack(input: TokenStream) -> TokenStream {
    expand_container_macro("NavigationStack", input)
}

#[proc_macro]
pub fn Button(input: TokenStream) -> TokenStream {
    expand_container_macro("Button", input)
}

fn is_container_component(name: &str) -> bool {
    matches!(
        name,
        "Page" | "VStack" | "HStack" | "Section" | "Grid" | "NavigationStack" | "ToolbarTop" | "Button"
    )
}

fn is_leaf_component(name: &str) -> bool {
    matches!(
        name,
        "Text" | "Icon" | "Image" | "Switch" | "TextInput" | "Picker" | "Spacer"
    )
}

fn is_component(name: &str) -> bool {
    is_container_component(name) || is_leaf_component(name)
}

fn is_known_dot_prop(name: &str) -> bool {
    matches!(
        name,
        "font"
            | "color"
            | "background"
            | "stop"
            | "stop_with_opacity"
            | "add_stop"
            | "variant"
            | "rendering"
            | "placement"
            | "width"
            | "align"
            | "top_style"
            | "presentation"
            | "content_mode"
            | "style"
            | "direction"
            | "kind"
    )
}

fn is_raw_struct_brace(group: &Group) -> bool {
    let mut iter = group.stream().into_iter();
    if let (Some(TokenTree::Ident(id)), Some(TokenTree::Punct(p))) = (iter.next(), iter.next()) {
        if id == "props" && p.as_char() == ':' {
            return true;
        }
    }
    false
}

fn is_custom_component_ident(name: &str) -> bool {
    if is_component(name) {
        return false;
    }
    if matches!(
        name,
        "Gradient" | "Route" | "Router" | "Some" | "None" | "Ok" | "Err" | "Vec" | "String" | "Box" | "Arc" | "Rc"
    ) {
        return false;
    }
    name.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false)
}

fn is_component_start_ignoring_prev(tokens: &[TokenTree], idx: usize) -> bool {
    let Some(TokenTree::Ident(ident)) = tokens.get(idx) else {
        return false;
    };
    let name = ident.to_string();
    if is_component(&name) {
        if name == "Spacer" {
            return true;
        }
        return match tokens.get(idx + 1) {
            Some(TokenTree::Punct(p)) if p.as_char() == '!' => true,
            Some(TokenTree::Punct(p)) if p.as_char() == ':' => false,
            Some(TokenTree::Group(g)) => match g.delimiter() {
                Delimiter::Parenthesis => true,
                Delimiter::Brace => is_container_component(&name) && !is_raw_struct_brace(g),
                Delimiter::Bracket | Delimiter::None => false,
            },
            Some(TokenTree::Punct(p)) if p.as_char() == '.' => {
                is_container_component(&name)
                    || matches!(name.as_str(), "Switch" | "TextInput" | "Image" | "Text")
            }
            _ => false,
        };
    }

    if is_custom_component_ident(&name) {
        return match tokens.get(idx + 1) {
            Some(TokenTree::Punct(p)) if p.as_char() == ':' => false,
            Some(TokenTree::Punct(p)) if p.as_char() == '.' => true,
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace && !is_raw_struct_brace(g) => true,
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis && g.stream().is_empty() => true,
            _ => false,
        };
    }

    false
}

fn is_component_start(tokens: &[TokenTree], idx: usize) -> bool {
    if idx > 0 {
        if let TokenTree::Punct(p) = &tokens[idx - 1] {
            if p.as_char() == '.' || p.as_char() == ':' {
                return false;
            }
        }
    }
    is_component_start_ignoring_prev(tokens, idx)
}

fn is_string_literal(lit: &proc_macro2::Literal) -> bool {
    let s = lit.to_string();
    s.starts_with('"') || s.starts_with("r\"") || s.starts_with("r#")
}

fn rewrite_leading_dot_tokens(prop_table: &str, prop_span: Span, arg_tokens: &[TokenTree]) -> TokenStream2 {
    if let Some(TokenTree::Punct(first_dot)) = arg_tokens.first() {
        if first_dot.as_char() == '.' {
            let dot_span = first_dot.span();
            let table_ident = Ident::new(prop_table, prop_span);
            let rest_transformed = transform_rust_tokens(&arg_tokens[1..]);
            let prefix = quote::quote_spanned! { dot_span => ::native_kit::__props::#table_ident };
            return quote! { #prefix #first_dot #rest_transformed };
        }
    }
    transform_rust_tokens(arg_tokens)
}

fn transform_method_args(
    method_ident: &Ident,
    arg_stream: TokenStream2,
    comp_name: Option<&str>,
) -> TokenStream2 {
    let arg_tokens: Vec<TokenTree> = arg_stream.into_iter().collect();
    if arg_tokens.is_empty() {
        return TokenStream2::new();
    }
    let method_name = method_ident.to_string();
    let table_name = if comp_name == Some("Icon") && method_name == "variant" {
        "icon_variant"
    } else {
        method_name.as_str()
    };

    let is_custom = comp_name.map(|c| is_custom_component_ident(c)).unwrap_or(false);
    if !is_custom && (table_name == "icon_variant" || is_known_dot_prop(table_name)) {
        return rewrite_leading_dot_tokens(table_name, method_ident.span(), &arg_tokens);
    }

    // For custom component props (e.g. `Card.gradient(.option1)`), rewrite `.option1` into `Card::__nk_prop_opts_gradient().option1`
    // so rust-analyzer provides real IDE autocomplete for options declared on the component!
    if is_custom {
        if let Some(TokenTree::Punct(first_dot)) = arg_tokens.first() {
            if first_dot.as_char() == '.' {
                let c_name = comp_name.unwrap();
                let dot_span = first_dot.span();
                let comp_ident = Ident::new(c_name, dot_span);
                let helper_ident = Ident::new(&format!("__nk_prop_opts_{}", method_name), method_ident.span());
                let rest_transformed = transform_rust_tokens(&arg_tokens[1..]);
                let prefix = quote::quote_spanned! { dot_span => #comp_ident::#helper_ident() };
                return quote! { #prefix #first_dot #rest_transformed };
            }
        }
    }

    if table_name == "icon_variant" || is_known_dot_prop(table_name) {
        return rewrite_leading_dot_tokens(table_name, method_ident.span(), &arg_tokens);
    }

    transform_rust_tokens(&arg_tokens)
}

fn split_comma_args(stream: TokenStream2) -> Vec<TokenStream2> {
    let mut parts = Vec::new();
    let mut current = TokenStream2::new();
    for tt in stream {
        if let TokenTree::Punct(p) = &tt {
            if p.as_char() == ',' {
                if !current.is_empty() {
                    parts.push(current);
                    current = TokenStream2::new();
                }
                continue;
            }
        }
        current.extend(std::iter::once(tt));
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

fn consume_modifier_chain(
    tokens: &[TokenTree],
    idx: &mut usize,
    comp_name: Option<&str>,
    complete_mods: &mut TokenStream2,
    incomplete_mods: &mut TokenStream2,
) {
    while *idx < tokens.len() {
        let TokenTree::Punct(dot) = &tokens[*idx] else {
            break;
        };
        if dot.as_char() != '.' {
            break;
        }

        if *idx + 1 < tokens.len() {
            if let TokenTree::Ident(method_ident) = &tokens[*idx + 1] {
                // If `method_ident` is actually the next sibling component (e.g. user typed `.` at the end of a line before `Button(...)` or `Text(...)`)
                if is_component_start_ignoring_prev(tokens, *idx + 1) {
                    incomplete_mods.extend(std::iter::once(TokenTree::Punct(dot.clone())));
                    *idx += 1;
                    break;
                }

                // If `method_ident` is followed by `{ ... }` (e.g. user typed `.` or `.foo` right before `{ HStack ... }`),
                // do NOT consume the following `{ ... }` children block as part of the incomplete method!
                if *idx + 2 < tokens.len() {
                    if let TokenTree::Group(next_group) = &tokens[*idx + 2] {
                        if next_group.delimiter() == Delimiter::Brace {
                            incomplete_mods.extend([
                                TokenTree::Punct(dot.clone()),
                                TokenTree::Ident(method_ident.clone()),
                            ]);
                            *idx += 2;
                            break;
                        }
                    }
                }

                if *idx + 2 < tokens.len() {
                    if let TokenTree::Group(arg_group) = &tokens[*idx + 2] {
                        if arg_group.delimiter() == Delimiter::Parenthesis {
                            let m_name = method_ident.to_string();
                            if m_name == "align" {
                                let parts = split_comma_args(arg_group.stream());
                                if parts.len() > 1 {
                                    for part in parts {
                                        let transformed_part = transform_method_args(
                                            method_ident,
                                            part,
                                            comp_name,
                                        );
                                        let mut new_group = Group::new(Delimiter::Parenthesis, transformed_part);
                                        new_group.set_span(arg_group.span());
                                        complete_mods.extend([
                                            TokenTree::Punct(dot.clone()),
                                            TokenTree::Ident(method_ident.clone()),
                                            TokenTree::Group(new_group),
                                        ]);
                                    }
                                    *idx += 3;
                                    continue;
                                }
                            }
                            let transformed_args = if m_name == "wrap" && arg_group.stream().is_empty() {
                                quote! { true }
                            } else if arg_group.stream().is_empty()
                                && comp_name.map(|c| is_custom_component_ident(c)).unwrap_or(false)
                            {
                                quote! { true }
                            } else {
                                transform_method_args(
                                    method_ident,
                                    arg_group.stream(),
                                    comp_name,
                                )
                            };
                            let mut new_group = Group::new(Delimiter::Parenthesis, transformed_args);
                            new_group.set_span(arg_group.span());
                            complete_mods.extend([
                                TokenTree::Punct(dot.clone()),
                                TokenTree::Ident(method_ident.clone()),
                                TokenTree::Group(new_group),
                            ]);
                            *idx += 3;
                            continue;
                        }
                    }
                }

                // Incomplete method call (or rust-analyzer's completion marker `intellijRulezz`)
                incomplete_mods.extend([
                    TokenTree::Punct(dot.clone()),
                    TokenTree::Ident(method_ident.clone()),
                ]);
                *idx += 2;
                continue;
            }
        }

        // Lone `.` before `{` or end of block
        incomplete_mods.extend(std::iter::once(TokenTree::Punct(dot.clone())));
        *idx += 1;
        break;
    }
}

fn parse_component_invocation(tokens: &[TokenTree], idx: &mut usize) -> TokenStream2 {
    let TokenTree::Ident(comp_ident) = &tokens[*idx] else {
        return TokenStream2::new();
    };
    let comp_name = comp_ident.to_string();
    let comp_span = comp_ident.span();
    *idx += 1;

    // Skip optional `!` for backwards compatibility (`VStack!`, `Text!`, `Button!`, etc.)
    if *idx < tokens.len() {
        if let TokenTree::Punct(p) = &tokens[*idx] {
            if p.as_char() == '!' {
                *idx += 1;
            }
        }
    }

    // Optional parenthesized constructor arguments `( ... )`
    let mut ctor_group: Option<Group> = None;
    if *idx < tokens.len() {
        if let TokenTree::Group(g) = &tokens[*idx] {
            if g.delimiter() == Delimiter::Parenthesis {
                ctor_group = Some(g.clone());
                *idx += 1;
            }
        }
    }

    // Build base component expression (preserve `comp_ident` directly so Go-to-Definition on `Card` or `Button` works in rust-analyzer)
    let mut base_expr = match comp_name.as_str() {
        "Text" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#comp_ident::new() }
                } else if let Some(TokenTree::Literal(lit)) = inner.first() {
                    if is_string_literal(lit) {
                        let transformed = transform_rust_tokens(&inner);
                        quote! {
                            ::native_kit::#comp_ident {
                                props: vec![("label".to_string(), ::native_kit::PropValue::String(format!(#transformed)))],
                                children: vec![],
                            }
                        }
                    } else {
                        let transformed = transform_rust_tokens(&inner);
                        quote! {
                            ::native_kit::#comp_ident {
                                props: vec![("label".to_string(), ::native_kit::PropValue::String((#transformed).into()))],
                                children: vec![],
                            }
                        }
                    }
                } else {
                    let transformed = transform_rust_tokens(&inner);
                    quote! {
                        ::native_kit::#comp_ident {
                            props: vec![("label".to_string(), ::native_kit::PropValue::String((#transformed).into()))],
                            children: vec![],
                        }
                    }
                }
            } else {
                quote! { ::native_kit::#comp_ident::new() }
            }
        }
        "Icon" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#comp_ident::new("") }
                } else {
                    let transformed = rewrite_leading_dot_tokens("icon", comp_span, &inner);
                    quote! { ::native_kit::#comp_ident::new(#transformed) }
                }
            } else {
                quote! { ::native_kit::#comp_ident::new("") }
            }
        }
        "Image" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#comp_ident::new() }
                } else {
                    let transformed = transform_rust_tokens(&inner);
                    quote! {
                        ::native_kit::#comp_ident {
                            props: vec![("src".to_string(), ::native_kit::PropValue::String((#transformed).into()))],
                            children: vec![],
                        }
                    }
                }
            } else {
                quote! { ::native_kit::#comp_ident::new() }
            }
        }
        "TextInput" | "Switch" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#comp_ident::new() }
                } else {
                    let transformed = transform_rust_tokens(&inner);
                    quote! { ::native_kit::#comp_ident::new().value(#transformed) }
                }
            } else {
                quote! { ::native_kit::#comp_ident::new() }
            }
        }
        "Picker" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                let transformed = transform_rust_tokens(&inner);
                quote! { ::native_kit::#comp_ident::new(#transformed) }
            } else {
                quote! { ::native_kit::#comp_ident::new(Vec::<String>::new()) }
            }
        }
        "Spacer" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#comp_ident::new() }
                } else {
                    let transformed = transform_rust_tokens(&inner);
                    quote! { ::native_kit::#comp_ident::new().size((#transformed) as u32) }
                }
            } else {
                quote! { ::native_kit::#comp_ident::new() }
            }
        }
        "Button" => {
            let mut expr = quote! { ::native_kit::#comp_ident::new() };
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if !inner.is_empty() {
                    if let Some(TokenTree::Literal(lit)) = inner.first() {
                        if is_string_literal(lit) {
                            let transformed = transform_rust_tokens(&inner);
                            expr.extend(quote! { .child(format!(#transformed)) });
                        } else {
                            for child in parse_children_tokens(&inner) {
                                expr.extend(quote! { .children(#child) });
                            }
                        }
                    } else {
                        for child in parse_children_tokens(&inner) {
                            expr.extend(quote! { .children(#child) });
                        }
                    }
                }
            }
            expr
        }
        _ => {
            let mut expr = if is_component(&comp_name) {
                quote! { ::native_kit::#comp_ident::new() }
            } else {
                quote! { #comp_ident::new() }
            };
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                for child in parse_children_tokens(&inner) {
                    expr.extend(quote! { .children(#child) });
                }
            }
            expr
        }
    };

    let mut pre_mods = TokenStream2::new();
    let mut incomplete_mods = TokenStream2::new();
    consume_modifier_chain(
        tokens,
        idx,
        Some(&comp_name),
        &mut pre_mods,
        &mut incomplete_mods,
    );

    let mut children_calls = TokenStream2::new();
    let mut post_mods = TokenStream2::new();

    // Optional `{ children }` block for container or custom components
    let can_have_brace_children =
        is_container_component(&comp_name) || is_custom_component_ident(&comp_name);
    if can_have_brace_children && *idx < tokens.len() {
        if let TokenTree::Group(g) = &tokens[*idx] {
            if g.delimiter() == Delimiter::Brace && !is_raw_struct_brace(g) {
                let brace_tokens: Vec<TokenTree> = g.stream().into_iter().collect();
                let mut brace_idx = 0;
                // Also allow leading `.prop(...)` inside the `{ ... }` block itself
                consume_modifier_chain(
                    &brace_tokens,
                    &mut brace_idx,
                    Some(&comp_name),
                    &mut pre_mods,
                    &mut incomplete_mods,
                );
                for child in parse_children_tokens(&brace_tokens[brace_idx..]) {
                    children_calls.extend(quote! { .children(#child) });
                }
                *idx += 1;

                // Post-brace `.prop(...)` chain (backwards-compatible with `VStack { ... }.spacing(12)`)
                consume_modifier_chain(
                    tokens,
                    idx,
                    Some(&comp_name),
                    &mut post_mods,
                    &mut incomplete_mods,
                );
            }
        }
    }

    base_expr.extend(pre_mods);
    base_expr.extend(children_calls);
    base_expr.extend(post_mods);
    if !incomplete_mods.is_empty() {
        let mut fallback = if is_component(&comp_name) {
            quote! { ::native_kit::#comp_ident::new() }
        } else {
            quote! { #comp_ident::new() }
        };
        if let Some(g) = &ctor_group {
            if matches!(comp_name.as_str(), "Icon") {
                fallback = quote! { ::native_kit::#comp_ident::new("") };
            } else if matches!(comp_name.as_str(), "Picker") {
                fallback = quote! { ::native_kit::#comp_ident::new(Vec::<String>::new()) };
            }
            let _ = g;
        }
        return quote! {
            {
                let _ = #base_expr #incomplete_mods;
                #fallback
            }
        };
    }
    base_expr
}

fn parse_children_tokens(tokens: &[TokenTree]) -> Vec<TokenStream2> {
    let mut children = Vec::new();
    let mut idx = 0;

    while idx < tokens.len() {
        if let TokenTree::Punct(p) = &tokens[idx] {
            if p.as_char() == ',' || p.as_char() == ';' {
                idx += 1;
                continue;
            }
        }

        if is_component_start(tokens, idx) {
            let comp = parse_component_invocation(tokens, &mut idx);
            children.push(comp);
            continue;
        }

        let expr = parse_rust_child_expr(tokens, &mut idx);
        if !expr.is_empty() {
            children.push(expr);
        }
    }

    children
}

fn is_control_keyword(ident: &Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "if" | "else" | "match" | "for" | "in" | "while" | "loop"
    )
}

fn is_prefix_keyword(ident: &Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "if"
            | "else"
            | "match"
            | "for"
            | "in"
            | "while"
            | "loop"
            | "let"
            | "as"
            | "move"
            | "async"
            | "await"
            | "return"
            | "ref"
            | "mut"
    )
}

fn parse_rust_child_expr(tokens: &[TokenTree], idx: &mut usize) -> TokenStream2 {
    let mut out = Vec::new();
    let mut expect_operand = true;
    let mut waiting_for_control_brace = false;
    let mut in_closure_params = false;
    let mut after_closure_params = false;
    let mut after_colon = false;
    let mut angle_depth: usize = 0;

    while *idx < tokens.len() {
        let tok = &tokens[*idx];

        if angle_depth == 0 && !in_closure_params {
            if let TokenTree::Punct(p) = tok {
                if p.as_char() == ',' || p.as_char() == ';' {
                    *idx += 1;
                    break;
                }
            }
        }

        // If we already have a complete expression (`!expect_operand`) and are not inside `<...>` or `|...|`,
        // check whether the next token continues the current expression or starts a new sibling child.
        if !expect_operand && angle_depth == 0 && !in_closure_params {
            let continues = match tok {
                TokenTree::Punct(p) => matches!(
                    p.as_char(),
                    '.' | ':' | '?' | '!' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' | '=' | '<' | '>'
                ),
                TokenTree::Group(g) => match g.delimiter() {
                    Delimiter::Parenthesis | Delimiter::Bracket => true,
                    Delimiter::Brace => waiting_for_control_brace || after_closure_params,
                    Delimiter::None => true,
                },
                TokenTree::Ident(id) => matches!(id.to_string().as_str(), "else" | "as" | "in"),
                TokenTree::Literal(_) => false,
            };

            if !continues {
                break;
            }
        }

        // Consume token and update state
        match tok {
            TokenTree::Punct(p) => {
                let ch = p.as_char();
                if ch == '|' {
                    if expect_operand && !in_closure_params {
                        in_closure_params = true;
                    } else if in_closure_params {
                        in_closure_params = false;
                        after_closure_params = true;
                    }
                    expect_operand = true;
                    after_colon = false;
                } else if ch == ':' {
                    after_colon = true;
                    expect_operand = true;
                } else if ch == '<' && after_colon {
                    angle_depth += 1;
                    after_colon = false;
                    expect_operand = true;
                } else if ch == '<' && angle_depth > 0 {
                    angle_depth += 1;
                    expect_operand = true;
                } else if ch == '>' && angle_depth > 0 {
                    angle_depth -= 1;
                    expect_operand = angle_depth > 0;
                    after_colon = false;
                } else if ch == '?' {
                    expect_operand = false;
                    after_colon = false;
                } else if ch == '!' {
                    expect_operand = true;
                    after_colon = false;
                } else {
                    expect_operand = true;
                    after_colon = false;
                }
                out.push(tok.clone());
                *idx += 1;
            }
            TokenTree::Ident(id) => {
                after_colon = false;
                if is_control_keyword(id) {
                    waiting_for_control_brace = true;
                    expect_operand = true;
                } else if is_prefix_keyword(id) {
                    expect_operand = true;
                } else {
                    expect_operand = false;
                }
                out.push(tok.clone());
                *idx += 1;
            }
            TokenTree::Literal(_) => {
                after_colon = false;
                after_closure_params = false;
                expect_operand = false;
                out.push(tok.clone());
                *idx += 1;
            }
            TokenTree::Group(g) => {
                after_colon = false;
                if g.delimiter() == Delimiter::Brace {
                    waiting_for_control_brace = false;
                    after_closure_params = false;
                }
                expect_operand = false;
                out.push(tok.clone());
                *idx += 1;
            }
        }
    }

    if let Some(TokenTree::Ident(first_id)) = out.first() {
        if first_id == "if" {
            return transform_if_child_tokens(&out);
        }
        if first_id == "for" {
            return transform_for_child_tokens(&out);
        }
    }

    transform_rust_tokens(&out)
}

fn transform_for_child_tokens(tokens: &[TokenTree]) -> TokenStream2 {
    if let Some(last_idx) = tokens
        .iter()
        .rposition(|t| matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace))
    {
        let header = transform_rust_tokens(&tokens[..last_idx]);
        let body_group = match &tokens[last_idx] {
            TokenTree::Group(g) => g,
            _ => unreachable!(),
        };
        let inner: Vec<TokenTree> = body_group.stream().into_iter().collect();
        let children = parse_children_tokens(&inner);
        return quote! {
            {
                let mut __nk_for_nodes: ::std::vec::Vec<::native_kit::Node> = ::std::vec::Vec::new();
                #header {
                    #(
                        __nk_for_nodes.extend(::native_kit::IntoChildren::into_children(#children));
                    )*
                }
                __nk_for_nodes
            }
        };
    }
    transform_rust_tokens(tokens)
}

fn transform_if_child_tokens(tokens: &[TokenTree]) -> TokenStream2 {
    let mut out = TokenStream2::new();
    let mut idx = 0;
    let mut waiting_for_branch_brace = false;
    let mut last_was_else = false;
    let mut has_final_else = false;

    while idx < tokens.len() {
        match &tokens[idx] {
            TokenTree::Ident(id) if id == "if" => {
                waiting_for_branch_brace = true;
                last_was_else = false;
                has_final_else = false;
                out.extend(std::iter::once(tokens[idx].clone()));
                idx += 1;
            }
            TokenTree::Ident(id) if id == "else" => {
                last_was_else = true;
                out.extend(std::iter::once(tokens[idx].clone()));
                idx += 1;
            }
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace && (waiting_for_branch_brace || last_was_else) => {
                if last_was_else && !waiting_for_branch_brace {
                    has_final_else = true;
                }
                waiting_for_branch_brace = false;
                last_was_else = false;

                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                let children = parse_children_tokens(&inner);
                let branch_expr = if children.is_empty() {
                    quote! { ::std::vec::Vec::<::native_kit::Node>::new() }
                } else {
                    quote! {
                        {
                            let mut __nk_branch_nodes: ::std::vec::Vec<::native_kit::Node> = ::std::vec::Vec::new();
                            #(
                                __nk_branch_nodes.extend(::native_kit::IntoChildren::into_children(#children));
                            )*
                            __nk_branch_nodes
                        }
                    }
                };
                out.extend(branch_expr);
                idx += 1;
            }
            _ => {
                last_was_else = false;
                if let TokenTree::Group(g) = &tokens[idx] {
                    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                    let transformed_inner = transform_rust_tokens(&inner);
                    let mut new_group = Group::new(g.delimiter(), transformed_inner);
                    new_group.set_span(g.span());
                    out.extend(std::iter::once(TokenTree::Group(new_group)));
                } else {
                    out.extend(std::iter::once(tokens[idx].clone()));
                }
                idx += 1;
            }
        }
    }

    if !has_final_else {
        out.extend(quote! {
            else {
                ::std::vec::Vec::<::native_kit::Node>::new()
            }
        });
    }

    out
}

fn transform_rust_tokens(tokens: &[TokenTree]) -> TokenStream2 {
    let mut out = TokenStream2::new();
    let mut idx = 0;

    while idx < tokens.len() {
        // 1. Transform UI component invocations inside arbitrary Rust expressions (closures, if/else, method args)
        if is_component_start(tokens, idx) {
            let comp = parse_component_invocation(tokens, &mut idx);
            out.extend(comp);
            continue;
        }

        // 2. Transform `.method_name( .option )` leading-dot prop calls on any expression (e.g. `Gradient::linear().direction(.to_bottom_right).stop(.primary, 0.0)`)
        // Don't pre-transform inside `Return! { ... }`, `View! { ... }`, etc. so their own macro parsers see raw `.option` tokens
        if idx + 2 < tokens.len() {
            if let (TokenTree::Ident(mac_ident), TokenTree::Punct(bang), TokenTree::Group(_)) =
                (&tokens[idx], &tokens[idx + 1], &tokens[idx + 2])
            {
                if bang.as_char() == '!'
                    && (matches!(
                        mac_ident.to_string().as_str(),
                        "Return" | "View" | "view" | "render" | "ui"
                    ) || is_container_component(&mac_ident.to_string()))
                {
                    out.extend([
                        tokens[idx].clone(),
                        tokens[idx + 1].clone(),
                        tokens[idx + 2].clone(),
                    ]);
                    idx += 3;
                    continue;
                }
            }
        }

        if let TokenTree::Punct(dot) = &tokens[idx] {
            if dot.as_char() == '.' && idx + 2 < tokens.len() {
                if let (TokenTree::Ident(method_ident), TokenTree::Group(arg_group)) =
                    (&tokens[idx + 1], &tokens[idx + 2])
                {
                    if arg_group.delimiter() == Delimiter::Parenthesis {
                        let transformed_args =
                            transform_method_args(method_ident, arg_group.stream(), None);
                        let mut new_group = Group::new(Delimiter::Parenthesis, transformed_args);
                        new_group.set_span(arg_group.span());
                        out.extend([
                            TokenTree::Punct(dot.clone()),
                            TokenTree::Ident(method_ident.clone()),
                            TokenTree::Group(new_group),
                        ]);
                        idx += 3;
                        continue;
                    }
                }
            }
        }

        // 3. Transform `match expr { .variant => ... }` leading-dot patterns in match arms
        if let TokenTree::Ident(kw) = &tokens[idx] {
            if kw == "match" {
                out.extend(std::iter::once(tokens[idx].clone()));
                idx += 1;
                while idx < tokens.len() {
                    if let TokenTree::Group(g) = &tokens[idx] {
                        if g.delimiter() == Delimiter::Brace {
                            let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                            let transformed_inner = transform_rust_tokens(&inner);
                            let mut new_group = Group::new(Delimiter::Brace, transformed_inner);
                            new_group.set_span(g.span());
                            out.extend(std::iter::once(TokenTree::Group(new_group)));
                            idx += 1;
                            break;
                        }
                    }
                    out.extend(std::iter::once(tokens[idx].clone()));
                    idx += 1;
                }
                continue;
            }
        }

        // 4. Recursively transform inside Groups `( ... )`, `{ ... }`, `[ ... ]`
        if let TokenTree::Group(g) = &tokens[idx] {
            let inner: Vec<TokenTree> = g.stream().into_iter().collect();
            let transformed_inner = transform_rust_tokens(&inner);
            let mut new_group = Group::new(g.delimiter(), transformed_inner);
            new_group.set_span(g.span());
            out.extend(std::iter::once(TokenTree::Group(new_group)));
            idx += 1;
            continue;
        }

        out.extend(std::iter::once(tokens[idx].clone()));
        idx += 1;
    }

    out
}

fn expand_container_macro(comp_name: &str, input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = TokenStream2::from(input).into_iter().collect();
    let mut idx = 0;

    // If someone wrote `Page! { Page.title("Orders") { ... } }`, avoid double-wrapping `Page` in `Page`
    if comp_name == "Page" && is_component_start(&tokens, 0) {
        if let TokenTree::Ident(first_ident) = &tokens[0] {
            if first_ident == "Page" {
                let mut test_idx = 0;
                let expr = parse_component_invocation(&tokens, &mut test_idx);
                if test_idx == tokens.len() {
                    return TokenStream::from(expr);
                }
            }
        }
    }

    let type_ident = Ident::new(comp_name, Span::call_site());
    let mut base_expr = quote! { ::native_kit::#type_ident::new() };
    let mut pre_mods = TokenStream2::new();
    let mut incomplete_mods = TokenStream2::new();
    let mut post_mods = TokenStream2::new();

    // 1. Consume any leading `.prop(...)` modifiers at the top of `Page! { .title("Orders") .top_style(.hard) ... }`
    consume_modifier_chain(
        &tokens,
        &mut idx,
        Some(comp_name),
        &mut pre_mods,
        &mut incomplete_mods,
    );

    let mut children_calls = TokenStream2::new();

    // 2. If the user wrote `Page! { .title("Orders") { ...children... } }`, unwrap the `{ ...children... }` block
    if !pre_mods.is_empty() && idx < tokens.len() {
        if let TokenTree::Group(g) = &tokens[idx] {
            if g.delimiter() == Delimiter::Brace && !is_raw_struct_brace(g) {
                let mut lookahead = idx + 1;
                let mut temp_post = TokenStream2::new();
                let mut temp_inc = TokenStream2::new();
                consume_modifier_chain(
                    &tokens,
                    &mut lookahead,
                    Some(comp_name),
                    &mut temp_post,
                    &mut temp_inc,
                );
                if lookahead == tokens.len() {
                    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                    for child in parse_children_tokens(&inner) {
                        children_calls.extend(quote! { .children(#child) });
                    }
                    post_mods.extend(temp_post);
                    incomplete_mods.extend(temp_inc);
                    base_expr.extend(pre_mods);
                    base_expr.extend(children_calls);
                    base_expr.extend(post_mods);
                    base_expr.extend(incomplete_mods);
                    return TokenStream::from(base_expr);
                }
            }
        }
    }

    // 3. Special case for `Button!("Label")` when `Button!` is invoked with a single string literal or format args
    if comp_name == "Button" && pre_mods.is_empty() && !tokens.is_empty() {
        if let Some(TokenTree::Literal(lit)) = tokens.first() {
            if is_string_literal(lit) && tokens.len() > 1 {
                if let Some(TokenTree::Punct(p)) = tokens.get(1) {
                    if p.as_char() == ',' {
                        let transformed = transform_rust_tokens(&tokens);
                        return TokenStream::from(quote! {
                            native_kit::Button::new().child(format!(#transformed))
                        });
                    }
                }
            }
        }
    }

    // 4. Parse remaining tokens as children
    for child in parse_children_tokens(&tokens[idx..]) {
        children_calls.extend(quote! { .children(#child) });
    }

    base_expr.extend(pre_mods);
    base_expr.extend(children_calls);
    base_expr.extend(post_mods);
    base_expr.extend(incomplete_mods);
    TokenStream::from(base_expr)
}
