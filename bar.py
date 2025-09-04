from pathlib import Path
import csv

base = Path("/Users/saida/Downloads")

f1 = ("Chromium(Suspensions).csv", 1)
f2 = ("Chromium(GEMs-Suspensions).csv", 0)

suspensions, gems_suspensions = [
    list(csv.DictReader(((base / filename).read_text().removeprefix("\ufeff").splitlines()[firstrow:])))
    for (filename, firstrow) in (f1, f2)
]
suspensions = {s["Suspension ID"]: s for s in suspensions}

submitters = set()

for gems_row in gems_suspensions:
    if gems_row["Chemistry"] == "Single Cell 5' PE":
        suspension_id = gems_row["Suspension ID"]
        suspension = suspensions[suspension_id]
        submitters.add(suspension["Lab"])


print(submitters)
