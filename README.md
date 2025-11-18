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
- Support multiple languages. We will start with English and Japanese, then expand to other languages.
- Make our list generation as flexible as possible. If you want a list of every holographic card featuring Squirtle before the year 2009, you can generate it!

---

# Fields
- `cameos` - A list of all Pokémon featured in a card. Excludes Pokémon whose names are in the name of the card.
- `animal_cameos` - A list of all animals featured in a card.
- `pokeball_cameos` - A list of all types of Poké Balls featured in the card. Poké Balls featured as emblems (such as on Potions) do **_not_** count.
- `first_appearance_for_language` - A boolean stating whether or not this is the first appearance of the artwork for the given language.

---

# Contributing

Want to contribute? Feel free to [create a pull request](https://github.com/JoshsOddCollection/Pokenomicon/pulls)!
