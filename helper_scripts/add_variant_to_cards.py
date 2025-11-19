import json

def main():
    json_file_location = "../data/english/cards/fossil.json"
    new_variant_name = "1999-2000 Copyright"
    starting_number = 16
    ending_number = 62

    with open(json_file_location) as file_in:
        json_data = json.load(file_in)

    set_name = json_file_location.split("/")[-1].split(".json")[0]
    for card in json_data:
        card_number = int(card["number"])
        if card_number < starting_number:
            continue
        if card_number > ending_number:
            break

        current_num_variants = len(card["variants"])
        new_variant_number = current_num_variants + 1

        new_variant = {}
        new_variant["id"] = f"{set_name}_{card_number}_{new_variant_number}"
        new_variant["variant"] = new_variant_name

        card["variants"].append(new_variant)

    print(json.dumps(json_data, indent="\t", ensure_ascii=False))

if __name__ == "__main__":
    main()
