pub const MELODY: [(&str, u64); 134] = [
    ("B♭4", 500), // "Look"
    ("G4", 500),  // "at"
    // bar
    ("D4", 3000), // "me"
    ("B♭3", 500), // "I'm"
    ("C4", 500),  // "as"
    // bar
    ("C#4", 500), // "help-"
    ("C5", 500),  // "less-"
    ("C5", 500),  // "as"
    ("A#4", 500), // "a"
    ("C5", 500),  // "kitt-"
    ("A#4", 500), // "-en"
    ("G4", 500),  // up
    ("E♭4", 500), // a
    // bar
    ("C4", 2500), // tree
    ("", 250),    // (quarter-note rest)
    ("G#3", 250), // and
    ("G#3", 250), // I
    ("G#3", 250), // feel
    ("C4", 250),  // like
    ("E♭4", 250), // I'm
    // bar
    ("B♭4", 500),  // cling-
    ("B♭4", 500),  // ing
    ("B♭4", 500),  // to
    ("G#4", 500),  // a
    ("B♭4", 1000), // cloud
    ("G#4", 1000), // I
    // bar
    ("G4", 1000),  // can't
    ("G#4", 500),  // un-
    ("B♭4", 500),  // -der-
    ("E♭4", 1000), // -stand
    ("F4", 500),   // I
    ("G4", 500),   // get
    // bar
    ("G#4", 500), // mist-
    ("C4", 1000), // -y
    ("C4", 500),  // just
    ("D4", 1000), // hold-
    ("E♭4", 500), // -ing
    ("F4", 500),  // your
    // bar
    ("G4", 1000), // hand
    ("G4", 1000), // hand
    ("G4", 1000), // hand
    ("G4", 1000), // hand
    // bar
    ("F4", 1000), // hand
    ("F4", 1000), // hand
    ("F4", 1000), // hand
    ("B♭4", 500), // "Walk"
    ("G4", 500),  // "my"
    // bar
    ("D4", 3000), // "way"
    ("B♭3", 500), // "and"
    ("C4", 500),  // "a"
    // bar
    ("C#4", 500), // "thou-"
    ("C5", 500),  // "-sand"
    ("C5", 500),  // "vi-"
    ("A#4", 500), // "-o-"
    ("C5", 500),  // "lins"
    ("A#4", 500), // "be"
    ("G4", 500),  // gin
    ("E♭4", 500), // to
    // bar
    ("C4", 2500), // play
    ("", 250),    // rest
    ("G#3", 250), // or
    ("G#3", 250), // it
    ("G#3", 250), // might
    ("C4", 250),  // be
    ("E♭4", 250), // the
    // bar
    ("B♭4", 500),  // sound
    ("B♭4", 500),  // of
    ("B♭4", 500),  // your
    ("G#4", 500),  // hell-
    ("B♭4", 1000), // o
    ("G#4", 1000), // that
    // bar
    ("G4", 1000),  // mu-
    ("G#4", 500),  // -sic
    ("B♭4", 500),  // I
    ("E♭4", 1000), // hear
    ("F4", 500),   // I
    ("G4", 500),   // get
    // bar
    ("G#4", 500), // mis
    ("C4", 1500), // -ty
    ("E♭4", 500), // when-
    ("D4", 500),  // -ev-
    ("E♭4", 500), // -er
    ("F4", 500),  // you're
    // bar
    ("E♭4", 1000), // near
    ("E♭4", 1000), // near
    ("E♭4", 1000), // near
    ("E♭4", 1000), // near
    // bar
    ("", 1500),   // *rest*
    ("E♭4", 500), // Don't
    ("F4", 500),  // you
    ("G4", 500),  // see
    ("B♭4", 500), // that
    ("C5", 500),  // you're
    // bar
    ("C#5", 500),  // lead-
    ("C#5", 500),  // -ing
    ("C#5", 500),  // me
    ("C#5", 2500), // on?
    // bar
    ("C#5", 500),  // (carry over note)
    ("C#5", 250),  // and
    ("E♭5", 250),  // it's
    ("E5", 1000),  // just
    ("E♭5", 1000), // what
    ("C#5", 1000), // I
    // bar
    ("C5", 1000),  // want
    ("C5", 500),   // you
    ("E♭4", 500),  // to
    ("C5", 2000),  // do?
    ("", 2000),    // --pause--
    ("E♭4", 500),  // Don't
    ("F4", 500),   // you
    ("G4", 500),   // no-
    ("B♭4", 500),  // -tice
    ("C5", 500),   // how
    ("D5", 500),   // hope-
    ("D5", 500),   // -less-
    ("D5", 500),   // -ly
    ("D5", 500),   // I'm
    ("D5", 2000),  // lost
    ("D5", 500),   // That's
    ("D5", 500),   // Why
    ("C#5", 500),  // I'm
    ("D5", 500),   // foll-
    ("F5", 500),   // -oll-
    ("D5", 500),   // -ow-
    ("C5", 500),   // ing
    ("B♭4", 1000), // you
    ("B♭4", 1000), // --riff--
    ("G#4", 1000), // --riff--
    ("G4", 1000),  // --riff--
    ("G#4", 1000), // --riff--
    ("G4", 1000),  // --riff--
    ("F4", 1000),  // --riff--
    ("B♭4", 1000), // --riff--
    ("", 3000),    // --pause--
];

pub const BASSLINE: [(&str, u64); 69] = [
    ("", 1000), // 2-beat lead for melody...
    // bar
    // E♭maj7
    ("E♭2", 1000), // me
    ("G2", 1000),  //
    ("G#2", 1000), //
    ("B♭2", 1000), //
    // bar
    // B♭-7 -> E♭7
    ("B♭1", 1000), // helpless
    ("C#2", 1000), // as a
    ("E♭2", 1000), // kitten
    ("G2", 1000),  // up a
    // bar
    // A♭maj7
    ("G#1", 1000), // tree...
    ("C2", 1000),
    ("E♭2", 1000),
    ("G#2", 1000),
    // Fmin7
    ("G#1", 1000), // clinging
    ("B2", 1000),  // to a
    ("E♭2", 1000), //  cloud
    ("G#2", 1000), // I
    // E♭7 0> Cmin7
    ("E♭2", 1000), // can't
    ("G2", 1000),  // under
    ("C2", 1000),  // stand
    ("E♭2", 1000), // I get
    // Fmin7 -> Bb7
    ("F1", 1000),  // misty
    ("G#2", 1000), // just
    ("B♭2", 1000), // holding
    ("C2", 1000),  // your
    //  Gmin7 -> C7
    ("G2", 1000), // hand...
    ("B♭2", 1000),
    ("C1", 1000),
    ("E1", 1000),
    // Fmin7 -> Bb7
    ("F2", 1000),
    ("A2", 1000),
    ("B♭2", 1000),
    ("C2", 1000), // walk my
    //  E♭7
    ("E♭2", 1000), // way
    ("G2", 1000),  //
    ("G#2", 1000), //
    ("B♭2", 1000), //
    // E♭maj7
    ("E♭2", 1000),
    ("G2", 1000),
    ("G#2", 1000),
    ("B♭2", 1000), // E♭maj7
    // Cmin7
    ("C2", 1000),
    ("E♭2", 1000),
    ("G2", 1000),
    ("A2", 1000), // Cmin7
    // Fmin7
    ("F2", 1000),
    ("A2", 1000),
    ("C3", 1000),
    ("D3", 1000), // Fmin7
    // B♭7
    ("B♭2", 1000),
    ("D3", 1000),
    ("F3", 1000),
    ("G3", 1000), // B♭7
    // E♭maj7
    ("E♭2", 1000),
    ("G2", 1000),
    ("G#2", 1000),
    ("B♭2", 1000), // E♭maj7
    // Cmin7
    ("C2", 1000),
    ("E♭2", 1000),
    ("G2", 1000),
    ("A2", 1000), // Cmin7
    // Fmin7
    ("F2", 1000),
    ("A2", 1000),
    ("C3", 1000),
    ("D3", 1000), // Fmin7
    // Cmin7
    ("C2", 1000),
    ("E♭2", 1000),
    ("G2", 1000),
    ("A2", 1000), // Cmin7
];
