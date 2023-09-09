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
    let continent_sq_mil_total: usize = CONTINENT_SQUARE_MILES.sum();
    let continent_sq_mil_persent =
        CONTINENT_SQUARE_MILES.map(|mil| (mil * 100) as f32 / continent_sq_mil_total as f32);

    for ((continent, sq_miles), sq_persent) in Continents::names()
        .into_iter()
        .zip(CONTINENT_SQUARE_MILES)
        .zip(continent_sq_mil_persent)
    {
        println!("{continent:15} = {sq_miles:10} (sq mi) {sq_persent:6.2} (%)");
    }
}
