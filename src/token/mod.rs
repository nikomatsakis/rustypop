pub enum Token {
    Bang, // "!"
    Pound, // "#"
    Dollar, // "$"
    Percent, // "%"
    ParenOpen, // "("
    ParenClose, // ")"
    Star, // "*"
    Plus, // "+"
    Comma, // ","
    Dash, // "-"
    Dot, // "."
    Slash, // "/"
    Colon, // ":"
    Semi, // ";"
    Equals, // "="
    QuestionMark, // "?"
    At, // "@"
    SquareBracketOpen, // "["
    SquareBracketClose, // "]"
    Hat, // "^"
    CurlyBraceOpen, // "{"
    CurlyBraceClose, // "}"
    Twiddle, // "~"
    EqualsEquals, // "=="
    BangEquals, // "!="
    DashEquals, // "-="
    AmpersandEquals, // "&="
    PipeEquals, // "|="
    PlusEquals, // "+="
    StarEquals, // "*="
    SlashEquals, // "/="
    HatEquals, // "^="
    PercentEquals, // "%="
    DotDot, // ".."
    DotDotDot, // "..."
    ColonColon, // "::"
    ThinArrow, // "->"
    FatArrow, // "=>"
    LessLessEqual, // "<<="
    RightRightEqual, // ">>="
    LeftThinArrow, // "<-"
    LessEqual, // "<="
    RightEqual, // ">="
    Underscore, // "_"

    // Compute tokens. The notation `x[y]` means "an x adjacent to
    // a y" and the notation `x[]` means "an x adjacent to nothing
    // of interest". So if we see `<<`, we will produce two
    // tokens: `<[<] <[]`, but if we see `<<=`, we would produce
    // three tokens: `<[<=] <[=] =`.
    PipeFollowedByPipe, // "|[|]"
    PipeFollowedByOther, // "|[]"
    AmpersandFollowedByAmpersand, // "&[&]"
    AmpersandFollowedByOther, // "&[]"
    LessFollowedByLess, // "<[<]"
    LessFollowedByOther, // "<[]"
    GreaterFollowedByGreater, // ">[>]"
    GreaterFollowedByOther, // ">[]"

    KeywordSelf, // "self"
    KeywordSuper, // "super"
    KeywordStatic, // "static"
    KeywordAs, // "as"
    KeywordBreak, // "break"
    KeywordCrate, // "crate"
    KeywordElse, // "else"
    KeywordEnum, // "enum"
    KeywordExtern, // "extern"
    KeywordFalse, // "false"
    KeywordFn, // "fn"
    KeywordFor, // "for"
    KeywordIf, // "if"
    KeywordImpl, // "impl"
    KeywordIn, // "in"
    KeywordLet, // "let"
    KeywordLoop, // "loop"
    KeywordMatch, // "match"
    KeywordMod, // "mod"
    KeywordMove, // "move"
    KeywordMut, // "mut"
    KeywordPriv, // "priv"
    KeywordPub, // "pub"
    KeywordRef, // "ref"
    KeywordReturn, // "return"
    KeywordStruct, // "struct"
    KeywordTrue, // "true"
    KeywordTrait, // "trait"
    KeywordType, // "type"
    KeywordUnsafe, // "unsafe"
    KeywordUse, // "use"
    KeywordWhile, // "while"
    KeywordContinue, // "continue"
    KeywordBox, // "box"
    KeywordConst, // "const"
    KeywordWhere, // "where"
    KeywordTypeof, // "typeof"

    LiteralByte,
    LiteralChar,
    LiteralInteger,
    LiteralFloat,
    LiteralString,
    LiteralStringRaw,
    LiteralByteString,
    LiteralByteStringRaw,
    Identifier,
    Lifetime,

    InnerDocComment,
    OuterDocComment,
    Shebang,
    ShebangLine,
}
