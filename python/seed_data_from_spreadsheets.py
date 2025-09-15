import csv
import json
import sys
from copy import deepcopy
from pathlib import Path

# A mapping of our homemade names to the names 10x uses in their product offerings. Right now, this is a minimal list containing the assays we perform as of September 2025.
MAP = {
    "Multiplex Flex Gene Expression v1 (GEM-X)": {
        "name": "Flex Gene Expression",
        "chemistry_version": "v1 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/flex-gene-expression/documentation/steps/library-prep/gem-x-flex-gene-expression-reagent-kit-for-multiplex-samples",
        "sample_multiplexing": "flex_barcode",
        "cmdlines": ["cellranger multi"],
    },
    "Multiplex Flex Gene Expression v1 (Next GEM)": {
        "name": "Flex Gene Expression",
        "chemistry_version": "v1 - Next GEM",
        "protocol_url": "https://www.10xgenomics.com/support/flex-gene-expression/documentation/steps/library-prep/chromium-single-cell-gene-expression-flex-reagent-kits-for-multiplexed-samples",
        "sample_multiplexing": "flex_barcode",
        "cmdlines": ["cellranger multi"],
    },
    "Single Cell 3' + Cell Surface Protein OCM v4": {
        "name": "Universal 3' Gene Expression",
        "chemistry_version": "v4 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-three-prime-gene-expression/documentation/steps/library-prep/gem-x-universal-3-prime-gene-expression-v-4-4-plex-reagent-kits-with-feature-barcode-technology-for-cell-surface-protein",
        "sample_multiplexing": "on_chip_multiplexing",
        "cmdlines": ["cellranger multi"],
    },
    "Single Cell 3' Gene Expression + Cell Surface Protein v4": {
        "name": "Universal 3' Gene Expression",
        "chemistry_version": "v4 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-three-prime-gene-expression/documentation/steps/library-prep/chromium-gem-x-single-cell-3-v4-gene-expression-with-feature-barcoding-technology-for-cell-surface-protein-user-guide",
        "sample_multiplexing": "singleplex",
        "cmdlines": ["cellranger count", "cellranger multi"],
    },
    "Single Cell 3' Gene Expression OCM v4": {
        "name": "Universal 3' Gene Expression",
        "chemistry_version": "v4 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-three-prime-gene-expression/documentation/steps/library-prep/gem-x-universal-3-prime-gene-expression-v-4-4-plex-reagent-kits",
        "sample_multiplexing": "on_chip_multiplexing",
        "cmdlines": ["cellranger multi"],
    },
    "Single Cell 3' Gene Expression v4": {
        "name": "Universal 3' Gene Expression",
        "chemistry_version": "v4 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-three-prime-gene-expression/documentation/steps/library-prep/chromium-gem-x-single-cell-3-v4-gene-expression-user-guide",
        "sample_multiplexing": "singleplex",
        "cmdlines": ["cellranger count", "cellranger multi"],
    },
    "Single Cell 5' Gene Expression +  V(D)J OCM v3": {
        "name": "Universal 5' Gene Expression",
        "chemistry_version": "v3 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-five-prime-gene-expression/documentation/steps/library-prep/gem-x-universal-5-prime-gene-expression-v-3-4-plex-reagent-kits",
        "sample_multiplexing": "on_chip_multiplexing",
        "cmdlines": ["cellranger multi"],
    },
    "Single Cell 5' Gene Expression +  V(D)J v3": {
        "name": "Universal 5' Gene Expression",
        "chemistry_version": "v3 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-five-prime-gene-expression/documentation/steps/library-prep/chromium-gem-x-single-cell-5-v3-gene-expression-user-guide",
        "sample_multiplexing": "on_chip_multiplexing",
        "cmdlines": ["cellranger multi"],
    },
    "Single Cell 5' Gene Expression + CRISPR Screening v3": {
        "name": "Universal 5' Gene Expression",
        "chemistry_version": "v3 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-five-prime-gene-expression/documentation/steps/library-prep/chromium-gem-x-single-cell-5-v3-gene-expression-with-feature-barcoding-technology-for-crispr-screening",
        "sample_multiplexing": "singleplex",
        "cmdlines": ["cellranger count", "cellranger multi"],
    },
    "Single Cell 5' Gene Expression OCM v3": {
        "name": "Universal 5' Gene Expression",
        "chemistry_version": "v3 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-five-prime-gene-expression/documentation/steps/library-prep/gem-x-universal-5-prime-gene-expression-v-3-4-plex-reagent-kits",
        "sample_multiplexing": "on_chip_multiplexing",
        "cmdlines": ["cellranger multi"],
    },
    "Single Cell 5' Gene Expression v3": {
        "name": "Universal 5' Gene Expression",
        "chemistry_version": "v3 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/universal-five-prime-gene-expression/documentation/steps/library-prep/chromium-gem-x-single-cell-5-v3-gene-expression-user-guide",
        "sample_multiplexing": "singleplex",
        "cmdlines": ["cellranger count", "cellranger multi"],
    },
    "Single Cell ATAC v2": {
        "name": "Epi ATAC",
        "chemistry_version": "v2",
        "protocol_url": "https://www.10xgenomics.com/support/epi-atac/documentation/steps/library-prep/chromium-single-cell-atac-reagent-kits-user-guide-v-2-chemistry",
        "sample_multiplexing": "singleplex",
        "cmdlines": ["cellranger-atac count"],
    },
    "Single Cell Multiome ATAC + Gene Expression v1": {
        "name": "Epi Multiome ATAC + Gene Expression",
        "chemistry_version": "v1",
        "protocol_url": "https://www.10xgenomics.com/support/epi-multiome/documentation/steps/library-prep/chromium-next-gem-single-cell-multiome-atac-plus-gene-expression-reagent-kits-user-guide",
        "sample_multiplexing": "singleplex",
        "cmdlines": ["cellranger-arc count"],
    },
    "Singleplex Flex Gene Expression v1 (GEM-X)": {
        "name": "Flex Gene Expression",
        "chemistry_version": "v1 - GEM-X",
        "protocol_url": "https://www.10xgenomics.com/support/flex-gene-expression/documentation/steps/library-prep/gem-x-flex-gene-expression-reagent-kit-for-singleplex-samples",
        "sample_multiplexing": "singleplex",
        "cmdlines": ["cellranger multi"],
    },
    "Singleplex Flex Gene Expression v1 (Next GEM)": {
        "name": "Flex Gene Expression",
        "chemistry_version": "v1 - Next GEM",
        "protocol_url": "https://www.10xgenomics.com/support/flex-gene-expression/documentation/steps/library-prep/chromium-fixed-rna-profiling-reagent-kits-for-singleplexed-samples",
        "sample_multiplexing": "singleplex",
        "cmdlines": ["cellranger multi"],
    },
}

seed_data = json.loads(Path("seed_data.sample.json").read_bytes())

base = Path(sys.argv[1])

library_types = "Chromium(Library Types).csv"
assays = "Chromium(Assays).csv"

library_types, assays = [
    list(csv.DictReader((base / p).read_text().removeprefix("\ufeff").splitlines()))
    for p in (library_types, assays)
]

library_types = {
    row["Library Type Name"]: row for row in library_types if row["Library Type Name"]
}

output = []

for assay in assays:
    assay_lib_types = assay["Library Types"].split(",")

    if not assay["Name"]:
        continue

    updated_assay_spec = deepcopy(MAP[assay["Name"]])
    updated_assay_spec["chromium_chip"] = assay["Chip"]
    updated_assay_spec["platform"] = "chromium"
    updated_assay_spec["library_type_specifications"] = []

    for lib_type in assay_lib_types:
        lib_type = lib_type.replace("V(D)J", "VDJ")
        library_spec = library_types[lib_type]

        updated_assay_spec["library_type_specifications"].append(
            {
                "library_type": lib_type.removesuffix(" Flex")
                .lower()
                .replace(" ", "_"),
                "index_kit": library_spec["Index Kit"],
                "cdna_volume_µl": int(
                    library_spec["Pre-amplification/cDNA Volume (μl)"],
                ),
                "library_volume_µl": int(library_spec["Library Volume (µl)"]),
            },
        )
        updated_assay_spec["library_type_specifications"].sort(
            key=lambda s: s["library_type"],
        )

    output.append(updated_assay_spec)

print(
    json.dumps(
        sorted(
            output,
            key=lambda d: (
                d["name"],
                d["chemistry_version"],
                *(s["library_type"] for s in d["library_type_specifications"]),
                d["chromium_chip"],
                d["sample_multiplexing"],
            ),
        ),
        ensure_ascii=False,
    ),
)
