using System.Diagnostics.CodeAnalysis;
using System.Text.Json;

namespace Ttd2089.Flason;

public readonly struct JsonTokenReaderOptions
{
    public JsonTokenReaderOptions() { }

    public readonly int InitialBufferSize { get; init; } = 512;

    public readonly int MaxDepth { get; init; } = int.MaxValue;

    public readonly bool AllowTrailingCommas { get; init; }  = true;

    public readonly JsonCommentHandling CommentHandling { get; init; } = JsonCommentHandling.Skip;
}
