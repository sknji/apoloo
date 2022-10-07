
pub(crate) enum Precedence {
    PrecedenceNone,
    PrecedenceAssignment,
    PrecedenceOr,
    PrecedenceAnd,
    PrecedenceEquality,
    PrecedenceComparison,
    PrecedenceTerm,
    PrecedenceFactor,
    PrecedenceUnary,
    PrecedenceCall,
    PrecedencePrimary,
}
