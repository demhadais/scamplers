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
export enum UserRole {
  AppAdmin = 0,
  BiologyStaff = 1,
  ComputationalStaff = 2,
}
export class CdnaGemsError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
}
export class CdnaHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class CdnaLibraryTypeError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  expected_library_types: any[];
  found_library_types: any[];
}
export class ChromiumRun {
  private constructor();
  free(): void;
  summary: ChromiumRunSummary;
  gems: GemsHandle[];
}
export class ChromiumRunHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class ChromiumRunSummary {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: ChromiumRunHandle;
  readable_id: string;
  chip: string;
  run_at: Date;
  succeeded: boolean;
  get notes(): string;
  set notes(value: string | null | undefined);
}
export class ClientError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
}
export class CommitteeApproval {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  institution: InstitutionHandle;
  committee_type: string;
  compliance_identifier: string;
}
export class CreatedUser {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  person: Person;
  api_key: string;
}
export class DatasetCmdlineError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  get chemistry(): string;
  set chemistry(value: string | null | undefined);
  expected_cmdline: string;
  found_cmdline: string;
}
export class DatasetHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class DatasetMetricsFileParseError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
}
export class DatasetNMetricsFilesError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  expected_n_metrics_files: bigint;
  found_n_metrics_files: bigint;
}
export class DatasetSummary {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: DatasetHandle;
  data_path: string;
  delivered_at: Date;
  get web_summary(): string;
  set web_summary(value: string | null | undefined);
}
export class DuplicateResourceError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  entity: string;
  fields: string[];
  values: string[];
}
export class EmptyStringError {
  private constructor();
  free(): void;
}
export class GemsHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class Institution {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: InstitutionHandle;
  name: string;
}
export class InstitutionHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class InstitutionQuery {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  constructor();
  ids: string[];
  get name(): string;
  set name(value: string | null | undefined);
  pagination: Pagination;
}
export class InvalidDataError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
}
export class InvalidMeasurementError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
}
export class InvalidReferenceError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  entity: string;
  referenced_entity: string;
  get value(): string;
  set value(value: string | null | undefined);
}
export class Lab {
  private constructor();
  free(): void;
  core: LabCore;
  members: PersonSummary[];
}
export class LabCore {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  summary: LabSummary;
  pi: PersonSummary;
}
export class LabHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class LabQuery {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  constructor();
  ids: string[];
  get name(): string;
  set name(value: string | null | undefined);
  pagination: Pagination;
}
export class LabSummary {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: LabHandle;
  name: string;
  delivery_dir: string;
}
export class LibraryHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class LibraryIndexSetError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
}
export class MalformedRequestError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
}
export class MultiplexingTag {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  tag_id: string;
  type_: string;
}
export class NewMsLogin {
  private constructor();
  free(): void;
  static new(): NewPersonEmpty;
}
export class NewPersonEmail {
  private constructor();
  free(): void;
  ms_user_id(ms_user_id: string): NewPersonMsUserId;
}
export class NewPersonEmpty {
  private constructor();
  free(): void;
  name(name: string): NewPersonName;
}
export class NewPersonInstitutionId {
  private constructor();
  free(): void;
  build(): NewMsLogin;
}
export class NewPersonMsUserId {
  private constructor();
  free(): void;
  institution_id(institution_id: string): NewPersonInstitutionId;
}
export class NewPersonName {
  private constructor();
  free(): void;
  email(email: string): NewPersonEmail;
}
export class Pagination {
  private constructor();
  free(): void;
  limit: bigint;
  offset: bigint;
}
export class PermissionDeniedError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
}
export class Person {
  private constructor();
  free(): void;
  core: PersonCore;
  roles: any[];
}
export class PersonCore {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  summary: PersonSummary;
  institution: Institution;
}
export class PersonHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class PersonQuery {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  constructor();
  ids: string[];
  get name(): string;
  set name(value: string | null | undefined);
  get email(): string;
  set email(value: string | null | undefined);
  get orcid(): string;
  set orcid(value: string | null | undefined);
  get ms_user_id(): string;
  set ms_user_id(value: string | null | undefined);
  pagination: Pagination;
}
export class PersonSummary {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: PersonHandle;
  name: string;
  get email(): string;
  set email(value: string | null | undefined);
  get orcid(): string;
  set orcid(value: string | null | undefined);
}
export class ResourceNotFoundError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  requested_resource_id: string;
}
export class ScamplersClient {
  free(): void;
  constructor(backend_base_url: string, frontend_token?: string | null, api_key?: string | null);
  ms_login(data: NewMsLogin): Promise<CreatedUser>;
}
export class ScamplersCoreErrorResponse {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  get status(): number | undefined;
  set status(value: number | null | undefined);
  readonly error: any;
}
export class SequencingRunHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class SequencingRunSummary {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: SequencingRunHandle;
  readable_id: string;
  begun_at: Date;
  finished_at: Date;
  get notes(): string;
  set notes(value: string | null | undefined);
}
export class ServerError {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  message: string;
  raw_response_body: string;
}
export class Specimen {
  private constructor();
  free(): void;
  core: SpecimenCore;
  measurements: SpecimenMeasurement[];
}
export class SpecimenCore {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  summary: SpecimenSummary;
  lab: LabSummary;
  submitted_by: PersonSummary;
}
export class SpecimenHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class SpecimenMeasurement {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  measured_by: PersonHandle;
}
export class SpecimenQuery {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  constructor();
  ids: string[];
  get name(): string;
  set name(value: string | null | undefined);
  submitters: string[];
  labs: string[];
  get received_before(): Date | undefined;
  set received_before(value: Date | null | undefined);
  get received_after(): Date | undefined;
  set received_after(value: Date | null | undefined);
  species: any[];
  get notes(): string;
  set notes(value: string | null | undefined);
  get storage_buffer(): string;
  set storage_buffer(value: string | null | undefined);
  get frozen(): boolean | undefined;
  set frozen(value: boolean | null | undefined);
  get cryopreserved(): boolean | undefined;
  set cryopreserved(value: boolean | null | undefined);
  pagination: Pagination;
}
export class SpecimenSummary {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: SpecimenHandle;
  readable_id: string;
  name: string;
  received_at: Date;
  get notes(): string;
  set notes(value: string | null | undefined);
  get returned_at(): Date | undefined;
  set returned_at(value: Date | null | undefined);
  type_: string;
  get embedded_in(): string;
  set embedded_in(value: string | null | undefined);
  get fixative(): string;
  set fixative(value: string | null | undefined);
  frozen: boolean;
  cryopreserved: boolean;
  get storage_buffer(): string;
  set storage_buffer(value: string | null | undefined);
}
export class SuspensionCore {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  summary: SuspensionSummary;
  parent_specimen: SpecimenSummary;
  multiplexing_tag: MultiplexingTag;
}
export class SuspensionHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class SuspensionMeasurement {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  measured_by: PersonHandle;
}
export class SuspensionPoolHandle {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  id: string;
  link: string;
}
export class SuspensionPoolSummary {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: SuspensionPoolHandle;
  readable_id: string;
  pooled_at: Date;
}
export class SuspensionSummary {
  private constructor();
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
  handle: SuspensionHandle;
  readable_id: string;
  biological_material: string;
  get created_at(): Date | undefined;
  set created_at(value: Date | null | undefined);
  get lysis_duration_minutes(): number | undefined;
  set lysis_duration_minutes(value: number | null | undefined);
  target_cell_recovery: number;
  target_reads_per_cell: number;
  get notes(): string;
  set notes(value: string | null | undefined);
}
