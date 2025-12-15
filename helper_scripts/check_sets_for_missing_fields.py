import json
from pathlib import Path

def check_set_for_missing_fields(json_data):
    card_fields = [
        "card_type", "name", "number", "artist", "rarity", "variants", "cameos",
        "animal_cameos", "pokeball_cameos", "is_first_art_appearance_for_language",
        "id_of_first_art_appearance_for_language"
    ]
    for card in json_data:
        card_id = card["id"]
        for field in card_fields:
            if field not in card:
                print(f"{card_id} - The field `{field}` is missing.")

def main():
    json_folder = Path("../data/english/cards")
    for json_file_location in json_folder.iterdir():
        with open(json_file_location) as file_in:
            json_data = json.load(file_in)

        check_set_for_missing_fields(json_data=json_data)

if __name__ == "__main__":
    main()
