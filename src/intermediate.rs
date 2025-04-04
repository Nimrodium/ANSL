pub struct AbstractSyntaxTree {}

pub struct Scope {
    inner: Vec<Scope>,
}

enum AnslVariableType {
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),

    Signed8(i8),
    Signed16(i16),
    Signed32(i32),
    Signed64(i64),

    Array(Vec<AnslVariableType>),
    Char(char),
    String(String),
}

struct Variable {
    content: AnslVariableType,
    // relative offset, first variable pushed is 0 and so on.
    address: Address,
}

enum MemoryPartition {
    Stack,
    Heap,
}

struct Address {
    partition: MemoryPartition,
    heap_label: Option<String>,
    stack_id: Option<usize>,
}
struct RawArray {
    content: Vec<String>,
    length: usize,
    ansl_type: AnslVariableType,
}

struct TokenizedArray {}
