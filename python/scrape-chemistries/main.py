from copy import deepcopy
import json
from pathlib import Path
from typing import Any

TENX_DIRNAMES = ["10x-genomics", "chromium", "10x-Genomics_Chromium", "visium"]
RANGER_DIR_TO_CMDLINE = {
    "cellranger": "cellranger count",
    "cellranger-atac": "cellranger-atac count",
    "cellanger-vdj": "cellranger vdj",
    "cellranger-arc": "cellranger-arc count",
    "cellranger-multi": "cellranger multi",
    "cellranger-multi-vdj": "cellranger multi",
    "spaceranger": "spaceranger count",
}


def merge_chemistry_defs(chemistry_defs: list[dict[str, Any]]) -> dict[str, Any]:
    definitive = {}
    for chemistry_def in chemistry_defs:
        definitive.update(deepcopy(chemistry_def))

    for chemistry_name in definitive:
        definitive[chemistry_name]["cmdlines"] = set()

    return definitive


def find_10x_dirs(lab_dir: Path) -> list[Path] | None:
    if not lab_dir.is_dir():
        return None

    dirs = [p for p in lab_dir.iterdir() if p.name in TENX_DIRNAMES]

    return dirs


def find_ranger_dir(lib_dir: Path) -> Path | None:
    if not lib_dir.is_dir():
        return None

    cellranger_paths = [
        p
        for p in lib_dir.iterdir()
        if p.name.startswith("cellranger") or p.name.startswith("spaceranger")
    ]

    if not cellranger_paths:
        citeseq_count = [
            p
            for p in lib_dir.iterdir()
            if "citeseq-count" in p.name or "run_report" in p.name
        ]
        if not citeseq_count:
            print(f"couldnt find *ranger path for {lib_dir}")
        return None

    return cellranger_paths[0]


def read_pipeline_metadata(lib_dir: Path) -> dict[str, Any] | None:
    pipeline_metadata_path = lib_dir / "pipeline-metadata.json"

    if not pipeline_metadata_path.exists():
        return None

    return json.loads((lib_dir / "pipeline-metadata.json").read_bytes())


def read_cmdline(
    ranger_dir: Path, pipeline_metadata: dict[str, Any] | None = None
) -> str | None:
    if pipeline_metadata:
        if record := pipeline_metadata.get("record"):
            tool, command = (record[s] for s in ("tool", "command"))
            return f"{tool} {command}"

    cmdline_path = ranger_dir / "_files" / "_cmdline"
    if cmdline_path.exists():
        return " ".join(cmdline_path.read_text().split()[:2])

    cmdline = RANGER_DIR_TO_CMDLINE.get(ranger_dir.name)
    lib_dir = ranger_dir.parent.name
    if cmdline is None:
        print(f"couldnt find cmdline for {lib_dir}")

    return cmdline


def read_chemistry_name_from_pipeline_metadata(
    pipeline_metadata: dict[str, Any], lib_dir: Path
) -> str | None:
    serialized_metrics_files = pipeline_metadata.get("metrics")
    if not serialized_metrics_files:
        return None

    if isinstance(serialized_metrics_files, list):
        for metrics_file in serialized_metrics_files:
            if chemistry_name := metrics_file.get("chemistry_name"):
                return chemistry_name

    if isinstance(serialized_metrics_files, dict):
        if chemistry_name := serialized_metrics_files.get("chemistry_name"):
            return chemistry_name

    if not isinstance(serialized_metrics_files, (list, dict)):
        raise Exception(
            f"encountered unknown format for pipeline_metadata in {lib_dir}"
        )


def read_chemistry_name_from_summary_json(ranger_dir: Path) -> str | None:
    chemistry_names: list[str | None] = [
        json.loads(p.read_bytes()).get("chemistry_name")
        for p in ranger_dir.iterdir()
        if "summary" in p.name.lower() and p.suffix == ".json"
    ]

    if not chemistry_names:
        return None

    if all(chem_name is None for chem_name in chemistry_names):
        print(f"no chemistry_name found in any summary*.json in {ranger_dir.parent}")
        return None

    return chemistry_names[0]


def read_chemistry_name_from_internals(ranger_dir: Path) -> str | None:
    finalstate_path = ranger_dir / "_files" / "_finalstate"

    if not finalstate_path.exists():
        return None

    finalstate = json.loads(finalstate_path.read_bytes())

    fork_return_vals = (
        fork["bindings"]["Return"]
        for data in finalstate
        for fork in data["forks"]
        if fork["bindings"]["Return"]
    )
    for return_val_list in fork_return_vals:
        for return_val in return_val_list:
            chemistry: dict[str, Any] | None = return_val["value"]
            if return_val["id"] == "chemistry_def" and chemistry:
                return chemistry["name"]
            elif return_val["id"] == "chemistry_defs" and chemistry:
                assert len(chemistry) == 1
                return chemistry.popitem()[1]["name"]


def add_cmdline_to_chemistry(
    cmdline: str, chemistry_name: str, chemistry_defs: dict[str, Any]
):
    chemistry = chemistry_defs[chemistry_name]
    chemistry["cmdlines"].add(cmdline)


def prepare_chemistry_for_json(
    chemistry_defs: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    for chem_def in chemistry_defs:
        chem_def["cmdlines"] = sorted(chem_def["cmdlines"])

    return sorted(chemistry_defs, key=lambda chem: chem["name"])


def parse_lib_dir(lib_dir: Path) -> tuple[str, str] | None:
    pipeline_metadata = read_pipeline_metadata(lib_dir)
    ranger_dir = find_ranger_dir(lib_dir)

    if not ranger_dir:
        return None

    cmdline = read_cmdline(ranger_dir, pipeline_metadata)

    if not cmdline or cmdline == "cellranger-atac count":
        return None

    if pipeline_metadata:
        chemistry_name = read_chemistry_name_from_pipeline_metadata(
            pipeline_metadata, lib_dir
        )
    else:
        chemistry_name = read_chemistry_name_from_summary_json(ranger_dir)

    if not chemistry_name:
        chemistry_name = read_chemistry_name_from_internals(ranger_dir)

    if not chemistry_name:
        print(f"couldnt find chemistry anywhere for {lib_dir}. cmdline: {cmdline}")
        return None

    return cmdline, chemistry_name


def parse_tenx_dir(tenx_dir: Path) -> list[tuple[str, str]]:
    res = (parse_lib_dir(lib_dir) for lib_dir in tenx_dir.iterdir())

    return [tup for tup in res if tup]


def parse_lab_dir(lab_dir: Path) -> list[tuple[str, str]] | None:
    tenx_dirs = find_10x_dirs(lab_dir)

    if not tenx_dirs:
        return None

    return [
        (cmdline, chemistry_name)
        for tenx_dir in tenx_dirs
        for cmdline, chemistry_name in parse_tenx_dir(tenx_dir)
    ]


def main(
    delivery_dir: str = "/sc/service/delivery",
    chemistry_def_paths: list[str] = [],
    out: str = "chemistry-defs.updated.json",
):
    chemistry_defs: list[dict[str, Any]] = [
        json.loads(Path(p).read_bytes()) for p in chemistry_def_paths
    ]
    merged_chemistry_defs = merge_chemistry_defs(chemistry_defs)
    del chemistry_defs

    for lab_dir in Path(delivery_dir).iterdir():
        cmdline_chemistry_pairs = parse_lab_dir(lab_dir)

        if not cmdline_chemistry_pairs:
            continue

        for cmdline, chemistry_name in cmdline_chemistry_pairs:
            add_cmdline_to_chemistry(cmdline, chemistry_name, merged_chemistry_defs)

    chemistry_defs = [
        val for val in merged_chemistry_defs.values() if val.get("cmdlines")
    ]
    chemistry_defs = prepare_chemistry_for_json(chemistry_defs)

    Path(out).write_text(json.dumps(chemistry_defs))


if __name__ == "__main__":
    try:
        import fire

        fire.Fire(main)
    except Exception:
        main(
            delivery_dir="/sc/service/delivery",
            chemistry_def_paths=[
                "cr7.2.0.json",
                "cr8.0.0.json",
                "cr8.0.1.json",
                "cr9.0.1.json",
            ],
            out="chemistry-defs.updated.json",
        )
