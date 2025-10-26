create table library_type_specifications (
    assay_id uuid references tenx_assays on delete restrict on update restrict not null,
    library_type text not null,
    index_kit text references index_kits on delete restrict on update restrict not null,
    cdna_volume_µl real not null,
    library_volume_µl real not null,
    primary key (assay_id, library_type)
);
