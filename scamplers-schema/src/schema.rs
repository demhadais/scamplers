// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (prefix, hash) {
        prefix -> Text,
        hash -> Text,
        user_id -> Uuid,
    }
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
        additional_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    cdna_measurements (id) {
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
    chip_loadings (id) {
        id -> Uuid,
        gems_id -> Uuid,
        suspension_id -> Nullable<Uuid>,
        suspension_pool_id -> Nullable<Uuid>,
        suspension_volume_loaded -> Jsonb,
        buffer_volume_loaded -> Jsonb,
        additional_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    chromium_dataset_libraries (dataset_id, library_id) {
        dataset_id -> Uuid,
        library_id -> Uuid,
    }
}

diesel::table! {
    chromium_dataset_web_summaries (id) {
        id -> Uuid,
        dataset_id -> Uuid,
        web_summary -> Text,
    }
}

diesel::table! {
    chromium_datasets (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
        lab_id -> Uuid,
        data_path -> Text,
        delivered_at -> Timestamptz,
        metrics -> Jsonb,
    }
}

diesel::table! {
    chromium_runs (id) {
        id -> Uuid,
        links -> Jsonb,
        assay_id -> Uuid,
        readable_id -> Text,
        run_at -> Timestamptz,
        run_by -> Uuid,
        succeeded -> Bool,
        additional_data -> Nullable<Jsonb>,
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
    dual_index_sets (name) {
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
        chromium_run_id -> Uuid,
    }
}

diesel::table! {
    index_kits (name) {
        name -> Text,
    }
}

diesel::table! {
    institutions (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
    }
}

diesel::table! {
    lab_membership (lab_id, member_id) {
        lab_id -> Uuid,
        member_id -> Uuid,
    }
}

diesel::table! {
    labs (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
        pi_id -> Uuid,
        delivery_dir -> Text,
    }
}

diesel::table! {
    libraries (id) {
        id -> Uuid,
        links -> Jsonb,
        readable_id -> Text,
        cdna_id -> Uuid,
        single_index_set_name -> Nullable<Text>,
        dual_index_set_name -> Nullable<Text>,
        number_of_sample_index_pcr_cycles -> Int4,
        target_reads_per_cell -> Int4,
        prepared_at -> Timestamptz,
        additional_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    library_measurements (id) {
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
    library_type_specifications (assay_id, library_type) {
        assay_id -> Uuid,
        library_type -> Text,
        index_kit -> Text,
        #[sql_name = "cdna_volume_µl"]
        cdna_volume_l -> Float4,
        #[sql_name = "library_volume_µl"]
        library_volume_l -> Float4,
    }
}

diesel::table! {
    multiplexing_tags (id) {
        id -> Uuid,
        tag_id -> Text,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

diesel::table! {
    people (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
        email -> Nullable<Text>,
        institution_id -> Uuid,
        orcid -> Nullable<Text>,
        ms_user_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    sequencing_runs (id) {
        id -> Uuid,
        links -> Jsonb,
        readable_id -> Text,
        begun_at -> Timestamptz,
        finished_at -> Nullable<Timestamptz>,
        additional_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    sequencing_submissions (library_id, sequencing_run_id) {
        library_id -> Uuid,
        sequencing_run_id -> Uuid,
        submitted_at -> Timestamptz,
    }
}

diesel::table! {
    single_index_sets (name) {
        name -> Text,
        kit -> Text,
        well -> Text,
        sequences -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    specimen_measurements (id) {
        id -> Uuid,
        specimen_id -> Uuid,
        measured_by -> Uuid,
        data -> Jsonb,
    }
}

diesel::table! {
    specimens (id) {
        id -> Uuid,
        readable_id -> Text,
        links -> Jsonb,
        name -> Text,
        submitted_by -> Uuid,
        lab_id -> Uuid,
        received_at -> Timestamptz,
        species -> Array<Nullable<Text>>,
        returned_at -> Nullable<Timestamptz>,
        returned_by -> Nullable<Uuid>,
        #[sql_name = "type"]
        type_ -> Text,
        embedded_in -> Nullable<Text>,
        fixative -> Nullable<Text>,
        frozen -> Bool,
        cryopreserved -> Bool,
        tissue -> Text,
        additional_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    suspension_measurements (id) {
        id -> Uuid,
        suspension_id -> Uuid,
        measured_by -> Uuid,
        data -> Jsonb,
    }
}

diesel::table! {
    suspension_pool_measurements (id) {
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
    suspension_pools (id) {
        id -> Uuid,
        links -> Jsonb,
        readable_id -> Text,
        name -> Text,
        pooled_at -> Timestamptz,
        additional_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    suspension_preparers (suspension_id, prepared_by) {
        suspension_id -> Uuid,
        prepared_by -> Uuid,
    }
}

diesel::table! {
    suspensions (id) {
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
        additional_data -> Nullable<Jsonb>,
    }
}

diesel::table! {
    tenx_assays (id) {
        id -> Uuid,
        links -> Jsonb,
        name -> Text,
        library_types -> Nullable<Array<Nullable<Text>>>,
        sample_multiplexing -> Nullable<Text>,
        chemistry_version -> Text,
        protocol_url -> Text,
        chromium_chip -> Nullable<Text>,
        cmdlines -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::joinable!(api_keys -> people (user_id));
diesel::joinable!(cdna -> gems (gems_id));
diesel::joinable!(cdna_measurements -> cdna (cdna_id));
diesel::joinable!(cdna_measurements -> people (measured_by));
diesel::joinable!(cdna_preparers -> cdna (cdna_id));
diesel::joinable!(cdna_preparers -> people (prepared_by));
diesel::joinable!(chip_loadings -> gems (gems_id));
diesel::joinable!(chip_loadings -> suspension_pools (suspension_pool_id));
diesel::joinable!(chip_loadings -> suspensions (suspension_id));
diesel::joinable!(chromium_dataset_libraries -> chromium_datasets (dataset_id));
diesel::joinable!(chromium_dataset_libraries -> libraries (library_id));
diesel::joinable!(chromium_dataset_web_summaries -> chromium_datasets (dataset_id));
diesel::joinable!(chromium_datasets -> labs (lab_id));
diesel::joinable!(chromium_runs -> people (run_by));
diesel::joinable!(chromium_runs -> tenx_assays (assay_id));
diesel::joinable!(committee_approval -> institutions (institution_id));
diesel::joinable!(committee_approval -> specimens (specimen_id));
diesel::joinable!(dual_index_sets -> index_kits (kit));
diesel::joinable!(gems -> chromium_runs (chromium_run_id));
diesel::joinable!(lab_membership -> labs (lab_id));
diesel::joinable!(lab_membership -> people (member_id));
diesel::joinable!(labs -> people (pi_id));
diesel::joinable!(libraries -> cdna (cdna_id));
diesel::joinable!(libraries -> dual_index_sets (dual_index_set_name));
diesel::joinable!(libraries -> single_index_sets (single_index_set_name));
diesel::joinable!(library_measurements -> libraries (library_id));
diesel::joinable!(library_measurements -> people (measured_by));
diesel::joinable!(library_preparers -> libraries (library_id));
diesel::joinable!(library_preparers -> people (prepared_by));
diesel::joinable!(library_type_specifications -> index_kits (index_kit));
diesel::joinable!(library_type_specifications -> tenx_assays (assay_id));
diesel::joinable!(people -> institutions (institution_id));
diesel::joinable!(sequencing_submissions -> libraries (library_id));
diesel::joinable!(sequencing_submissions -> sequencing_runs (sequencing_run_id));
diesel::joinable!(single_index_sets -> index_kits (kit));
diesel::joinable!(specimen_measurements -> people (measured_by));
diesel::joinable!(specimen_measurements -> specimens (specimen_id));
diesel::joinable!(specimens -> labs (lab_id));
diesel::joinable!(suspension_measurements -> people (measured_by));
diesel::joinable!(suspension_measurements -> suspensions (suspension_id));
diesel::joinable!(suspension_pool_measurements -> people (measured_by));
diesel::joinable!(suspension_pool_measurements -> suspension_pools (pool_id));
diesel::joinable!(suspension_pool_preparers -> people (prepared_by));
diesel::joinable!(suspension_pool_preparers -> suspension_pools (pool_id));
diesel::joinable!(suspension_preparers -> people (prepared_by));
diesel::joinable!(suspension_preparers -> suspensions (suspension_id));
diesel::joinable!(suspensions -> multiplexing_tags (multiplexing_tag_id));
diesel::joinable!(suspensions -> specimens (parent_specimen_id));
diesel::joinable!(suspensions -> suspension_pools (pooled_into));

diesel::allow_tables_to_appear_in_same_query!(
    api_keys,
    cdna,
    cdna_measurements,
    cdna_preparers,
    chip_loadings,
    chromium_dataset_libraries,
    chromium_dataset_web_summaries,
    chromium_datasets,
    chromium_runs,
    committee_approval,
    dual_index_sets,
    gems,
    index_kits,
    institutions,
    lab_membership,
    labs,
    libraries,
    library_measurements,
    library_preparers,
    library_type_specifications,
    multiplexing_tags,
    people,
    sequencing_runs,
    sequencing_submissions,
    single_index_sets,
    specimen_measurements,
    specimens,
    suspension_measurements,
    suspension_pool_measurements,
    suspension_pool_preparers,
    suspension_pools,
    suspension_preparers,
    suspensions,
    tenx_assays,
);
