# static_map (help with array based map) 📙

A rust macro to build a static `Map` based on const array.


```rust
use static_map::static_map;

static_map!(
    pub Continents;
    ASIA,
    AFRICA,
    AMERICA_NORTH,
    AMERICA_SOUTH,
    ANTARCTICA,
    EUROPE,
    AUSTRALIA,
);

pub const CONTINENT_SQUARE_MILES: Continents<usize> = static_map!(
    Continents;
    ASIA                = 17_212_000,
    AFRICA              = 11_608_000,
    AMERICA_NORTH       = 9_365_000,
    AMERICA_SOUTH       = 6_880_000,
    ANTARCTICA          = 5_100_000,
    EUROPE              = 3_837_000,
    AUSTRALIA           = 2_968_000,
);

pub const CONTINENT_SQUARE_MILES_TOTAL: usize = *CONTINENT_SQUARE_MILES.get(Continents::ASIA)
    + *CONTINENT_SQUARE_MILES.get(Continents::AFRICA)
    + *CONTINENT_SQUARE_MILES.get(Continents::AMERICA_NORTH)
    + *CONTINENT_SQUARE_MILES.get(Continents::AMERICA_SOUTH)
    + *CONTINENT_SQUARE_MILES.get(Continents::ANTARCTICA)
    + *CONTINENT_SQUARE_MILES.get(Continents::EUROPE)
    + *CONTINENT_SQUARE_MILES.get(Continents::AUSTRALIA);

pub const CONTINENT_SQUARE_MILES_PERSENT: Continents<f32> = static_map!(
    Continents;
    ASIA                = (*CONTINENT_SQUARE_MILES.get(Continents::ASIA) * 100) as f32 / CONTINENT_SQUARE_MILES_TOTAL as f32,
    AFRICA              = (*CONTINENT_SQUARE_MILES.get(Continents::AFRICA) * 100) as f32 / CONTINENT_SQUARE_MILES_TOTAL as f32,
    AMERICA_NORTH       = (*CONTINENT_SQUARE_MILES.get(Continents::AMERICA_NORTH) * 100) as f32 / CONTINENT_SQUARE_MILES_TOTAL as f32,
    AMERICA_SOUTH       = (*CONTINENT_SQUARE_MILES.get(Continents::AMERICA_SOUTH) * 100) as f32 / CONTINENT_SQUARE_MILES_TOTAL as f32,
    ANTARCTICA          = (*CONTINENT_SQUARE_MILES.get(Continents::ANTARCTICA) * 100) as f32 / CONTINENT_SQUARE_MILES_TOTAL as f32,
    EUROPE              = (*CONTINENT_SQUARE_MILES.get(Continents::EUROPE) * 100) as f32 / CONTINENT_SQUARE_MILES_TOTAL as f32,
    AUSTRALIA           = (*CONTINENT_SQUARE_MILES.get(Continents::AUSTRALIA) * 100) as f32 / CONTINENT_SQUARE_MILES_TOTAL as f32,
);

fn main() {
    for ((continent, sq_miles), sq_persent) in Continents::names()
        .into_iter()
        .zip(CONTINENT_SQUARE_MILES)
        .zip(CONTINENT_SQUARE_MILES_PERSENT)
    {
        println!("{continent:15} = {sq_miles:10} (sq mi) {sq_persent:6.2} (%)");
    }
}
```