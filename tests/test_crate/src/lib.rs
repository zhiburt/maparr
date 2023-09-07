static_map::static_map!(
    #[derive(Debug, Clone)]
    pub EarthMap;
    Asia,
    Africa,
    America_North,
    America_South,
    Antarctica,
    Europe,
    Australia,
);

pub const EARTH: EarthMap<usize> = static_map::static_map!(
    EarthMap;
    Asia                = 1000,
    Africa              = 2000,
    America_North       = 3000,
    America_South       = 4000,
    Antarctica          = 5000,
    Europe              = 6000,
    Australia           = 7000,
);

pub const EARTH_AFRICA: usize = *EARTH.get(EarthMap::Africa);
