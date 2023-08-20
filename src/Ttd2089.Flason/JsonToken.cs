using System.Diagnostics.CodeAnalysis;
using System.Text.Json;

namespace Ttd2089.Flason;

public readonly record struct JsonToken
{
    [SetsRequiredMembers]
    public JsonToken(JsonTokenType type, ReadOnlySpan<byte> value, int depth, bool valueIsEscaped)
    {
        Type = type;
        Utf8ValueBytes = value.ToArray();
        Depth = depth;
        ContainsEscapeSequences = valueIsEscaped;
    }

    public required JsonTokenType Type { get; init; }

    /// <summary>
    /// The UTF8 bytes representing the value of the JSON token.
    /// </summary>
    /// <remarkks>
    /// <see cref="JsonTokenType.PropertyName"/> and <see cref="JsonTokenType.String"/> values do
    /// not include the surrounding double quotes, and <see cref="JsonTokenType.Comment"/> values
    /// do not include the leading or surrounding comment markers.
    /// </remarkks>
    public required byte[] Utf8ValueBytes { get; init; }

    /// <summary>
    /// The depth of the token within the JSON value it came from.
    /// </summary>
    public required int Depth { get; init; }

    /// <summary>
    /// Indicates whether <see cref="Utf8ValueBytes"/> contains escape sequences as described by
    /// RFC8259: The JavaScript Object Notation (JSON) Data Interchange Format in section 7.
    /// </summary>
    /// <remarks>
    /// The value of this property is only meaningful when <see cref="Type"/> is
    /// <see cref="JsonTokenType.String"/> or <see cref="JsonTokenType.PropertyName"/>.
    /// </remarks>
    public required bool ContainsEscapeSequences { get; init; }
}
