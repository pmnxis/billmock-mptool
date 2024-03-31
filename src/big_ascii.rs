pub struct BigAsciiSymbol([&'static str; 11]);

const UPPER_CASE_SYMBOL: [BigAsciiSymbol; 26] = [
    /* A */
    BigAsciiSymbol {
        0: [
            r#"       d8888  "#,
            r#"      d88888  "#,
            r#"     d88P888  "#,
            r#"    d88P 888  "#,
            r#"   d88P  888  "#,
            r#"  d88P   888  "#,
            r#" d8888888888  "#,
            r#"d88P     888  "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* B */
    BigAsciiSymbol {
        0: [
            r#"888888b.      "#,
            r#"888  "88b     "#,
            r#"888  .88P     "#,
            r#"8888888K.     "#,
            r#"888  "Y88b    "#,
            r#"888    888    "#,
            r#"888   d88P    "#,
            r#"8888888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* C */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"888    888    "#,
            r#"888           "#,
            r#"888           "#,
            r#"888    888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* D */
    BigAsciiSymbol {
        0: [
            r#"8888888b.     "#,
            r#"888  "Y88b    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"888  .d88P    "#,
            r#"8888888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* E */
    BigAsciiSymbol {
        0: [
            r#"8888888888    "#,
            r#"888           "#,
            r#"888           "#,
            r#"8888888       "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"8888888888    "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* F */
    BigAsciiSymbol {
        0: [
            r#"8888888888    "#,
            r#"888           "#,
            r#"888           "#,
            r#"8888888       "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* G */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"888    888    "#,
            r#"888           "#,
            r#"888  88888    "#,
            r#"888    888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P88    "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* H */
    BigAsciiSymbol {
        0: [
            r#"888    888    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"8888888888    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* I */
    BigAsciiSymbol {
        0: [
            r#"8888888       "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"8888888       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* J */
    BigAsciiSymbol {
        0: [
            r#"  888888      "#,
            r#"    "88b      "#,
            r#"     888      "#,
            r#"     888      "#,
            r#"     888      "#,
            r#"     888      "#,
            r#"     88P      "#,
            r#"     888      "#,
            r#"   .d88P      "#,
            r#" .d88P"       "#,
            r#"888P"         "#,
        ],
    },
    /* K */
    BigAsciiSymbol {
        0: [
            r#"888    d8P    "#,
            r#"888   d8P     "#,
            r#"888  d8P      "#,
            r#"888d88K       "#,
            r#"8888888b      "#,
            r#"888  Y88b     "#,
            r#"888   Y88b    "#,
            r#"888    Y88b   "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* L */
    BigAsciiSymbol {
        0: [
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"88888888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* M */
    BigAsciiSymbol {
        0: [
            r#"888b     d888 "#,
            r#"8888b   d8888 "#,
            r#"88888b.d88888 "#,
            r#"888Y88888P888 "#,
            r#"888 Y888P 888 "#,
            r#"888  Y8P  888 "#,
            r#"888   "   888 "#,
            r#"888       888 "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* N */
    BigAsciiSymbol {
        0: [
            r#"888b    888   "#,
            r#"8888b   888   "#,
            r#"88888b  888   "#,
            r#"888Y88b 888   "#,
            r#"888 Y88b888   "#,
            r#"888  Y88888   "#,
            r#"888   Y8888   "#,
            r#"888    Y888   "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* O */
    BigAsciiSymbol {
        0: [
            r#" .d88888b.    "#,
            r#"d88P" "Y88b   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"Y88b. .d88P   "#,
            r#" "Y88888P"    "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* P */
    BigAsciiSymbol {
        0: [
            r#"8888888b.     "#,
            r#"888   Y88b    "#,
            r#"888    888    "#,
            r#"888   d88P    "#,
            r#"8888888P"     "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* Q */
    BigAsciiSymbol {
        0: [
            r#" .d88888b.    "#,
            r#"d88P" "Y88b   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888 Y8b 888   "#,
            r#"Y88b.Y8b88P   "#,
            r#" "Y888888"    "#,
            r#"       Y8b    "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* R */
    BigAsciiSymbol {
        0: [
            r#"8888888b.     "#,
            r#"888   Y88b    "#,
            r#"888    888    "#,
            r#"888   d88P    "#,
            r#"8888888P"     "#,
            r#"888 T88b      "#,
            r#"888  T88b     "#,
            r#"888   T88b    "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* S */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"Y88b.         "#,
            r#" "Y888b.      "#,
            r#"    "Y88b.    "#,
            r#"      "888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* T */
    BigAsciiSymbol {
        0: [
            r#"88888888888   "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* U */
    BigAsciiSymbol {
        0: [
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"Y88b. .d88P   "#,
            r#" "Y88888P"    "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* V */
    BigAsciiSymbol {
        0: [
            r#"888     888   "#,
            r#"888     888   "#,
            r#"888     888   "#,
            r#"Y88b   d88P   "#,
            r#" Y88b d88P    "#,
            r#"  Y88o88P     "#,
            r#"   Y888P      "#,
            r#"    Y8P       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* W */
    BigAsciiSymbol {
        0: [
            r#"888       888 "#,
            r#"888   o   888 "#,
            r#"888  d8b  888 "#,
            r#"888 d888b 888 "#,
            r#"888d88888b888 "#,
            r#"88888P Y88888 "#,
            r#"8888P   Y8888 "#,
            r#"888P     Y888 "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* X */
    BigAsciiSymbol {
        0: [
            r#"Y88b   d88P   "#,
            r#" Y88b d88P    "#,
            r#"  Y88o88P     "#,
            r#"   Y888P      "#,
            r#"   d888b      "#,
            r#"  d88888b     "#,
            r#" d88P Y88b    "#,
            r#"d88P   Y88b   "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* Y */
    BigAsciiSymbol {
        0: [
            r#"Y88b   d88P   "#,
            r#" Y88b d88P    "#,
            r#"  Y88o88P     "#,
            r#"   Y888P      "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"    888       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* Z */
    BigAsciiSymbol {
        0: [
            r#"8888888888P   "#,
            r#"      d88P    "#,
            r#"     d88P     "#,
            r#"    d88P      "#,
            r#"   d88P       "#,
            r#"  d88P        "#,
            r#" d88P         "#,
            r#"d8888888888   "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
];

const LOWER_CASE_SYMBOL: [BigAsciiSymbol; 26] = [
    /* A */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#" 8888b.       "#,
            r#"    "88b      "#,
            r#".d888888      "#,
            r#"888  888      "#,
            r#""Y888888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* B */
    BigAsciiSymbol {
        0: [
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"88888b.       "#,
            r#"888 "88b      "#,
            r#"888  888      "#,
            r#"888 d88P      "#,
            r#"88888P"       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* C */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#" .d8888b      "#,
            r#"d88P"         "#,
            r#"888           "#,
            r#"Y88b.         "#,
            r#" "Y8888P      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* D */
    BigAsciiSymbol {
        0: [
            r#"     888      "#,
            r#"     888      "#,
            r#"     888      "#,
            r#" .d88888      "#,
            r#"d88" 888      "#,
            r#"888  888      "#,
            r#"Y88b 888      "#,
            r#" "Y88888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* E */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#" .d88b.       "#,
            r#"d8P  Y8b      "#,
            r#"88888888      "#,
            r#"Y8b.          "#,
            r#" "Y8888       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* F */
    BigAsciiSymbol {
        0: [
            r#" .d888        "#,
            r#"d88P"         "#,
            r#"888           "#,
            r#"888888        "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* G */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#" .d88b.       "#,
            r#"d88P"88b      "#,
            r#"888  888      "#,
            r#"Y88b 888      "#,
            r#" "Y88888      "#,
            r#"     888      "#,
            r#"Y8b d88P      "#,
            r#" "Y88P"       "#,
        ],
    },
    /* H */
    BigAsciiSymbol {
        0: [
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"88888b.       "#,
            r#"888 "88b      "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* I */
    BigAsciiSymbol {
        0: [
            r#"8888888       "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"8888888       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* J */
    BigAsciiSymbol {
        0: [
            r#"   d8b        "#,
            r#"   Y8P        "#,
            r#"              "#,
            r#"  8888        "#,
            r#"  "888        "#,
            r#"   888        "#,
            r#"   888        "#,
            r#"   888        "#,
            r#"   888        "#,
            r#"  d88P        "#,
            r#"888P"         "#,
        ],
    },
    /* K */
    BigAsciiSymbol {
        0: [
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888  888      "#,
            r#"888 .88P      "#,
            r#"888888K       "#,
            r#"888 "88b      "#,
            r#"888  888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* L */
    BigAsciiSymbol {
        0: [
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* M */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"88888b.d88b.  "#,
            r#"888 "888 "88b "#,
            r#"888  888  888 "#,
            r#"888  888  888 "#,
            r#"888  888  888 "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* N */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"88888b.       "#,
            r#"888 "88b      "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* O */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#" .d88b.       "#,
            r#"d88""88b      "#,
            r#"888  888      "#,
            r#"Y88..88P      "#,
            r#" "Y88P"       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* P */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"88888b.       "#,
            r#"888 "88b      "#,
            r#"888  888      "#,
            r#"888 d88P      "#,
            r#"88888P"       "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
        ],
    },
    /* Q */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#" .d88888      "#,
            r#"d88" 888      "#,
            r#"888  888      "#,
            r#"Y88b 888      "#,
            r#" "Y88888      "#,
            r#"     888      "#,
            r#"     888      "#,
            r#"     888      "#,
        ],
    },
    /* R */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"888d888       "#,
            r#"888P"         "#,
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* S */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#".d8888b       "#,
            r#"88K           "#,
            r#""Y8888b.      "#,
            r#"     X88      "#,
            r#" 88888P'      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* T */
    BigAsciiSymbol {
        0: [
            r#"888           "#,
            r#"888           "#,
            r#"888           "#,
            r#"888888        "#,
            r#"888           "#,
            r#"888           "#,
            r#"Y88b.         "#,
            r#" "Y888        "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* U */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"Y88b 888      "#,
            r#" "Y88888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* V */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"Y88  88P      "#,
            r#" Y8bd8P       "#,
            r#"  Y88P        "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* W */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"888  888  888 "#,
            r#"888  888  888 "#,
            r#"888  888  888 "#,
            r#"Y88b 888 d88P "#,
            r#" "Y8888888P"  "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* X */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"888  888      "#,
            r#"`Y8bd8P'      "#,
            r#"  X88K        "#,
            r#".d8""8b.      "#,
            r#"888  888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* Y */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"888  888      "#,
            r#"Y88b 888      "#,
            r#" "Y88888      "#,
            r#"     888      "#,
            r#"Y8b d88P      "#,
            r#" "Y88P"       "#,
        ],
    },
    /* Z */
    BigAsciiSymbol {
        0: [
            r#"              "#,
            r#"              "#,
            r#"              "#,
            r#"88888888      "#,
            r#"   d88P       "#,
            r#"  d88P        "#,
            r#" d88P         "#,
            r#"88888888      "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
];

const NUMBER_SYMBOL: [BigAsciiSymbol; 10] = [
    /* 0 */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"888    888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 1 */
    BigAsciiSymbol {
        0: [
            r#" d888         "#,
            r#"d8888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"  888         "#,
            r#"8888888       "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 2 */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"       888    "#,
            r#"     .d88P    "#,
            r#" .od888P"     "#,
            r#"d88P"         "#,
            r#"888"          "#,
            r#"888888888     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 3 */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"     .d88P    "#,
            r#"    8888"     "#,
            r#"     "Y8b.    "#,
            r#"888    888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 4 */
    BigAsciiSymbol {
        0: [
            r#"    d8888     "#,
            r#"   d8P888     "#,
            r#"  d8P 888     "#,
            r#" d8P  888     "#,
            r#"d88   888     "#,
            r#"8888888888    "#,
            r#"      888     "#,
            r#"      888     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 5 */
    BigAsciiSymbol {
        0: [
            r#"888888888     "#,
            r#"888           "#,
            r#"888           "#,
            r#"8888888b.     "#,
            r#"     "Y88b    "#,
            r#"       888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 6 */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"888           "#,
            r#"888d888b.     "#,
            r#"888P "Y88b    "#,
            r#"888    888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 7 */
    BigAsciiSymbol {
        0: [
            r#"8888888888    "#,
            r#"      d88P    "#,
            r#"     d88P     "#,
            r#"    d88P      "#,
            r#" 88888888     "#,
            r#"  d88P        "#,
            r#" d88P         "#,
            r#"d88P          "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 8 */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"Y88b. d88P    "#,
            r#" "Y88888"     "#,
            r#".d8P""Y8b.    "#,
            r#"888    888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
    /* 9 */
    BigAsciiSymbol {
        0: [
            r#" .d8888b.     "#,
            r#"d88P  Y88b    "#,
            r#"888    888    "#,
            r#"Y88b. d888    "#,
            r#" "Y888P888    "#,
            r#"       888    "#,
            r#"Y88b  d88P    "#,
            r#" "Y8888P"     "#,
            r#"              "#,
            r#"              "#,
            r#"              "#,
        ],
    },
];

const SYMBOL_QUESTION: BigAsciiSymbol = BigAsciiSymbol {
    0: [
        r#" .d8888b.     "#,
        r#"d88P  Y88b    "#,
        r#"     .d88P    "#,
        r#"   .d88P"     "#,
        r#"   888"       "#,
        r#"   888        "#,
        r#"              "#,
        r#"   888        "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
    ],
};

const SYMBOL_EXCLMATION: BigAsciiSymbol = BigAsciiSymbol {
    0: [
        r#"888           "#,
        r#"888           "#,
        r#"888           "#,
        r#"888           "#,
        r#"888           "#,
        r#"Y8P           "#,
        r#" "            "#,
        r#"888           "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
    ],
};

const SYMBOL_SLASH: BigAsciiSymbol = BigAsciiSymbol {
    0: [
        r#"        d88P  "#,
        r#"       d88P   "#,
        r#"      d88P    "#,
        r#"     d88P     "#,
        r#"    d88P      "#,
        r#"   d88P       "#,
        r#"  d88P        "#,
        r#" d88P         "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
    ],
};

const SYMBOL_MINUS: BigAsciiSymbol = BigAsciiSymbol {
    0: [
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#" .d888888888P "#,
        r#"d8888888888"  "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
    ],
};

const SYMBOL_BLANK: BigAsciiSymbol = BigAsciiSymbol {
    0: [
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
        r#"              "#,
    ],
};

impl Into<&'static BigAsciiSymbol> for char {
    fn into(self) -> &'static BigAsciiSymbol {
        if self.is_ascii_lowercase() {
            let idx = (self as usize) - (b'a' as usize);

            &LOWER_CASE_SYMBOL[idx]
        } else if self.is_ascii_uppercase() {
            let idx = (self as usize) - (b'A' as usize);

            &UPPER_CASE_SYMBOL[idx]
        } else if self.is_numeric() {
            let idx = (self as usize) - (b'0' as usize);

            &NUMBER_SYMBOL[idx]
        } else if self.is_ascii() {
            match self as u8 {
                b' ' => &SYMBOL_BLANK,
                b'?' => &SYMBOL_QUESTION,
                b'/' => &SYMBOL_SLASH,
                b'!' => &SYMBOL_EXCLMATION,
                b'-' => &SYMBOL_MINUS,
                _ => &SYMBOL_BLANK,
            }
        } else {
            &SYMBOL_BLANK
        }
    }
}

impl std::fmt::Display for BigAsciiSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0 {
            writeln!(f, "{}", line)?;
        }

        std::fmt::Result::Ok(())
    }
}

pub struct BigAsciiString(Vec<&'static BigAsciiSymbol>);

impl Into<BigAsciiString> for &String {
    fn into(self) -> BigAsciiString {
        let mut ret = Vec::new();

        for c in self.chars() {
            ret.push(c.into());
        }

        BigAsciiString { 0: ret }
    }
}

impl Into<BigAsciiString> for String {
    fn into(self) -> BigAsciiString {
        let mut ret = Vec::new();

        for c in self.chars() {
            ret.push(c.into());
        }

        BigAsciiString { 0: ret }
    }
}

impl std::fmt::Display for BigAsciiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..11 {
            for s in &self.0 {
                write!(f, "{}", s.0[i])?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}
