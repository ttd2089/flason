using System.Diagnostics.CodeAnalysis;

namespace Ttd2089.Flason.Rfc6901;

public readonly record struct ReferenceToken
{
    /// <summary>
    /// Initializes a new <see cref="ReferenceToken"/>.
    /// </summary>
    /// <param name="name">
    /// The name of the <see cref="ReferenceToken"/>. See <paramref name="nameIsEscaped"/>.
    /// </param>
    /// <param name="escapeName">
    /// When <paramref name="nameIsEscaped"/> is <c>false</c>, the default value, the value of
    /// <paramref name="name"/> will be escaped as described by RFC 6901. Set
    /// <paramref name="nameIsEscaped"/> to <c>true</c> if <paramref name="name"/> is already
    /// escaped to prevent escaping the '~' characters in the existing escape sequences.
    /// </param>
    [SetsRequiredMembers]
    public ReferenceToken(ReferenceTokenType type, string name, bool nameIsEscaped = false)
    {
        ArgumentNullException.ThrowIfNull(name);

        Type = type;
        Name = nameIsEscaped ? name : name.Replace("~", "~0").Replace("/", "~1");
    }

    public required ReferenceTokenType Type { get; init; }

    public required string Name { get; init; }
}
