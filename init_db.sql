CREATE TABLE IF NOT EXISTS location
(
    City varchar(255) NOT NULL,
    State varchar(255) NOT NULL,
    Population int,
    Latitude float(4) NOT NULL,
    Longitude float(4) NOT NULL,
    PRIMARY KEY(City)
);