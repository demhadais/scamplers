/* tslint:disable */
/* eslint-disable */
export enum LibraryType {
  AntibodyCapture = 0,
  AntigenCapture = 1,
  ChromatinAccessibility = 2,
  CrisprGuideCapture = 3,
  Custom = 4,
  GeneExpression = 5,
  MultiplexingCapture = 6,
  Vdj = 7,
  VdjB = 8,
  VdjT = 9,
  VdjTGd = 10,
}
export enum SampleMultiplexing {
  Cellplex = 0,
  FlexBarcode = 1,
  Hashtag = 2,
  OnChipMultiplexing = 3,
  Singleplex = 4,
}
export enum Species {
  AmbystomaMexicanum = 0,
  CanisFamiliaris = 1,
  CallithrixJacchus = 2,
  DrosophilaMelanogaster = 3,
  GasterosteusAculeatus = 4,
  HomoSapiens = 5,
  MusMusculus = 6,
  RattusNorvegicus = 7,
  SminthopsisCrassicaudata = 8,
}
export enum SpecimenType {
  Block = 0,
  Suspension = 1,
  Tissue = 2,
}
export enum UserRole {
  AppAdmin = 0,
  BiologyStaff = 1,
  ComputationalStaff = 2,
}
export class Cdna {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): Cdna;
  static from_json_string(json_str: string): Cdna;
  static from_base64_json(base64_json_bytes: string): Cdna;
  summary: CdnaSummary;
  prepared_by: string[];
  measurements: CdnaMeasurement[];
}
export class CdnaGemsError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): CdnaGemsError;
  static from_json_string(json_str: string): CdnaGemsError;
  static from_base64_json(base64_json_bytes: string): CdnaGemsError;
  message: string;
}
export class CdnaLibraryTypeError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): CdnaLibraryTypeError;
  static from_json_string(json_str: string): CdnaLibraryTypeError;
  static from_base64_json(base64_json_bytes: string): CdnaLibraryTypeError;
  expected_specifications: LibraryTypeSpecification[];
}
export class CdnaMeasurement {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): CdnaMeasurement;
  static from_json_string(json_str: string): CdnaMeasurement;
  static from_base64_json(base64_json_bytes: string): CdnaMeasurement;
  id: string;
  cdna_id: string;
  measured_by: string;
}
export class CdnaQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): CdnaQuery;
  static from_json_string(json_str: string): CdnaQuery;
  static from_base64_json(base64_json_bytes: string): CdnaQuery;
  constructor();
  ids: string[];
  order_by: OrderBy[];
  pagination: Pagination;
}
export class CdnaSummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): CdnaSummary;
  static from_json_string(json_str: string): CdnaSummary;
  static from_base64_json(base64_json_bytes: string): CdnaSummary;
  id: string;
  readonly links: Map<any, any>;
  library_type: LibraryType;
  readable_id: string;
  prepared_at: Date;
  n_amplification_cycles: number;
  get additional_data(): any | undefined;
  set additional_data(value: any | null | undefined);
}
export class ChromiumDataset {
  private constructor();
  free(): void;
  summary: ChromiumDatasetSummary;
  library_ids: string[];
}
export class ChromiumDatasetError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ChromiumDatasetError;
  static from_json_string(json_str: string): ChromiumDatasetError;
  static from_base64_json(base64_json_bytes: string): ChromiumDatasetError;
  message: string;
}
export class ChromiumDatasetQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ChromiumDatasetQuery;
  static from_json_string(json_str: string): ChromiumDatasetQuery;
  static from_base64_json(base64_json_bytes: string): ChromiumDatasetQuery;
  constructor();
  ids: string[];
  names: string[];
  lab_ids: string[];
  get delivered_before(): Date | undefined;
  set delivered_before(value: Date | null | undefined);
  get delivered_after(): Date | undefined;
  set delivered_after(value: Date | null | undefined);
  get tenx_assay(): TenxAssayQuery | undefined;
  set tenx_assay(value: TenxAssayQuery | null | undefined);
  get specimen(): SpecimenQuery | undefined;
  set specimen(value: SpecimenQuery | null | undefined);
  order_by: OrderBy[];
  pagination: Pagination;
}
export class ChromiumDatasetSummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ChromiumDatasetSummary;
  static from_json_string(json_str: string): ChromiumDatasetSummary;
  static from_base64_json(base64_json_bytes: string): ChromiumDatasetSummary;
  id: string;
  readonly links: Map<any, any>;
  name: string;
  lab: LabSummary;
  data_path: string;
  delivered_at: Date;
  tenx_assay: TenxAssay;
  web_summary: string;
}
export class ChromiumRun {
  private constructor();
  free(): void;
  info: ChromiumRunSummaryWithParents;
  gems: Gems[];
}
export class ChromiumRunQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ChromiumRunQuery;
  static from_json_string(json_str: string): ChromiumRunQuery;
  static from_base64_json(base64_json_bytes: string): ChromiumRunQuery;
  constructor();
  ids: string[];
  readable_ids: string[];
  get assay(): TenxAssayQuery | undefined;
  set assay(value: TenxAssayQuery | null | undefined);
  get run_before(): Date | undefined;
  set run_before(value: Date | null | undefined);
  get run_after(): Date | undefined;
  set run_after(value: Date | null | undefined);
  get succeeded(): boolean | undefined;
  set succeeded(value: boolean | null | undefined);
  additional_data: any[];
  order_by: OrderBy[];
  pagination: Pagination;
}
export class ChromiumRunSummaryWithParents {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ChromiumRunSummaryWithParents;
  static from_json_string(json_str: string): ChromiumRunSummaryWithParents;
  static from_base64_json(base64_json_bytes: string): ChromiumRunSummaryWithParents;
  id: string;
  readonly links: Map<any, any>;
  readable_id: string;
  run_at: Date;
  run_by: string;
  succeeded: boolean;
  assay: TenxAssay;
  get additional_data(): any | undefined;
  set additional_data(value: any | null | undefined);
}
export class ClientError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ClientError;
  static from_json_string(json_str: string): ClientError;
  static from_base64_json(base64_json_bytes: string): ClientError;
  message: string;
}
export class CommitteeApproval {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): CommitteeApproval;
  static from_json_string(json_str: string): CommitteeApproval;
  static from_base64_json(base64_json_bytes: string): CommitteeApproval;
  institution_id: string;
  specimen_id: string;
  institution: Institution;
  committee_type: string;
  compliance_identifier: string;
}
export class Concentration {
  private constructor();
  free(): void;
  value: number;
  readonly unit: string;
}
export class CreatedUser {
  private constructor();
  free(): void;
  person: Person;
  api_key: string;
}
export class DatasetMetricsFileParseError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): DatasetMetricsFileParseError;
  static from_json_string(json_str: string): DatasetMetricsFileParseError;
  static from_base64_json(base64_json_bytes: string): DatasetMetricsFileParseError;
  message: string;
}
export class DatasetNMetricsFilesError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): DatasetNMetricsFilesError;
  static from_json_string(json_str: string): DatasetNMetricsFilesError;
  static from_base64_json(base64_json_bytes: string): DatasetNMetricsFilesError;
  expected_n_metrics_files: bigint;
  found_n_metrics_files: bigint;
}
export class DuplicateResourceError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): DuplicateResourceError;
  static from_json_string(json_str: string): DuplicateResourceError;
  static from_base64_json(base64_json_bytes: string): DuplicateResourceError;
  entity: string;
  fields: string[];
  values: string[];
}
export class EmptyStringError {
  private constructor();
  free(): void;
}
export class Gems {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): Gems;
  static from_json_string(json_str: string): Gems;
  static from_base64_json(base64_json_bytes: string): Gems;
  id: string;
  readable_id: string;
  chromium_run_id: string;
}
export class Institution {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): Institution;
  static from_json_string(json_str: string): Institution;
  static from_base64_json(base64_json_bytes: string): Institution;
  id: string;
  readonly links: Map<any, any>;
  name: string;
}
export class InstitutionQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): InstitutionQuery;
  static from_json_string(json_str: string): InstitutionQuery;
  static from_base64_json(base64_json_bytes: string): InstitutionQuery;
  constructor();
  ids: string[];
  names: string[];
  order_by: OrderBy[];
  pagination: Pagination;
}
export class InvalidDataError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): InvalidDataError;
  static from_json_string(json_str: string): InvalidDataError;
  static from_base64_json(base64_json_bytes: string): InvalidDataError;
  message: string;
}
export class InvalidMeasurementError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): InvalidMeasurementError;
  static from_json_string(json_str: string): InvalidMeasurementError;
  static from_base64_json(base64_json_bytes: string): InvalidMeasurementError;
  message: string;
}
export class InvalidReferenceError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): InvalidReferenceError;
  static from_json_string(json_str: string): InvalidReferenceError;
  static from_base64_json(base64_json_bytes: string): InvalidReferenceError;
  entity: string;
  referenced_entity: string;
  get value(): string;
  set value(value: string | null | undefined);
}
export class Lab {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): Lab;
  static from_json_string(json_str: string): Lab;
  static from_base64_json(base64_json_bytes: string): Lab;
  info: LabSummaryWithParents;
  members: PersonSummary[];
}
export class LabQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): LabQuery;
  static from_json_string(json_str: string): LabQuery;
  static from_base64_json(base64_json_bytes: string): LabQuery;
  constructor();
  ids: string[];
  names: string[];
  order_by: OrderBy[];
  pagination: Pagination;
}
export class LabSummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): LabSummary;
  static from_json_string(json_str: string): LabSummary;
  static from_base64_json(base64_json_bytes: string): LabSummary;
  id: string;
  readonly links: Map<any, any>;
  name: string;
  delivery_dir: string;
}
export class LabSummaryWithParents {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): LabSummaryWithParents;
  static from_json_string(json_str: string): LabSummaryWithParents;
  static from_base64_json(base64_json_bytes: string): LabSummaryWithParents;
  id_: string;
  summary: LabSummary;
  pi: PersonSummary;
}
export class LabUpdate {
  private constructor();
  free(): void;
  id: string;
  get name(): string;
  set name(value: string | null | undefined);
  get pi_id(): string;
  set pi_id(value: string | null | undefined);
  get delivery_dir(): string;
  set delivery_dir(value: string | null | undefined);
  add_members: string[];
  remove_members: string[];
}
export class Library {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): Library;
  static from_json_string(json_str: string): Library;
  static from_base64_json(base64_json_bytes: string): Library;
  info: LibrarySummaryWithParents;
  prepared_by: string[];
  measurements: LibraryMeasurement[];
}
export class LibraryIndexSetError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): LibraryIndexSetError;
  static from_json_string(json_str: string): LibraryIndexSetError;
  static from_base64_json(base64_json_bytes: string): LibraryIndexSetError;
  message: string;
}
export class LibraryMeasurement {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): LibraryMeasurement;
  static from_json_string(json_str: string): LibraryMeasurement;
  static from_base64_json(base64_json_bytes: string): LibraryMeasurement;
  id: string;
  library_id: string;
  measured_by: string;
}
export class LibraryQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): LibraryQuery;
  static from_json_string(json_str: string): LibraryQuery;
  static from_base64_json(base64_json_bytes: string): LibraryQuery;
  constructor();
  ids: string[];
  library_types: any[];
  pagination: Pagination;
  order_by: OrderBy[];
}
export class LibrarySummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): LibrarySummary;
  static from_json_string(json_str: string): LibrarySummary;
  static from_base64_json(base64_json_bytes: string): LibrarySummary;
  id: string;
  readonly links: Map<any, any>;
  readable_id: string;
  get single_index_set_name(): string;
  set single_index_set_name(value: string | null | undefined);
  get dual_index_set_name(): string;
  set dual_index_set_name(value: string | null | undefined);
  number_of_sample_index_pcr_cycles: number;
  target_reads_per_cell: number;
  prepared_at: Date;
  get additional_data(): any | undefined;
  set additional_data(value: any | null | undefined);
}
export class LibrarySummaryWithParents {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): LibrarySummaryWithParents;
  static from_json_string(json_str: string): LibrarySummaryWithParents;
  static from_base64_json(base64_json_bytes: string): LibrarySummaryWithParents;
  id_: string;
  summary: LibrarySummary;
  cdna: CdnaSummary;
}
export class LibraryTypeSpecification {
  private constructor();
  free(): void;
  assay_id: string;
  library_type: LibraryType;
  index_kit: string;
}
export class MalformedRequestError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): MalformedRequestError;
  static from_json_string(json_str: string): MalformedRequestError;
  static from_base64_json(base64_json_bytes: string): MalformedRequestError;
  message: string;
}
export class MultiplexingTag {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): MultiplexingTag;
  static from_json_string(json_str: string): MultiplexingTag;
  static from_base64_json(base64_json_bytes: string): MultiplexingTag;
  id: string;
  tag_id: string;
  type_: string;
}
export class NewLab {
  private constructor();
  free(): void;
  name: string;
  pi_id: string;
  delivery_dir: string;
  member_ids: string[];
}
export class NewPerson {
  free(): void;
  constructor(ms_user_id: string);
  name: string;
  email: string;
  institution_id: string;
  get ms_user_id(): string;
  set ms_user_id(value: string | null | undefined);
}
export class OrderBy {
  free(): void;
  constructor(field: string, descending: boolean);
  field: string;
  descending: boolean;
}
export class Pagination {
  private constructor();
  free(): void;
  limit: bigint;
  offset: bigint;
}
export class PermissionDeniedError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): PermissionDeniedError;
  static from_json_string(json_str: string): PermissionDeniedError;
  static from_base64_json(base64_json_bytes: string): PermissionDeniedError;
  message: string;
}
export class Person {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): Person;
  static from_json_string(json_str: string): Person;
  static from_base64_json(base64_json_bytes: string): Person;
  info: PersonSummaryWithParents;
  roles: any[];
}
export class PersonQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): PersonQuery;
  static from_json_string(json_str: string): PersonQuery;
  static from_base64_json(base64_json_bytes: string): PersonQuery;
  constructor();
  ids: string[];
  names: string[];
  emails: string[];
  orcids: string[];
  ms_user_ids: string[];
  order_by: OrderBy[];
  pagination: Pagination;
}
export class PersonSummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): PersonSummary;
  static from_json_string(json_str: string): PersonSummary;
  static from_base64_json(base64_json_bytes: string): PersonSummary;
  id: string;
  readonly links: Map<any, any>;
  name: string;
  get email(): string;
  set email(value: string | null | undefined);
  get orcid(): string;
  set orcid(value: string | null | undefined);
}
export class PersonSummaryWithParents {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): PersonSummaryWithParents;
  static from_json_string(json_str: string): PersonSummaryWithParents;
  static from_base64_json(base64_json_bytes: string): PersonSummaryWithParents;
  id_: string;
  summary: PersonSummary;
  institution: Institution;
}
export class ResourceNotFoundError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ResourceNotFoundError;
  static from_json_string(json_str: string): ResourceNotFoundError;
  static from_base64_json(base64_json_bytes: string): ResourceNotFoundError;
  requested_resource_id: string;
}
export class ScamplersClient {
  free(): void;
  constructor(api_base_url: string, frontend_token?: string | null, api_key?: string | null);
  list_institutions(data: InstitutionQuery): Promise<Institution[]>;
  ms_login(data: NewPerson): Promise<CreatedUser>;
  list_people(data: PersonQuery): Promise<Person[]>;
  create_lab(data: NewLab): Promise<Lab>;
  list_labs(data: LabQuery): Promise<Lab[]>;
  update_lab(data: LabUpdate): Promise<Lab>;
  list_specimens(data: SpecimenQuery): Promise<Specimen[]>;
  list_sequencing_runs(data: SequencingRunQuery): Promise<SequencingRun[]>;
  list_suspensions(data: SuspensionQuery): Promise<Suspension[]>;
  list_suspension_pools(data: SuspensionPoolQuery): Promise<SuspensionPool[]>;
  list_chromium_runs(data: ChromiumRunQuery): Promise<ChromiumRun[]>;
  list_cdna(data: CdnaQuery): Promise<Cdna[]>;
  list_libraries(data: LibraryQuery): Promise<Library[]>;
  list_chromium_datasets(data: ChromiumDatasetQuery): Promise<ChromiumDataset[]>;
  fetch_institution(data: string): Promise<Institution>;
  fetch_person(data: string): Promise<Person>;
  fetch_lab(data: string): Promise<Lab>;
  fetch_specimen(data: string): Promise<Specimen>;
  fetch_sequencing_run(data: string): Promise<SequencingRun>;
  fetch_suspension(data: string): Promise<Suspension>;
  fetch_suspension_pool(data: string): Promise<SuspensionPool>;
  fetch_chromium_run(data: string): Promise<ChromiumRun>;
  fetch_cdna(data: string): Promise<Cdna>;
  fetch_library(data: string): Promise<Library>;
  fetch_chromium_dataset(data: string): Promise<ChromiumDataset>;
  list_person_specimens(id: string, query: SpecimenQuery): Promise<Specimen[]>;
}
export class ScamplersErrorResponse {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ScamplersErrorResponse;
  static from_json_string(json_str: string): ScamplersErrorResponse;
  static from_base64_json(base64_json_bytes: string): ScamplersErrorResponse;
  get status(): number | undefined;
  set status(value: number | null | undefined);
  readonly error: any;
}
export class SequencingRun {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SequencingRun;
  static from_json_string(json_str: string): SequencingRun;
  static from_base64_json(base64_json_bytes: string): SequencingRun;
  summary: SequencingRunSummary;
  libraries: string[];
}
export class SequencingRunQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SequencingRunQuery;
  static from_json_string(json_str: string): SequencingRunQuery;
  static from_base64_json(base64_json_bytes: string): SequencingRunQuery;
  constructor();
  ids: string[];
  order_by: OrderBy[];
  pagination: Pagination;
}
export class SequencingRunSummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SequencingRunSummary;
  static from_json_string(json_str: string): SequencingRunSummary;
  static from_base64_json(base64_json_bytes: string): SequencingRunSummary;
  id: string;
  readonly links: Map<any, any>;
  readable_id: string;
  begun_at: Date;
  get finished_at(): Date | undefined;
  set finished_at(value: Date | null | undefined);
  get additional_data(): any | undefined;
  set additional_data(value: any | null | undefined);
}
export class ServerError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): ServerError;
  static from_json_string(json_str: string): ServerError;
  static from_base64_json(base64_json_bytes: string): ServerError;
  message: string;
  raw_response_body: string;
}
export class Specimen {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): Specimen;
  static from_json_string(json_str: string): Specimen;
  static from_base64_json(base64_json_bytes: string): Specimen;
  info: SpecimenSummaryWithParents;
  measurements: SpecimenMeasurement[];
}
export class SpecimenMeasurement {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SpecimenMeasurement;
  static from_json_string(json_str: string): SpecimenMeasurement;
  static from_base64_json(base64_json_bytes: string): SpecimenMeasurement;
  id: string;
  specimen_id: string;
  measured_by: string;
}
export class SpecimenQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SpecimenQuery;
  static from_json_string(json_str: string): SpecimenQuery;
  static from_base64_json(base64_json_bytes: string): SpecimenQuery;
  constructor();
  ids: string[];
  names: string[];
  submitters: string[];
  labs: string[];
  get received_before(): Date | undefined;
  set received_before(value: Date | null | undefined);
  get received_after(): Date | undefined;
  set received_after(value: Date | null | undefined);
  species: any[];
  types: any[];
  get frozen(): boolean | undefined;
  set frozen(value: boolean | null | undefined);
  get cryopreserved(): boolean | undefined;
  set cryopreserved(value: boolean | null | undefined);
  tissues: string[];
  additional_data: any[];
  order_by: OrderBy[];
  pagination: Pagination;
}
export class SpecimenSummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SpecimenSummary;
  static from_json_string(json_str: string): SpecimenSummary;
  static from_base64_json(base64_json_bytes: string): SpecimenSummary;
  id: string;
  readonly links: Map<any, any>;
  readable_id: string;
  name: string;
  received_at: Date;
  get returned_at(): Date | undefined;
  set returned_at(value: Date | null | undefined);
  type_: SpecimenType;
  get embedded_in(): string;
  set embedded_in(value: string | null | undefined);
  get fixative(): string;
  set fixative(value: string | null | undefined);
  frozen: boolean;
  cryopreserved: boolean;
  tissue: string;
  get additional_data(): any | undefined;
  set additional_data(value: any | null | undefined);
  readonly species: string[];
}
export class SpecimenSummaryWithParents {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SpecimenSummaryWithParents;
  static from_json_string(json_str: string): SpecimenSummaryWithParents;
  static from_base64_json(base64_json_bytes: string): SpecimenSummaryWithParents;
  id_: string;
  summary: SpecimenSummary;
  lab: LabSummary;
  submitted_by: PersonSummary;
}
export class Suspension {
  private constructor();
  free(): void;
  info: SuspensionSummaryWithParents;
  prepared_by: string[];
  measurements: SuspensionMeasurement[];
}
export class SuspensionMeasurement {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SuspensionMeasurement;
  static from_json_string(json_str: string): SuspensionMeasurement;
  static from_base64_json(base64_json_bytes: string): SuspensionMeasurement;
  id: string;
  measured_by: string;
  suspension_id: string;
}
export class SuspensionPool {
  private constructor();
  free(): void;
  summary: SuspensionPoolSummary;
  suspensions: SuspensionSummary[];
  preparers: string[];
  measurements: SuspensionPoolMeasurement[];
}
export class SuspensionPoolMeasurement {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SuspensionPoolMeasurement;
  static from_json_string(json_str: string): SuspensionPoolMeasurement;
  static from_base64_json(base64_json_bytes: string): SuspensionPoolMeasurement;
  id: string;
  pool_id: string;
  measured_by: string;
}
export class SuspensionPoolQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SuspensionPoolQuery;
  static from_json_string(json_str: string): SuspensionPoolQuery;
  static from_base64_json(base64_json_bytes: string): SuspensionPoolQuery;
  constructor();
  ids: string[];
  order_by: OrderBy[];
  pagination: Pagination;
}
export class SuspensionPoolSummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SuspensionPoolSummary;
  static from_json_string(json_str: string): SuspensionPoolSummary;
  static from_base64_json(base64_json_bytes: string): SuspensionPoolSummary;
  id: string;
  readonly links: Map<any, any>;
  readable_id: string;
  name: string;
  pooled_at: Date;
  get additional_data(): any | undefined;
  set additional_data(value: any | null | undefined);
}
export class SuspensionQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SuspensionQuery;
  static from_json_string(json_str: string): SuspensionQuery;
  static from_base64_json(base64_json_bytes: string): SuspensionQuery;
  constructor();
  ids: string[];
  get parent_specimen(): SpecimenQuery | undefined;
  set parent_specimen(value: SpecimenQuery | null | undefined);
  order_by: OrderBy[];
  pagination: Pagination;
}
export class SuspensionSummary {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SuspensionSummary;
  static from_json_string(json_str: string): SuspensionSummary;
  static from_base64_json(base64_json_bytes: string): SuspensionSummary;
  id: string;
  readonly links: Map<any, any>;
  readable_id: string;
  biological_material: string;
  get pooled_into(): string;
  set pooled_into(value: string | null | undefined);
  get created_at(): Date | undefined;
  set created_at(value: Date | null | undefined);
  get lysis_duration_minutes(): number | undefined;
  set lysis_duration_minutes(value: number | null | undefined);
  target_cell_recovery: number;
  get additional_data(): any | undefined;
  set additional_data(value: any | null | undefined);
}
export class SuspensionSummaryWithParents {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SuspensionSummaryWithParents;
  static from_json_string(json_str: string): SuspensionSummaryWithParents;
  static from_base64_json(base64_json_bytes: string): SuspensionSummaryWithParents;
  id_: string;
  summary: SuspensionSummary;
  parent_specimen: SpecimenSummary;
  get multiplexing_tag(): MultiplexingTag | undefined;
  set multiplexing_tag(value: MultiplexingTag | null | undefined);
}
export class TenxAssay {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): TenxAssay;
  static from_json_string(json_str: string): TenxAssay;
  static from_base64_json(base64_json_bytes: string): TenxAssay;
  id: string;
  readonly links: Map<any, any>;
  name: string;
  get sample_multiplexing(): SampleMultiplexing | undefined;
  set sample_multiplexing(value: SampleMultiplexing | null | undefined);
  chemistry_version: string;
  protocol_url: string;
  get chromium_chip(): string;
  set chromium_chip(value: string | null | undefined);
}
export class TenxAssayQuery {
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): TenxAssayQuery;
  static from_json_string(json_str: string): TenxAssayQuery;
  static from_base64_json(base64_json_bytes: string): TenxAssayQuery;
  constructor();
  ids: string[];
  names: string[];
  sample_multiplexing: any[];
  chemistry_versions: string[];
  chromium_chips: string[];
  order_by: OrderBy[];
  pagination: Pagination;
}
