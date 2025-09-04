from pathlib import Path
import csv
import json

# seed_data = json.loads(Path("seed_data.sample.json").read_bytes())

# for chem in seed_data["chemistries"]:
#     if chem["name"] not in [chem_lib["chemistry"] for chem_lib in seed_data["library_type_specifications"]]:
#         print(f"chemistry {chem['name']} has no library type spec")

# base = Path("/Users/saida/Downloads")

# library_types = "Chromium(Library Types).csv"
# chemistries = "Chromium(Chemistries).csv"

# library_types, chemistries = [
#     list(csv.DictReader(((base / p).read_text().removeprefix("\ufeff").splitlines())))
#     for p in (library_types, chemistries)
# ]

# library_types = {row["Library Type Name"]: row for row in library_types if row["Library Type Name"]}

# output = []

# for chemistry in chemistries:
#     chem_lib_types = chemistry["Library Types"].split(",")

#     if chemistry["Name"] == "ATAC-v2":
#         continue
#     if chemistry["Name And Description Are Official"] == "FALSE":
#         chemistry["Name"] = input(f"name for {chemistry['Name']}: ")
#     elif chemistry["Name And Description Are Official"] == "TRUE":
#         pass
#     else:
#         raise Exception(f"Unexpected value: {chemistry['Name And Description Are Official']}")

#     if chemistry["Name"] in [o["chemistry"] for o in output]:
#         continue

#     for lib_type in chem_lib_types:
#         if found := library_types.get(lib_type):
#             output.append(
#                 {
#                     "chemistry": chemistry["Name"],
#                     "library_type": lib_type.removesuffix(" Flex")
#                     .lower()
#                     .replace(" ", "_"),
#                     "index_kit": found["Index Kit"],
#                     "cdna_volume_µl": int(found["Pre-amplification/cDNA Volume (μl)"]),
#                     "library_volume_µl": int(found["Library Volume (µl)"]),
#                 }
#             )

# print(json.dumps(sorted(output, key = lambda d: (d["chemistry"], d["library_type"])), ensure_ascii=False))
