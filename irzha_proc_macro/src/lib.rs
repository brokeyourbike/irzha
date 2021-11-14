use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Хиба" => "Err",
        "Файно" => "Ok",
        "Строка" => "String",
        "Словник" => "HashMap",
        "Помилка" => "Error",
        "Можливо" => "Option",
        "Дещо" => "Some",
        "Нічого" => "None",
        "Результат" => "Result",
        "Власний" => "Self",
        "друкувати" => "println",
        "зупинка" => "break",
        "асинхронна" => "async",
        "чекати" => "await",
        "петля" => "loop",
        "рухатися" => "move",
        "ящик" => "crate",
        "недоступний_код" => "unreachable_code",
        "як" => "as",
        "константа" => "const",
        "конвенція" => "trait",
        "небезпечно" => "unsafe",
        "у" => "in",
        "із" => "from",
        "динамічний" => "dyn",
        "розпакувати" => "unwrap",
        "як_посилання" => "as_ref",
        "зовнішній" => "extern",
        "брехня" => "false",
        "функція" => "fn",
        "супер" => "super",
        "вставити" => "insert",
        "отримати" => "get",
        "дозволити" => "allow",
        "лайно" | "дідько" | "халепа" => "panic",
        "модуль" => "mod",
        "змінна" => "mut",
        "новий" => "new",
        "де" => "where",
        "для" => "for",
        "отримати_або_вставити_із" => "get_or_insert_with",
        "основна" => "main",
        "публічна" => "pub",
        "повернути" => "return",
        "реалізація" => "impl",
        "посилання" => "ref",
        "порівняти" => "match",
        "якщо" => "if",
        "інакше" => "else",
        "власний" => "self",
        "нехай" => "let",
        "статична" => "static",
        "структура" => "struct",
        "поки" => "while",
        "вжити" => "use",
        "до" => "into",
        "правда" => "true",
        "перелік" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn irzha(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
