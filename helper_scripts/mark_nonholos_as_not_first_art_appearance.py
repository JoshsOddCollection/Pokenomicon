import json

def main():
    json_file_location = "../data/english/cards/fossil.json"
    ending_holo_card_number = 15

    with open(json_file_location) as file_in:
        json_data = json.load(file_in)

    for card in json_data:
        card_number = int(card["number"])
        if card_number <= ending_holo_card_number:
            continue
        if card_number > ending_holo_card_number * 2:
            break
        # print(card_number)
        card["is_first_art_appearance_for_language"] = False
        set_name = card["id"].split("_")[0]
        first_appearance_id_number = card_number - ending_holo_card_number
        card["id_of_first_art_appearance_for_language"] = f"{set_name}_{first_appearance_id_number}"

    print(json.dumps(json_data, indent="\t", ensure_ascii=False))

if __name__ == "__main__":
    main()
