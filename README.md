# FlaSON

FlaSON is a flattened representation for JSON values that uses a series of assignment-like expressions to represent the value.

For example, the following JSON and FlaSON texts represent the same value:

```json
{
    "foo": "bar",
    "baz": [ "b", "a", "z" ]
}
```

```flason
. = {}
.foo = "bar" 
.baz = []
.baz[0] = "b"
.baz[1] = "a"
.baz[2] = "z"
```


## ABNF

```abnf
flaSON-text = ws 1*( statement ws )

statement = assignment [ *WSP meta ]

assignment = member-access *WSP eq *WSP value

member-access = todo ; a valid JavaScript member access expression, e.g. `.foo[1].bar["irk !@#$%"].baz`

value = empty-collection / scalar 

empty-collection = empty-object / empty-array

empty-object = begin-object *WSP end-object
begin-object = "{"
end-object   = "}"

empty-array = begin-array *WSP end-array
begin-array = "["
end-array   = "]"

scalar = JSON-string / JSON-number / JSON-literal

JSON-string = todo ; a valid JSON string

JSON-number = todo ; a valid JSON number

JSON-literal = "true" / "false" / "null"

meta            = meta-start *( *WSP meta-tag )
meta-tag        = meta-id eq meta-value ; NOTE no spaces surrounding eq
meta-id         = 1*lc-alpha *( ( "_" / "." ) 1*lc-alpha )
meta-value      = single-line-JSON-value

single-line-JSON-value = todo ; any value value but with no newlines allowed

lc-alpha =  %x61-7A ; a-z

int = zero / ( digit1-9 *DIGIT)
DIGIT = zero / digit1-9
digit1-9 = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9"
zero = "0"

eq = "="
meta-start = ";"

ws = *( WSP / newline )
WSP = SP / HTAB
SP   = %x20
HTAB = "\t"
newline = [ CR ] LF
CR   = "\r"
LF   = "\n"

todo = "todo"
```