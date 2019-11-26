table! {
    mapCelestialStatistics (celestialID) {
        celestialID -> Int4,
        temperature -> Nullable<Float8>,
        spectralClass -> Nullable<Varchar>,
        luminosity -> Nullable<Float8>,
        age -> Nullable<Float8>,
        life -> Nullable<Float8>,
        orbitRadius -> Nullable<Float8>,
        eccentricity -> Nullable<Float8>,
        massDust -> Nullable<Float8>,
        massGas -> Nullable<Float8>,
        fragmented -> Nullable<Bool>,
        density -> Nullable<Float8>,
        surfaceGravity -> Nullable<Float8>,
        escapeVelocity -> Nullable<Float8>,
        orbitPeriod -> Nullable<Float8>,
        rotationRate -> Nullable<Float8>,
        locked -> Nullable<Bool>,
        pressure -> Nullable<Float8>,
        radius -> Nullable<Float8>,
        mass -> Nullable<Int4>,
    }
}

table! {
    mapLandmarks (landmarkID) {
        landmarkID -> Int4,
        landmarkName -> Nullable<Varchar>,
        description -> Nullable<Text>,
        locationID -> Nullable<Int4>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        iconID -> Nullable<Int4>,
    }
}

table! {
    mapRegions (regionID) {
        regionID -> Int4,
        regionName -> Nullable<Varchar>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        xMin -> Nullable<Float8>,
        xMax -> Nullable<Float8>,
        yMin -> Nullable<Float8>,
        yMax -> Nullable<Float8>,
        zMin -> Nullable<Float8>,
        zMax -> Nullable<Float8>,
        factionID -> Nullable<Int4>,
        radius -> Nullable<Float8>,
    }
}

table! {
    mapSolarSystemJumps (fromSolarSystemID, toSolarSystemID) {
        fromRegionID -> Nullable<Int4>,
        fromConstellationID -> Nullable<Int4>,
        fromSolarSystemID -> Int4,
        toSolarSystemID -> Int4,
        toConstellationID -> Nullable<Int4>,
        toRegionID -> Nullable<Int4>,
    }
}

table! {
    mapSolarSystems (solarSystemID) {
        regionID -> Nullable<Int4>,
        // constellationID -> Nullable<Int4>,
        solarSystemID -> Int4,
        solarSystemName -> Nullable<Varchar>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        // xMin -> Nullable<Float8>,
        // xMax -> Nullable<Float8>,
        // yMin -> Nullable<Float8>,
        // yMax -> Nullable<Float8>,
        // zMin -> Nullable<Float8>,
        // zMax -> Nullable<Float8>,
        luminosity -> Nullable<Float8>,
        // border -> Nullable<Bool>,
        // fringe -> Nullable<Bool>,
        // corridor -> Nullable<Bool>,
        // hub -> Nullable<Bool>,
        // international -> Nullable<Bool>,
        // regional -> Nullable<Bool>,
        // constellation -> Nullable<Bool>,
        security -> Nullable<Float8>,
        // factionID -> Nullable<Int4>,
        // radius -> Nullable<Float8>,
        // sunTypeID -> Nullable<Int4>,
        // securityClass -> Nullable<Varchar>,
    }
}

table! {
    mapUniverse (universeID) {
        universeID -> Int4,
        universeName -> Nullable<Varchar>,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
        z -> Nullable<Float8>,
        xMin -> Nullable<Float8>,
        xMax -> Nullable<Float8>,
        yMin -> Nullable<Float8>,
        yMax -> Nullable<Float8>,
        zMin -> Nullable<Float8>,
        zMax -> Nullable<Float8>,
        radius -> Nullable<Float8>,
    }
}

allow_tables_to_appear_in_same_query!(
    mapCelestialStatistics,
    mapLandmarks,
    mapRegions,
    mapSolarSystemJumps,
    mapSolarSystems,
    mapUniverse,
);

joinable!(
    mapSolarSystems -> mapRegions(regionID)
);