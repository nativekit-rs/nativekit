#![allow(non_snake_case)]
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

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
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let result = rt.block_on(async { #fn_name().await });
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

#[proc_macro]
pub fn View(input: TokenStream) -> TokenStream {
    expand_view_macro(input)
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
    let children = parse_children_tokens(&tokens);
    if children.len() == 1 {
        let single = &children[0];
        return TokenStream::from(quote! { #single });
    }
    let mut out = quote! { ::native_kit::VStack::new() };
    for child in children {
        out.extend(quote! { .children(#child) });
    }
    TokenStream::from(out)
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

fn is_component_start_ignoring_prev(tokens: &[TokenTree], idx: usize) -> bool {
    let Some(TokenTree::Ident(ident)) = tokens.get(idx) else {
        return false;
    };
    let name = ident.to_string();
    if !is_component(&name) {
        return false;
    }
    if name == "Spacer" {
        return true;
    }
    match tokens.get(idx + 1) {
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
    }
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
            let table_ident = Ident::new(prop_table, prop_span);
            let rest_transformed = transform_rust_tokens(&arg_tokens[1..]);
            return quote! { ::native_kit::__props::#table_ident #first_dot #rest_transformed };
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
    let method_name = method_ident.to_string();
    let table_name = if comp_name == Some("Icon") && method_name == "variant" {
        "icon_variant"
    } else {
        method_name.as_str()
    };

    if table_name == "icon_variant" || is_known_dot_prop(table_name) {
        rewrite_leading_dot_tokens(table_name, method_ident.span(), &arg_tokens)
    } else {
        transform_rust_tokens(&arg_tokens)
    }
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

                // If `method_ident` is actually the next sibling component (e.g. user typed `.` at the end of a line before `Button(...)`)
                if is_component_start_ignoring_prev(tokens, *idx + 1) {
                    incomplete_mods.extend(std::iter::once(TokenTree::Punct(dot.clone())));
                    *idx += 1;
                    break;
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

    // Build base component expression
    let type_ident = Ident::new(&comp_name, comp_span);
    let mut base_expr = match comp_name.as_str() {
        "Text" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#type_ident::new() }
                } else if let Some(TokenTree::Literal(lit)) = inner.first() {
                    if is_string_literal(lit) {
                        let transformed = transform_rust_tokens(&inner);
                        quote! {
                            ::native_kit::#type_ident {
                                props: vec![("label".to_string(), ::native_kit::PropValue::String(format!(#transformed)))],
                                children: vec![],
                            }
                        }
                    } else {
                        let transformed = transform_rust_tokens(&inner);
                        quote! {
                            ::native_kit::#type_ident {
                                props: vec![("label".to_string(), ::native_kit::PropValue::String((#transformed).into()))],
                                children: vec![],
                            }
                        }
                    }
                } else {
                    let transformed = transform_rust_tokens(&inner);
                    quote! {
                        ::native_kit::#type_ident {
                            props: vec![("label".to_string(), ::native_kit::PropValue::String((#transformed).into()))],
                            children: vec![],
                        }
                    }
                }
            } else {
                quote! { ::native_kit::#type_ident::new() }
            }
        }
        "Icon" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#type_ident::new("") }
                } else {
                    let transformed = rewrite_leading_dot_tokens("icon", comp_span, &inner);
                    quote! { ::native_kit::#type_ident::new(#transformed) }
                }
            } else {
                quote! { ::native_kit::#type_ident::new("") }
            }
        }
        "Image" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#type_ident::new() }
                } else {
                    let transformed = transform_rust_tokens(&inner);
                    quote! {
                        ::native_kit::#type_ident {
                            props: vec![("src".to_string(), ::native_kit::PropValue::String((#transformed).into()))],
                            children: vec![],
                        }
                    }
                }
            } else {
                quote! { ::native_kit::#type_ident::new() }
            }
        }
        "TextInput" | "Switch" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#type_ident::new() }
                } else {
                    let transformed = transform_rust_tokens(&inner);
                    quote! { ::native_kit::#type_ident::new().value(#transformed) }
                }
            } else {
                quote! { ::native_kit::#type_ident::new() }
            }
        }
        "Picker" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                let transformed = transform_rust_tokens(&inner);
                quote! { ::native_kit::#type_ident::new(#transformed) }
            } else {
                quote! { ::native_kit::#type_ident::new(Vec::<String>::new()) }
            }
        }
        "Spacer" => {
            if let Some(g) = &ctor_group {
                let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                if inner.is_empty() {
                    quote! { ::native_kit::#type_ident::new() }
                } else {
                    let transformed = transform_rust_tokens(&inner);
                    quote! { ::native_kit::#type_ident::new().size(#transformed) }
                }
            } else {
                quote! { ::native_kit::#type_ident::new() }
            }
        }
        "Button" => {
            let mut expr = quote! { ::native_kit::#type_ident::new() };
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
            let mut expr = quote! { ::native_kit::#type_ident::new() };
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

    // Optional `{ children }` block for container components
    if is_container_component(&comp_name) && *idx < tokens.len() {
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
    base_expr.extend(incomplete_mods);
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

    transform_rust_tokens(&out)
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

        // 3. Recursively transform inside Groups `( ... )`, `{ ... }`, `[ ... ]`
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
