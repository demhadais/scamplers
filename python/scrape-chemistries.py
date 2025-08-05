from pathlib import Path
import json
from copy import deepcopy

# Load the oldest cellranger chemistries first then overwrite with newer definitions
chemistries_in: dict = json.loads(Path("chemistries_in.old2.json").read_text())
chemistries_in.update(json.loads(Path("chemistries_in.old.json").read_text()))
chemistries_in.update(json.loads(Path("chemistries_in.json").read_text()))

chemistries_missing = deepcopy(chemistries_in)

chemistries_out = []

chromium_dirnames = ["10x-genomics", "chromium", "10x-Genomics_Chromium"]
cellranger_cmdline_map = {
    "cellranger": "cellranger count",
    "cellranger-atac": "cellranger-atac count",
    "cellanger-vdj": "cellranger vdj",
    "cellranger-arc": "cellranger-arc count",
    "cellranger-multi": "cellranger multi",
}

for sub_dir in Path("/sc/service/delivery").iterdir():
    if not sub_dir.is_dir():
        continue

    try:
        chromium_dir = [p for p in sub_dir.iterdir() if p.name in chromium_dirnames][0]
    except Exception:
        continue

    for lib_dir in chromium_dir.iterdir():
        if not lib_dir.is_dir():
            continue

        cmdline = None
        chemistry_name = None

        cellranger_paths = [
            p for p in lib_dir.iterdir() if p.name.startswith("cellranger")
        ]
        if not cellranger_paths:
            continue

        cellranger_path = cellranger_paths[0]
        cmdline_path = cellranger_path / "_files" / "_cmdline"
        if cmdline_path.exists():
            cmdline = " ".join(cmdline_path.read_text().split()[:2])

        pipeline_metadata_path = lib_dir / "pipeline-metadata.json"

        if pipeline_metadata_path.exists():
            pipeline_metadata = json.loads(pipeline_metadata_path.read_text())

            try:
                metrics = pipeline_metadata["metrics"]
                for file in metrics:
                    chemistry_name = file.get("chemistry_name")
            except Exception:
                pass

            if not cmdline:
                try:
                    record = pipeline_metadata["record"]
                    tool, command = record["tool"], record["command"]
                    cmdline = f"{tool} {command}"
                except Exception:
                    pass

        if not chemistry_name:
            try:
                summary_json = json.loads(
                    (cellranger_path / "summary.json").read_text()
                )
                chemistry_name = summary_json["chemistry_name"]
            except Exception:
                ...

        if not cmdline:
            try:
                cmdline = cellranger_cmdline_map[cellranger_path.name]
            except Exception:
                ...

        if cmdline and chemistry_name:
            try:
                chemistry = deepcopy(chemistries_in[chemistry_name])
                chemistry["cmdline"] = cmdline
                chemistries_out.append(chemistry)
                del chemistries_missing[chemistry_name]
            except Exception:
                print(f"couldnt find chemistry {chemistry_name} in chemistries_in.json")
        else:
            ...

        break

Path("chemistries_out.json").write_text(json.dumps(chemistries_out))
Path("chemistries_missing.json").write_text(json.dumps(chemistries_missing))
