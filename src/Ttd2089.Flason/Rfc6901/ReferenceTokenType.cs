namespace Ttd2089.Flason.Rfc6901;

public enum ReferenceTokenType
{
    /// <summary>
    /// Indicates that an <see cref="ReferenceToken"/> refers to the name of a property
    /// within a JSON object.
    /// </summary>
    Property,

    /// <summary>
    /// Indicates that an <see cref="ReferenceToken"/> refers to an index in a JSON array.
    /// </summary>
    Index,
}