# Warfare

This is the codebase for a multiplayer, 2d, turn-based strategy wargame tentatively titled Warfare (imaginative, I know). It's written in [rust](https://www.rust-lang.org/) (because I like rust) and uses the [bevy game engine](https://bevyengine.org/). Warfare is set in the modern day (using weapons and units currently available in 202X) and is intended to be a highly granular, detailed game.

## Proposed Features

* Procedurally generated maps up to 1000x1000 tiles (62500 sq/mi)
* Seasons and day/night cycles that affect the environment
* Simulated plant growth, water cycles etc.
* Units that are effected by terrain, morale, accident rates, logistic concerns, etc.
* Soldiers in units are tracked and can gain experience individually
* Most unit attributes (veteran status, accuracy etc.) are aggregations of included soldier attributes
* Units can be re-structured and individual soldiers can be re-assigned

There may be more.

## Implemented Features

* Basic hex map generation up to 1000x1000
* Elevation generation from various noise sources
* Water distribution based on elevation

## Goals

The goal here is total battlefield simulation. Everything from the rockiness of the soil to cloud cover should have *some* impact, no matter how small, and everything should interact with everything else. Below, I've broken the gameplay down into a few large categories to make it easier to discuss.

### Terrain and Environment


Terrain is made up of a grid of hexagonal tiles. Each tile is considered to be roughly 0.25 by 0.21 miles (0.0406 sq/mi). They have the following attributes:

* Location
* Elevation
* Water content
* Soil type
* Biome
* Rockiness
* Temperature
* Fertility

Most attributes (with the exception of elevation and location) are determined through calculations based on other attributes. For example, fertility is calculated based on soil type, proximity to water and temperature. 

Each turn is considered a single 24 hour period and will include a day and night stage. Environmental effects may apply to units differently depending on the stage (accident rates increased, visibility reduced etc.)

Every 365 turns is considered a year and each year is broken up into four seasons. The season will affect temperature, available food, accident rates and types of weather.

### Units and Soldiers

Units contain some number of soldiers and a large number of attributes that modify the unit's state each turn:

* Readiness
* Injuries
* Equipment
* Food
* Vehicles
* Ammunition
* Morale

Some of these attributes rely on others while some are entirely based on the affects of the environment. Readiness, for example, is the number of soldiers that are healthy and equipped for combat- increased accident and illness rates because of difficult terrain or injuries from combat would reduce this attribute.

Units can be merged or divided, changing their capabilities.

They have actions that they can take based on these capabilities, and will succeed or fail based on the experience and number of the individual soldiers. For example, a unit made up entirely of support personnel might have difficulty performing an assault, while a unit of infantry may have similar difficulty recovering damaged vehicles.

### Logistics and Gear

Soldiers require resources in order to maintain their readiness:

* Food,
* Water,
* Ammunition,
* Fuel
* Medical

These resources are supplied by logistics and support personnel. Units made up of these soldiers are capable of transporting or supplying resources to nearby combat units.


## Non-Goals

There are some things that Warfare doesn't care about:

* **The real-world national origin of each side of the battlefield**. No units in this game are explicitly US, Iraqi, Russian, or any other real-world nationality. This is a game about exploring the practical considerations involved in waging war. It's not about politics or historical grudge matches. 
* **Civilization building or peaceful coexistence**. Units can build fortifications and find some local resources, but they cannot build cities, plant crops or research technology.
* **Poltics**. This game does not include negotiation, cease-fires, diplomacy or any other method of "waging war by other means." It *may* include some opsec/sigint-type mechanics.

There are a lot of games out there for doing the rest of this stuff, but that isn't what I want to make here.

## Contributing

If you want to contribute, make a pull request.

## Building

`sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev`