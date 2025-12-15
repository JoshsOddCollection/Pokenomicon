import json

def main():
    json_file_location = "./wotc.json"
    with open(json_file_location) as file_in:
        json_data = json.load(file_in)

    set_name = "wotc_promos"

    all_json = []

    for card in json_data:
        name = card["name"]
        supertype = card["supertype"]
        number = card["id"].split("-")[1]
        artist = card["artist"]
        rarity = card.get("rarity")
        card_id = f"{set_name}_{number}"

        variants = []

        variant_1 = {}
        variant_1["id"] = f"{set_name}_{number}_1"
        variant_1["variant"] = "Unlimited"
        variants.append(variant_1)

        # variant_2 = {}
        # variant_2["id"] = f"{set_name}_{number}_2"
        # variant_2["variant"] = "Unlimited"
        # variants.append(variant_2)

        card_json = {}
        card_json["id"] = card_id
        card_json["card_type"] = supertype
        card_json["name"] = name
        card_json["number"] = number
        card_json["artist"] = artist
        card_json["rarity"] = rarity
        card_json["variants"] = variants
        card_json["error_variants"] = None
        card_json["cameos"] = None
        card_json["animal_cameos"] = None
        card_json["pokeball_cameos"] = None
        card_json["fossil_cameos"] = None
        card_json["is_first_art_appearance_for_language"] = None
        card_json["id_of_first_art_appearance_for_language"] = None

        all_json.append(card_json)

    print(json.dumps(all_json, indent="\t", ensure_ascii=False))

if __name__ == "__main__":
    main()
