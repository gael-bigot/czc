#[derive(Debug, Clone)]
pub enum CodeElement<'src> {
    Instruction(Instruction<'src>),
    Const(&'src str, Box<Expr<'src>>),
    Reference(RefBinding<'src>, Box<RValue<'src>>),
    LocalVar(TypedIdentifier<'src>, Option<Box<Expr<'src>>>),
    TempVar(TypedIdentifier<'src>, Option<Box<Expr<'src>>>),
    CompoundAssertEq(Box<Expr<'src>>, Box<Expr<'src>>),
    StaticAssert(Box<Expr<'src>>, Box<Expr<'src>>),
    Return(Box<Expr<'src>>),
    If(Box<Expr<'src>>, Vec<CodeElement<'src>>, Option<Vec<CodeElement<'src>>>),
    FuncCall(FunctionCall<'src>),
    Label(&'src str),
    Function(Function<'src>),
    Struct(Struct<'src>),
    Namespace(Namespace<'src>),
    Typedef(&'src str, Type<'src>),
    WithAttr(WithAttr<'src>),
    With(With<'src>),
    Hint(&'src str),
    Directive(Directive<'src>),
    Import(Import<'src>),
    AllocLocals,
    EmptyLine,
}

#[derive(Debug, Clone)]
pub enum Type<'src> {
    Felt,
    CodeOffset,
    Pointer(Box<Type<'src>>),
    Pointer2(Box<Type<'src>>),
    Tuple(Vec<NamedType<'src>>),
    Struct(&'src str),
}

#[derive(Debug, Clone)]
pub struct NamedType<'src> {
    pub name: &'src str,
    pub type_: Option<Type<'src>>,
}

#[derive(Debug, Clone)]
pub struct TypedIdentifier<'src> {
    pub modifier: Option<Modifier>,
    pub name: &'src str,
    pub type_: Option<Type<'src>>,
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Local,
}

#[derive(Debug, Clone)]
pub enum RefBinding<'src> {
    TypedIdentifier(TypedIdentifier<'src>),
    IdentifierList(Vec<TypedIdentifier<'src>>),
}

#[derive(Debug, Clone)]
pub enum RValue<'src> {
    CallInstruction(CallInstruction<'src>),
    Expr(Box<Expr<'src>>),
}

#[derive(Debug, Clone)]
pub enum CallInstruction<'src> {
    CallRel(Box<Expr<'src>>),
    CallAbs(Box<Expr<'src>>),
    CallLabel(&'src str),
}

#[derive(Debug, Clone)]
pub enum Instruction<'src> {
    AssertEq(Box<Expr<'src>>, Box<Expr<'src>>),
    JmpRel(Box<Expr<'src>>),
    JmpAbs(Box<Expr<'src>>),
    JmpToLabel(&'src str),
    Jnz(Box<Expr<'src>>, Box<Expr<'src>>),
    JnzToLabel(&'src str, Box<Expr<'src>>),
    Call(CallInstruction<'src>),
    Ret,
    AddAp(Box<Expr<'src>>),
    DataWord(Box<Expr<'src>>),
}

#[derive(Debug, Clone)]
pub struct FunctionCall<'src> {
    pub name: &'src str,
    pub implicit_args: Option<Vec<ExprAssignment<'src>>>,
    pub args: Vec<ExprAssignment<'src>>,
}

#[derive(Debug, Clone)]
pub enum ExprAssignment<'src> {
    Assign(&'src str, Box<Expr<'src>>),
    Expr(Box<Expr<'src>>),
}


#[derive(Debug, Clone)]
pub struct Function<'src> {
    pub decorators: Vec<&'src str>,
    pub name: &'src str,
    pub implicit_args: Option<Vec<TypedIdentifier<'src>>>,
    pub args: Vec<TypedIdentifier<'src>>,
    pub returns: Option<Type<'src>>,
    pub body: Vec<CodeElement<'src>>,
}

#[derive(Debug, Clone)]
pub struct Struct<'src> {
    pub decorators: Vec<&'src str>,
    pub name: &'src str,
    pub members: Vec<TypedIdentifier<'src>>,
}

#[derive(Debug, Clone)]
pub struct Namespace<'src> {
    pub decorators: Vec<&'src str>,
    pub name: &'src str,
    pub body: Vec<CodeElement<'src>>,
}

#[derive(Debug, Clone)]
pub struct WithAttr<'src> {
    pub name: &'src str,
    pub attr_values: Option<Vec<String>>,
    pub body: Vec<CodeElement<'src>>,
}

#[derive(Debug, Clone)]
pub struct With<'src> {
    pub aliases: Vec<AliasedIdentifier<'src>>,
    pub body: Vec<CodeElement<'src>>,
}

#[derive(Debug, Clone)]
pub struct AliasedIdentifier<'src> {
    pub name: &'src str,
    pub alias: Option<&'src str>,
}

#[derive(Debug, Clone)]
pub enum Directive<'src> {
    Builtins(Vec<&'src str>),
    Lang(&'src str),
}

#[derive(Debug, Clone)]
pub struct Import<'src> {
    pub module: &'src str,
    pub items: Vec<AliasedIdentifier<'src>>,
}

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    IntegerLiteral(u64),
    Identifier(&'src str),
    Register(Register),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>),
    Div(Box<Expr<'src>>, Box<Expr<'src>>),
    Minus(Box<Expr<'src>>),
    Deref(Box<Expr<'src>>),
    AddressOf(Box<Expr<'src>>),
    Cast(Box<Expr<'src>>, Box<Expr<'src>>),
    New(Box<Expr<'src>>),
    Eq(Box<Expr<'src>>, Box<Expr<'src>>),
    Neq(Box<Expr<'src>>, Box<Expr<'src>>),
    And(Box<Expr<'src>>, Box<Expr<'src>>),
    FunctionCall(&'src str, Vec<Box<ExprAssignment<'src>>>),
}

#[derive(Debug, Clone)]
pub enum Register {
    Ap,
    Fp,
}
