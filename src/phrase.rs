/// Phrase of words
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Phrase {
    /// A noun phrase functions as a **Noun**.
    /// - The family enjoyed *a picnic*.
    /// - *This old house* fell over.
    /// - *Your ugly dog* ate my slipper.
    NounPhr,
    /// A verb phrase functions as a **Verb**.
    /// - Herbert *drove* the car.
    /// - The friends *are walking* in the park.
    /// - The rock *could fall* at any time.
    VerbPhr,
    /// An adjective phrase functions as an **Adjective** for a noun.
    /// It can be separate from a noun phrase in a sentence using a form of
    /// "to be" as the verb.  This consists of an adjective plus any words
    /// that modify it (usually intensifiers).
    /// - The stunned audience was *utterly silent*.
    /// - A fish can be *very slippery*.
    AdjectivePhr,
    /// An adjective prepositional phrase functions as an **Adjective**.
    /// It is part of a noun phrase.
    /// - The boy *in the striped pajamas* was sleepy.
    /// - *With bald heads*, men can sense the rain.
    AdjectivePrepPhr,
    /// An adverb phrase functions an an **Adverb**.
    /// It describes *how*, *when*, *where*, *why*, or *to what extent*.
    /// - She retreated *very quickly* after detecting the smell.
    /// - The boy walks *extremely slowly* in the rain.
    AdverbPhr,
    /// An adverb prepositional phrase functions as an **Adverb**.
    /// - We stayed *at home*.
    /// - It rained *until dawn*.
    /// - *Before dinner*, she walked.
    /// - The moth flew *to the lighthouse*.
    /// - He pushed "with all his might".
    AdverbPrepPhr,
    /// A gerund phrase starts with an "ing" verb and functions as a **Noun**.
    /// - *Being cool* is overrated.
    /// - Bob enjoys *hammering nails*.
    /// - *Creeping sliently* can help avoid detection.
    GerundPhr,
    /// An infinitive phrase starts with "to" followed by a verb.  It can
    /// function as a **Noun**, **Adjective**, or **Adverb**.
    /// - *To fly a plane* was his dream.
    /// - They plan *to marry*.
    /// - The man had an hour *to kill*.
    /// - She wants *to play a game*.
    /// - He likes *to eat bugs*.
    InfinitivePhr,
    /// An appositive phrase follows a **Noun**, explaining or renaming it.
    /// It is usually enclosed by commas.
    /// - The man, *an emotional wreck*, started hyperventilating.
    /// - Her uncle, *the mustached one*, resembled a man-faced stink bug.
    AppositivePhr,
    /// A participial phrase functions as an **Adjective**, and begins with
    /// a participle form of a verb.
    /// - The lake, *covered in snow*, was quiet.
    ParticipialPhr,
    /// An absolute phrase is set off by commas does not function as a
    /// specific word type, but instead modifies an entire sentence.
    /// - *The lake being cold,* they decided not to swim.
    AbsolutePhr,
}
