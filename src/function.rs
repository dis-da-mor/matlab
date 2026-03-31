use std::{collections::HashMap, hash::Hash, sync::LazyLock};

use crate::tokeniser::{Token, TokenType, Tuple, TupleType};

type Call = fn(Token) -> Token;

pub(crate) struct Function {
    display_name: &'static str,
    calls: HashMap<TokenType, Call>,
}

pub(crate) static BUILT_IN: LazyLock<HashMap<&'static str, Function>> = LazyLock::new(|| {
    map_of([(
        "tan",
        Function {
            display_name: "tangent",
            calls: map_of([(TokenType::Number, Call::from(tan_single))]),
        },
    )])
});

fn tan_single(input: Token) -> Token {
    input
}

fn map_of<T: Hash + Eq, U, S: Into<U>>(source: impl IntoIterator<Item = (T, S)>) -> HashMap<T, U> {
    HashMap::from_iter(source.into_iter().map(|(t, s)| (t, s.into())))
}
