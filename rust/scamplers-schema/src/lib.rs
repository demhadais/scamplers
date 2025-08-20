// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(std::fmt::Debug, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "hashed_key"))]
    pub struct HashedKey;
}

diesel::table! {
    cdna (id) {
        id -> Uuid,
        links -> Jsonb,
        library_type -> Text,
        readable_id -> Text,
        prepared_at -> Timestamptz,
        gems_id -> Nullable<Uuid>,
        n_amplification_cycles -> Int4,
        storage_location -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    cdna_measurement (id) {
        id -> Uuid,
        cdna_id -> Uuid,
        measured_by -> Uuid,
        data -> Jsonb,
    }
}

diesel::table! {
    cdna_preparers (cdna_id, prepared_by) {
        cdna_id -> Uuid,
        prepared_by -> Uuid,
    }
}

diesel::table! {
    chemistry (name) {
        name -> Text,
        description -> Text,
        definition -> Jsonb,
        cmdlines -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    chip_loading (id) {
        id -> Uuid,
        gems_id -> Uuid,
        suspension_id -> Nullable<Uuid>,
        suspension_pool_id -> Nullable<Uuid>,
        suspension_volume_loaded -> Jsonb,
        buffer_volume_loaded -> Jsonb,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    chromium_run (id) {
        id -> Uuid,
        links -> Jsonb,
        readable_id -> Text,
        chip -> Text,
        run_at -> Timestamptz,
        run_by -> Uuid,
        succeeded -> Bool,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    committee_approval (institution_id, committee_type, specimen_id) {
        institution_id -> Uuid,
        specimen_id -> Uuid,
        committee_type -> Text,
        compliance_identifier -> Text,
    }
}

diesel::table! {
    dataset (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
        lab_id -> Uuid,
        data_path -> Text,
        delivered_at -> Timestamptz,
        gems_id -> Nullable<Uuid>,
        metrics -> Nullable<Jsonb>,
        web_summary -> Nullable<Text>,
    }
}

diesel::table! {
    dual_index_set (name) {
        name -> Text,
        kit -> Text,
        well -> Text,
        index_i7 -> Text,
        index2_workflow_a_i5 -> Text,
        index2_workflow_b_i5 -> Text,
    }
}

diesel::table! {
    gems (id) {
        id -> Uuid,
        readable_id -> Text,
        chemistry -> Nullable<Text>,
        chromium_run_id -> Uuid,
    }
}

diesel::table! {
    index_kit (name) {
        name -> Text,
    }
}

diesel::table! {
    institution (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
    }
}

diesel::table! {
    lab (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
        pi_id -> Uuid,
        delivery_dir -> Text,
    }
}

diesel::table! {
    lab_membership (lab_id, member_id) {
        lab_id -> Uuid,
        member_id -> Uuid,
    }
}

diesel::table! {
    library (id) {
        id -> Uuid,
        links -> Jsonb,
        readable_id -> Text,
        cdna_id -> Uuid,
        single_index_set_name -> Nullable<Text>,
        dual_index_set_name -> Nullable<Text>,
        number_of_sample_index_pcr_cycles -> Int4,
        target_reads_per_cell -> Int4,
        prepared_at -> Timestamptz,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    library_measurement (id) {
        id -> Uuid,
        library_id -> Uuid,
        measured_by -> Uuid,
        data -> Jsonb,
    }
}

diesel::table! {
    library_preparers (library_id, prepared_by) {
        library_id -> Uuid,
        prepared_by -> Uuid,
    }
}

diesel::table! {
    library_type_specification (chemistry, library_type) {
        chemistry -> Text,
        library_type -> Text,
        index_kit -> Text,
        #[sql_name = "cdna_volume_µl"]
        cdna_volume_l -> Float4,
        #[sql_name = "library_volume_µl"]
        library_volume_l -> Float4,
    }
}

diesel::table! {
    multiplexing_tag (id) {
        id -> Uuid,
        tag_id -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HashedKey;

    person (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
        email -> Nullable<Text>,
        institution_id -> Uuid,
        orcid -> Nullable<Text>,
        ms_user_id -> Nullable<Uuid>,
        hashed_api_key -> Nullable<HashedKey>,
    }
}

diesel::table! {
    sequencing_run (id) {
        id -> Uuid,
        links -> Jsonb,
        readable_id -> Text,
        begun_at -> Timestamptz,
        finished_at -> Nullable<Timestamptz>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    sequencing_submissions (library_id, sequencing_run_id) {
        library_id -> Uuid,
        sequencing_run_id -> Uuid,
        fastq_paths -> Nullable<Array<Nullable<Text>>>,
        submitted_at -> Timestamptz,
    }
}

diesel::table! {
    single_index_set (name) {
        name -> Text,
        kit -> Text,
        well -> Text,
        sequences -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    specimen (id) {
        id -> Uuid,
        readable_id -> Text,
        links -> Jsonb,
        name -> Text,
        submitted_by -> Uuid,
        lab_id -> Uuid,
        received_at -> Timestamptz,
        species -> Array<Nullable<Text>>,
        notes -> Nullable<Text>,
        returned_at -> Nullable<Timestamptz>,
        returned_by -> Nullable<Uuid>,
        #[sql_name = "type"]
        type_ -> Text,
        embedded_in -> Nullable<Text>,
        fixative -> Nullable<Text>,
        frozen -> Bool,
        cryopreserved -> Bool,
        storage_buffer -> Nullable<Text>,
    }
}

diesel::table! {
    specimen_measurement (id) {
        id -> Uuid,
        specimen_id -> Uuid,
        measured_by -> Uuid,
        data -> Jsonb,
    }
}

diesel::table! {
    suspension (id) {
        id -> Uuid,
        links -> Jsonb,
        readable_id -> Text,
        parent_specimen_id -> Uuid,
        biological_material -> Text,
        created_at -> Nullable<Timestamptz>,
        pooled_into -> Nullable<Uuid>,
        multiplexing_tag_id -> Nullable<Uuid>,
        lysis_duration_minutes -> Nullable<Float4>,
        target_cell_recovery -> Float4,
        target_reads_per_cell -> Int4,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    suspension_measurement (id) {
        id -> Uuid,
        suspension_id -> Uuid,
        measured_by -> Uuid,
        data -> Jsonb,
    }
}

diesel::table! {
    suspension_pool (id) {
        id -> Uuid,
        links -> Jsonb,
        readable_id -> Text,
        name -> Text,
        pooled_at -> Timestamptz,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    suspension_pool_measurement (id) {
        id -> Uuid,
        pool_id -> Uuid,
        measured_by -> Uuid,
        data -> Jsonb,
    }
}

diesel::table! {
    suspension_pool_preparers (pool_id, prepared_by) {
        pool_id -> Uuid,
        prepared_by -> Uuid,
    }
}

diesel::table! {
    suspension_preparers (suspension_id, prepared_by) {
        suspension_id -> Uuid,
        prepared_by -> Uuid,
    }
}

diesel::joinable!(cdna -> gems (gems_id));
diesel::joinable!(cdna_measurement -> cdna (cdna_id));
diesel::joinable!(cdna_measurement -> person (measured_by));
diesel::joinable!(cdna_preparers -> cdna (cdna_id));
diesel::joinable!(cdna_preparers -> person (prepared_by));
diesel::joinable!(chip_loading -> gems (gems_id));
diesel::joinable!(chip_loading -> suspension (suspension_id));
diesel::joinable!(chip_loading -> suspension_pool (suspension_pool_id));
diesel::joinable!(chromium_run -> person (run_by));
diesel::joinable!(committee_approval -> institution (institution_id));
diesel::joinable!(committee_approval -> specimen (specimen_id));
diesel::joinable!(dataset -> gems (gems_id));
diesel::joinable!(dataset -> lab (lab_id));
diesel::joinable!(dual_index_set -> index_kit (kit));
diesel::joinable!(gems -> chemistry (chemistry));
diesel::joinable!(gems -> chromium_run (chromium_run_id));
diesel::joinable!(lab -> person (pi_id));
diesel::joinable!(lab_membership -> lab (lab_id));
diesel::joinable!(lab_membership -> person (member_id));
diesel::joinable!(library -> cdna (cdna_id));
diesel::joinable!(library -> dual_index_set (dual_index_set_name));
diesel::joinable!(library -> single_index_set (single_index_set_name));
diesel::joinable!(library_measurement -> library (library_id));
diesel::joinable!(library_measurement -> person (measured_by));
diesel::joinable!(library_preparers -> library (library_id));
diesel::joinable!(library_preparers -> person (prepared_by));
diesel::joinable!(library_type_specification -> chemistry (chemistry));
diesel::joinable!(library_type_specification -> index_kit (index_kit));
diesel::joinable!(person -> institution (institution_id));
diesel::joinable!(sequencing_submissions -> library (library_id));
diesel::joinable!(sequencing_submissions -> sequencing_run (sequencing_run_id));
diesel::joinable!(single_index_set -> index_kit (kit));
diesel::joinable!(specimen -> lab (lab_id));
diesel::joinable!(specimen_measurement -> person (measured_by));
diesel::joinable!(specimen_measurement -> specimen (specimen_id));
diesel::joinable!(suspension -> multiplexing_tag (multiplexing_tag_id));
diesel::joinable!(suspension -> specimen (parent_specimen_id));
diesel::joinable!(suspension -> suspension_pool (pooled_into));
diesel::joinable!(suspension_measurement -> person (measured_by));
diesel::joinable!(suspension_measurement -> suspension (suspension_id));
diesel::joinable!(suspension_pool_measurement -> person (measured_by));
diesel::joinable!(suspension_pool_measurement -> suspension_pool (pool_id));
diesel::joinable!(suspension_pool_preparers -> person (prepared_by));
diesel::joinable!(suspension_pool_preparers -> suspension_pool (pool_id));
diesel::joinable!(suspension_preparers -> person (prepared_by));
diesel::joinable!(suspension_preparers -> suspension (suspension_id));

diesel::allow_tables_to_appear_in_same_query!(
    cdna,
    cdna_measurement,
    cdna_preparers,
    chemistry,
    chip_loading,
    chromium_run,
    committee_approval,
    dataset,
    dual_index_set,
    gems,
    index_kit,
    institution,
    lab,
    lab_membership,
    library,
    library_measurement,
    library_preparers,
    library_type_specification,
    multiplexing_tag,
    person,
    sequencing_run,
    sequencing_submissions,
    single_index_set,
    specimen,
    specimen_measurement,
    suspension,
    suspension_measurement,
    suspension_pool,
    suspension_pool_measurement,
    suspension_pool_preparers,
    suspension_preparers,
);
