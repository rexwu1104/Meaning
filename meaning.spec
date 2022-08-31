Syntax
    ::= Statements
    ::= MacroDefinition

Statements
    ::= Statements Statement
    ::= Statement

Statement
    ::= TypeDefinition
    ::= BlockStatement
    ::= Expression ";"
    ::= RetExpression
    ::= FunctionDefinition
    ::= Decorated

MacroDefinition ::= "#" MacroName Identifier* ("(" Expression ")")* { MacroBody }
MacroName ::= "symbol" | "function" | "if" | "elif" | "else" | "attribute"
MacroBody ::= Statements

BlockStatement ::= BlockPrefix* "{" Statements "}"
BlockPrefix
    ::= Identifier
    ::= CallFunction
    ::= "if" Condition
    ::= "elif" Condition
    ::= "else"
    ::= "match" Expression
    ::= Expression
    ::= "when" Expression
    ::= "==" Expression
    ::= ">=" Expression
    ::= "<=" Expression
    ::= "!=" Expression
    ::= ">" Expression
    ::= "<" Expression
    ::= "for" Identifier "in" Expression
    ::= "while" Expression
    ::= "loop"

DefineParam ::= "..."* Identifier ":" Type
DefineParams
    ::= DefineParams "," DefineParam
    ::= DefineParam

Types
    ::= Types "," Type
    ::= Type

Type 
    ::= "str"
    ::= "bool"
    ::= "chr"
    ::= "const" Type
    ::= "i8"
    ::= "i16"
    ::= "i32"
    ::= "i64"
    ::= "i128"
    ::= "u8"
    ::= "u16"
    ::= "u32"
    ::= "u64"
    ::= "u128"
    ::= "f32"
    ::= "f64"
    ::= "f128"
    ::= "[]" Type
    ::= "[" Types "]"
    ::= "{" Type ";" Type "}"
    ::= "{-" Type ";" Type ";" Type "-}"
    ::= "(" Types ")" "->" Type
    ::= Identifier

ClassDefinition ::= "class" Identifier ("<" GenericTypes ">")* (":" Identifiers)* "{" ClassStatement "}"
ClassStatement
    ::= ClassProperty
    ::= ClassMethod

ClassProperty ::= Identifier ":" Type ("by" Identifier)* ";"
ClassMethod ::= Identifier "(" DefineParams ")" "->" Type "{" Statements "}"

RetExpression ::= Expression
Expression
    ::= CallFunction
    ::= AssignVariable

CallFunction
    ::= Identifier "(" Values ")"
    ::= Identifier "(" ValuesAssign ")"

AssignVariable
    ::= "var"* Identifier "=" Expression
    ::= "var"* Identifier "=" BlockStatement

Value
    ::= Identifier
    ::= Number
    ::= Boolean
    ::= String
    ::= Char
    ::= Map
    ::= DataFrame
    ::= Array
    ::= Lambda

Values
    ::= Values "," ("...")* Value
    ::= ("...")* Value

FunctionDefinition
    ::= "var" Identifier "(" DefineParams ")" "->" Type
    ::= "override" Identifier "(" DefineParams ")" "->" Type
    ::= "operator" '"' Op '"' "(" DefineParams ")" "->" Type

Identifier ::= [a-zA-Z_][a-zA-Z0-9_]*
Number
    ::= [0-9]+
    ::= (0x)[0-9a-fA-F]+
    ::= (0b)[01]+
    ::= (0o)[0-7]+
    ::= [0-9]+\.[0-9]* | [0-9]*\.[0-9]+

Boolean
    ::= "true"
    ::= "false"

String ::= \"[^\n]*\"
Char
    ::= \'^[\\]{1}\'
    ::= \'\\u\{[a-fA-F0-9]{1,4}\}\'
    ::= \'\\[\'\\trnbf]\'

Map ::= "{" KeyPairs "}"
DataFrame ::= "{-" FrameRows "-}"
Array ::= "[" Values "]"
Lambda ::= Type "{" DefineParams "->" Statements "}"

KeyPair ::= Identifier ":" Expression
KeyPairs
    ::= KeyPairs "," KeyPair
    ::= KeyPair

FrameRow ::= Values
FrameRows
    ::= FrameRows ";" FrameRow
    ::= FrameRow

Decorator ::= "@" "[" Values "]"
Decorated ::= Decorator Statement