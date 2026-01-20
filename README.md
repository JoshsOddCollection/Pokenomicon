# Pokénomicon

Welcome to the Pokénomicon!

A play on words between Pokémon and the Necronomicon, this tool aims to contain a wealth of knowledge for all Pokémon collectors. Similar to the Necronomicon, the forbidden contents of this tool will threaten your sanity - once you reveal its contents, you'll learn that your collection is never complete!

---

# Roadmap
There currently aren't any good resources out there for generating complete lists of TCG cards, and this project aims to fix that.

Our initial goals are to:
- Support generating checklists for species, including cameos.
- Support generating checklists for sets.
- Support all card variants.
- Support both English and Japanese.
- Make our list generation as flexible as possible. If you want a list of every holographic card featuring Squirtle before the year 2009, you can generate it!

---

# Validation
All data is validated with a strict ruleset as defined in the Rust schema validation code. The schema is continually improving and evolving as we discover edge cases. If you are trying to submit something that isn't passing validation, please tag someone on the merge request so that the schema can be updated. The schema is designed primarily to prevent simple human errors and omissions, in addition to helping with data consistency.

---

# Documentation
Documentation can be found in the [documentation folder](./documentation/). All documentation is formatted using Markdown.

---

# Contributing

Want to contribute? Please check out the documentation first, then feel free to [create a pull request](https://github.com/JoshsOddCollection/Pokenomicon/pulls)!

---

# Special Thanks
Special thanks to all of the following resources - they have been invaluable references for collating the information in the Pokénomicon TCG database in one way or another.
- [EliteFourum](https://www.elitefourum.com)
- [pkmncards](https://pkmncards.com/)
- [pokemon-tcg-data](https://github.com/PokemonTCG/pokemon-tcg-data) repo
- [TCGCollector](https://github.com/PokemonTCG/pokemon-tcg-data)
