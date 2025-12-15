import json

def main():
    json_file_location = "../data/english/cards/fossil.json"
    with open(json_file_location) as file_in:
        json_data = json.load(file_in)

    overall_number = 0
    for card in json_data:
        name = card["name"]
        number = card["number"]
        variants = card["variants"]
        for variant in variants:
            overall_number += 1
            variant_name = variant["variant"]
            if "error" in variant:
                error_type = variant["error_type"]
                print(f"{overall_number} - {number} - {name} - ERROR - {variant_name} - {error_type}")
            else:
                print(f"{overall_number} - {number} - {name} - {variant_name}")

if __name__ == "__main__":
    main()
