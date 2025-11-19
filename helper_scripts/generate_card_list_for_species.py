import json
from pathlib import Path, PosixPath


def get_list_of_files_in_directory(directory_path) -> list[PosixPath]:
    file_list = []
    for entry in directory_path.iterdir():
        if entry.is_file():
            file_list.append(entry)
    return file_list

def print_card_information(card, is_cameo: bool) -> None:
    card_id = card["id"]
    card_name = card["name"]
    if is_cameo:
        print(f"{card_id} - {card_name} (CAMEO)")
    else:
        print(f"{card_id} - {card_name}")

def get_pokemon_from_file(json_file: PosixPath, pokemon: str):
    with open(json_file) as file_in:
        json_data = json.load(file_in)

    for card in json_data:
        card_name = card["name"]
        card_cameos = card["cameos"]
        is_cameo = pokemon in card_cameos
        if pokemon in card_name or is_cameo:
            print_card_information(card=card, is_cameo=is_cameo)

def main():
    directory_path = Path("../data/english/cards/")
    pokemon = "Weedle"

    json_files = get_list_of_files_in_directory(directory_path=directory_path)
    for json_file in json_files:
        get_pokemon_from_file(json_file=json_file, pokemon=pokemon)


if __name__ == "__main__":
    main()
