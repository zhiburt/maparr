# `maparr` (help with array based maps) 📙

A rust macro to build a static `Map` based on const array.

## Get started

The idea is that you define your map first, and then you can use it wheather nessary.

```rust
use maparr::maparr;

maparr!(
    Continents;
    ASIA,
    AFRICA,
    AMERICA_NORTH,
    AMERICA_SOUTH,
    ANTARCTICA,
    EUROPE,
    AUSTRALIA,
);

const CONTINENT_SQUARE_MILES: Continents<usize> = maparr!(
    Continents;
    ASIA                = 17_212_000,
    AFRICA              = 11_608_000,
    AMERICA_NORTH       = 9_365_000,
    AMERICA_SOUTH       = 6_880_000,
    ANTARCTICA          = 5_100_000,
    EUROPE              = 3_837_000,
    AUSTRALIA           = 2_968_000,
);

fn main() {
    for (sq_miles, continent) in CONTINENT_SQUARE_MILES.into_iter().zip(Continents::names()) {
        println!("{continent:15} = {sq_miles:10} (sq mi)");
    }
}
```

You shall expect to get the following output in `stdout`.

```text
ASIA            =   17212000 (sq mi)
AFRICA          =   11608000 (sq mi)
AMERICA_NORTH   =    9365000 (sq mi)
AMERICA_SOUTH   =    6880000 (sq mi)
ANTARCTICA      =    5100000 (sq mi)
EUROPE          =    3837000 (sq mi)
AUSTRALIA       =    2968000 (sq mi)
```

You can modify the built map (even in `const` context if allowed).

```rust
use maparr::maparr;

maparr!(
    Continents<usize>;
    ASIA,
    AFRICA,
    AMERICA_NORTH,
    AMERICA_SOUTH,
    ANTARCTICA,
    EUROPE,
    AUSTRALIA,
);

fn main() {
    let mut continents = maparr!(
        Continents;
        ASIA                = 17_212_000,
        AFRICA              = 11_608_000,
        AMERICA_NORTH       = 9_365_000,
        AMERICA_SOUTH       = 6_880_000,
        ANTARCTICA          = 5_100_000,
        EUROPE              = 3_837_000,
        AUSTRALIA           = 2_968_000,
    );

    continents.set(Continents::ASIA, 17_212_001);
    assert_eq!(continents[Continents::ASIA], 17_212_001);

    continents = continents.map(|value| value * 2);
    assert_eq!(continents[Continents::ASIA], 17_212_001 * 2);
}
```