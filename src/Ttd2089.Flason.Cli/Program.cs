using System;
using System.Text;
using System.Text.Json;
using Ttd2089.Flason;
using Rfc6901JsonPointer = Ttd2089.Flason.Rfc6901.JsonPointer;
using Rfc6901ReferenceToken = Ttd2089.Flason.Rfc6901.ReferenceToken;
using Rfc6901ReferenceTokenType = Ttd2089.Flason.Rfc6901.ReferenceTokenType;

class Program
{
    static void Main()
    {
        using var stdin = Console.OpenStandardInput();
        var jsonTokenReader = new JsonTokenReader(stdin, new()
        {
            InitialBufferSize = 8,
            CommentHandling = JsonCommentHandling.Skip,
        });

        WriteFlason(jsonTokenReader);
    }

    static void WriteFlason(JsonTokenReader reader) => WriteFlason(reader, new(), reader.Next());

    static void WriteFlason(JsonTokenReader reader, Stack<Rfc6901ReferenceToken> jsonPointer, JsonToken? nextToken)
    {
        if (nextToken is not JsonToken token)
        {
            return;
        }

        switch (token.Type)
        {
            case JsonTokenType.StartArray:
                WriteFlasonArray(reader, jsonPointer);
                break;
            case JsonTokenType.StartObject:
                WriteFlasonObject(reader, jsonPointer);
                break;
            default:
                WriteFlasonScalar(jsonPointer, token);
                break;
        }
    }

    static void WriteFlasonObject(JsonTokenReader reader, Stack<Rfc6901ReferenceToken> jsonPointer)
    {
        while (reader.Next() is JsonToken token && token.Type != JsonTokenType.EndObject)
        {
            var propertyReferenceToken = new Rfc6901ReferenceToken(
                Rfc6901ReferenceTokenType.Property,
                Encoding.UTF8.GetString(token.Utf8ValueBytes));

            jsonPointer.Push(propertyReferenceToken);

            WriteFlason(reader, jsonPointer, reader.Next());

            jsonPointer.Pop();
        }
    }

    static void WriteFlasonArray(JsonTokenReader reader, Stack<Rfc6901ReferenceToken> jsonPointer)
    {
        var index = 0;
        while (reader.Next() is JsonToken token && token.Type != JsonTokenType.EndArray)
        {
            var indexReferenceToken = new Rfc6901ReferenceToken(Rfc6901ReferenceTokenType.Index, $"{index}");
            jsonPointer.Push(indexReferenceToken);
            WriteFlason(reader, jsonPointer, token);
            jsonPointer.Pop();
            ++index;
        }
    }

    static void WriteFlasonScalar(Stack<Rfc6901ReferenceToken> jsonPointer, JsonToken token)
    {
        var jsonValue = token.Type == JsonTokenType.String
            ? $"\"{Encoding.UTF8.GetString(token.Utf8ValueBytes)}\""
            : $"{Encoding.UTF8.GetString(token.Utf8ValueBytes)}";

        // todo: For some reason this wont print the '𝄞' character in pwsh. The bytes are correct so I THINK it's a
        // a problem with the terminal/font but I'm not 100% sure.
        Console.WriteLine($"\"{new Rfc6901JsonPointer(jsonPointer.Reverse())}\": {jsonValue}");
    }
}