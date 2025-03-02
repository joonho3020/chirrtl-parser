
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Info(pub String);

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct Width(pub u32);

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Identifier {
    ID(u32),
    Name(String),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Reference {
    Ref(Identifier),
    RefDot(Box<Reference>, Identifier),
    RefIdxInt(Box<Reference>, Identifier)
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum PrimOp2Expr {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Lt,
    Leq,
    Gt,
    Geq,
    Eq,
    Neq,
    Dshl,
    Dshr,
    And,
    Or,
    Xor,
    Cat,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum PrimOp1Expr {
    AsUInt,
    AsSInt,
    AsClock,
    AsAsyncReset,
    Cvt,
    Neg,
    Not,
    Andr,
    Orr,
    Xorr,
}


#[derive(Debug, Clone, PartialEq, Hash)]
pub enum PrimOp1Expr1Int {
    Pad,
    Shl,
    Shr,
    Head,
    Tail,
    BitSel,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum PrimOp1Expr2Int {
    BitSelRange,
}

impl From<String> for PrimOp2Expr {
    fn from(value: String) -> Self {
        match value.as_str() {
            "add" => { Self::Add },
            "sub" => { Self::Sub },
            "mul" => { Self::Mul },
            "div" => { Self::Div },
            "rem" => { Self::Rem },
            "lt"  => { Self::Lt },
            "leq"  => { Self::Leq },
            "gt"  => { Self::Gt },
            "geq"  => { Self::Geq },
            "eq"  => { Self::Eq },
            "neq"  => { Self::Neq },
            "dshl"  => { Self::Dshl },
            "dshr"  => { Self::Dshr },
            "and"  => { Self::And },
            "or"  => { Self::Or },
            "xor"  => { Self::Xor },
            "cat"  => { Self::Cat },
            _ => {
                panic!("Unrecognized operator {}", value);
            }
        }
    }
}

impl From<String> for PrimOp1Expr {
    fn from(value: String) -> Self {
        match value.as_str() {
            "asUInt"  => { Self::AsUInt },
            "asSInt"  => { Self::AsSInt },
            "asClock"  => { Self::AsClock },
            "asAsyncReset"  => { Self::AsAsyncReset },
            "cvt"  => { Self::Cvt },
            "neg"  => { Self::Neg },
            "not"  => { Self::Not },
            "andr"  => { Self::Andr },
            "orr"  => { Self::Orr },
            "xorr"  => { Self::Xorr },
            _ => {
                panic!("Unrecognized operator {}", value);
            }
        }
    }
}

impl From<String> for PrimOp1Expr1Int {
    fn from(value: String) -> Self {
        match value.as_str() {
            "pad"  => { Self::Pad },
            "shl"  => { Self::Shl },
            "shr"  => { Self::Shr },
            "head"  => { Self::Head },
            "tail"  => { Self::Tail },
            _ => { Self::BitSel }
        }
    }
}

impl From<String> for PrimOp1Expr2Int {
    fn from(value: String) -> Self {
        assert!(value.contains("bit"));
        Self::BitSelRange
    }
}

pub type Exprs = Vec<Box<Expr>>;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Expr {
    UIntNoInit(Width),
    UIntInit(Width, i64),
    SIntNoInit(Width),
    SIntInit(Width, i64),
    Reference(Reference),
    Mux(Box<Expr>, Box<Expr>, Box<Expr>),
    ValidIf(Box<Expr>, Box<Expr>),
    PrimOp2Expr(PrimOp2Expr, Box<Expr>, Box<Expr>),
    PrimOp1Expr(PrimOp1Expr, Box<Expr>),
    PrimOp1Expr1Int(PrimOp1Expr1Int, Box<Expr>, u32),
    PrimOp1Expr2Int(PrimOp1Expr2Int, Box<Expr>, u32, u32),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TypeGround {
    Clock,
    Reset,
    AsyncReset,
    UInt(Option<Width>),
    SInt(Option<Width>),
// ProbeType
// AnalType,
// FixedType
}

pub type Fields = Vec<Box<Field>>;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Field {
    Straight(Identifier, Box<Type>),
    Flipped(Identifier, Box<Type>),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum TypeAggregate {
    Fields(Box<Fields>),
    Array(Box<Type>, i64),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Type {
    TypeGround(TypeGround),
    ConstTypeGround(TypeGround),
    TypeAggregate(Box<TypeAggregate>),
    ConstTypeAggregate(Box<TypeAggregate>),
}

pub type Stmts = Vec<Box<Stmt>>;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Stmt {
    Wire(Identifier, Type, Info),
    Reg(Identifier,  Type, Expr, Info),
    RegReset(Identifier, Type, Expr, Expr, Expr, Info),
// Memory()
// ChirrtlMemory()
// ChirrtlMemoryPort()
    Inst(Identifier, Identifier, Info),
    Node(Identifier, Expr, Info),
    Connect(Expr, Expr, Info),
// Connect(Reference, Read, Expr, Info),
// Reference <- ???
    Invalidate(Expr, Info),
// Define(Define, Reference, Probe, Info),
// Define(Define, Reference, Expr, Probe, Info),
// Attach(References)
    When(Expr, Info, Stmts, Option<Stmts>),
// Stop(Expr, Expr, u64, Info),
// Stop(Expr, Expr, u64, Info),
    Printf(Expr, Expr, String, Exprs, Info),
    Assert(Expr, Expr, Expr, String, Info),
}

impl Stmt {
    pub fn traverse(&self) {
        match self {
            Self::When(e, i, tstmts, fstmts_opt) => {
                println!("When, {:?}, {:?}", e, i);
                for tstmt in tstmts.iter() {
                    println!("{:?}", tstmt);
                }
                match fstmts_opt {
                    Some(fstmts) => {
                        println!("ELSE");
                        for fstmt in fstmts.iter() {
                            println!("{:?}", fstmt);
                        }
                    }
                    None => {
                    }
                }
            }
            _ => {
                println!("{:?}", self);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Port {
    Input(Identifier, Type, Info),
    Output(Identifier, Type, Info),
}

pub type Ports = Vec<Box<Port>>;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Module  {
    pub name: Identifier,
    pub ports: Ports,
    pub stmts: Stmts,
    pub info: Info,
}

impl Module {
    pub fn new(name: Identifier, ports: Ports, stmts: Stmts, info: Info) -> Self {
        Self { name, ports, stmts, info, }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct DefName(Identifier);

impl From<Identifier> for DefName {
    fn from(value: Identifier) -> Self {
        Self(value)
    }
}

pub type Parameters = Vec<Box<Parameter>>;

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Parameter {
    IntParam(Identifier, i64),
    StringParam(Identifier, String),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ExtModule {
    pub name: Identifier,
    pub ports: Ports,
    pub defname: DefName,
    pub params: Parameters,
    pub info: Info,
}

impl ExtModule {
    pub fn new(name: Identifier, ports: Ports, defname: DefName, params: Parameters, info: Info) -> Self {
        Self { name, ports, defname, params, info }
    }
}

#[warn(dead_code)]
pub struct IntModule {
    pub name: Identifier,
    pub ports: Ports,
    pub params: Parameters,
    pub info: Info,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Version(pub u32, pub u32, pub u32);

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum CircuitModule {
    Module(Module),
    ExtModule(ExtModule),
// IntModule(IntModule),
}

pub type CircuitModules = Vec<Box<CircuitModule>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Circuit {
    pub version: Version,
    pub name: Identifier,
// TODO: pub annos: String,
    pub info: Info,
    pub modules: CircuitModules,
}

impl Circuit {
    pub fn new(version: Version, name: Identifier, info: Info, modules: CircuitModules) -> Self {
        Self { version, name, info, modules }
    }
}
