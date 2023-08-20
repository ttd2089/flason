# FlaSON

`FlaSON` is flattened representation of a JSON value. Each scalar value in the JSON value is represented as a key value pair where the key is the RFC 6902 JSON pointer and the value is the JSON value.


E.g.

```bash
> echo '{"obj":{"foo": "bar"}, "arr": ["foo", {"bar": "baz"}]}' | flason
"/obj/foo": "bar"
"/arr/0": "foo"
"/arr/1/bar": "baz"
```
