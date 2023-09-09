use maparr::maparr;

maparr!(
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

pub const EARTH: EarthMap<usize> = maparr!(
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

pub const LUA: EarthMap<usize> = maparr!(
    EarthMap;
    Asia                = *EARTH.get(EarthMap::Asia) * 2,
    Africa              = *EARTH.get(EarthMap::Africa) * 2,
    America_North       = *EARTH.get(EarthMap::America_North) * 2,
    America_South       = *EARTH.get(EarthMap::America_South) * 2,
    Antarctica          = *EARTH.get(EarthMap::Antarctica) * 2,
    Europe              = *EARTH.get(EarthMap::Europe) * 2,
    Australia           = *EARTH.get(EarthMap::Australia) * 2,
);
