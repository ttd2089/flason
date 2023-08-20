using System.Diagnostics.CodeAnalysis;

namespace Ttd2089.Flason.Rfc6901;

public readonly record struct JsonPointer
{
    private readonly Lazy<string> _stringRepresentation;

    [SetsRequiredMembers]
    public JsonPointer(IEnumerable<ReferenceToken> referenceTokens)
    {
        ReferenceTokens = referenceTokens?.ToList() ?? throw new ArgumentNullException(nameof(referenceTokens));

        _stringRepresentation = new(GetStringRepresentation);
    }

    public required IReadOnlyList<ReferenceToken> ReferenceTokens { get; init; }

    public override string ToString() => _stringRepresentation.Value;

    private string GetStringRepresentation() => string.Join("", ReferenceTokens.Select(x => $"/{x.Name}"));
}
