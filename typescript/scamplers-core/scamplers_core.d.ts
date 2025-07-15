/* tslint:disable */
/* eslint-disable */
export enum Species {
  AmbystomaMexicanum = 0,
  CanisFamiliaris = 1,
  DrosophilaMelanogaster = 2,
  GasterosteusAculeatus = 3,
  HomoSapiens = 4,
  MusMusculus = 5,
  RattusNorvegicus = 6,
  SminthopsisCrassicaudata = 7,
}
export enum UserRole {
  AppAdmin = 0,
  ComputationalStaff = 1,
  BiologyStaff = 2,
}
export class Client {
  free(): void;
  constructor(backend_base_url: string, token: string, api_key?: string | null);
  ms_login(data: NewMsLogin): Promise<CreatedUser>;
}
export class CommitteeApproval {
  private constructor();
  free(): void;
  institution: InstitutionHandle;
  committee_type: string;
  compliance_identifier: string;
}
export class CreatedUser {
  private constructor();
  free(): void;
  person: Person;
  api_key: string;
  readonly id: string;
  readonly link: string;
  readonly name: string;
  readonly email: string;
  readonly orcid: string;
  readonly roles: any[];
  readonly institution: Institution;
}
export class EmptyStringError {
  private constructor();
  free(): void;
}
export class Institution {
  private constructor();
  free(): void;
  handle: InstitutionHandle;
  name: string;
  readonly id: string;
  readonly link: string;
}
export class InstitutionHandle {
  private constructor();
  free(): void;
  id: string;
  link: string;
}
export class InstitutionQuery {
  free(): void;
  constructor();
  ids: string[];
  get name(): string;
  set name(value: string | null | undefined);
  pagination: Pagination;
}
export class Lab {
  private constructor();
  free(): void;
  core: LabCore;
  members: PersonSummary[];
  readonly id: string;
}
export class LabCore {
  private constructor();
  free(): void;
  summary: LabSummary;
  pi: PersonSummary;
}
export class LabHandle {
  private constructor();
  free(): void;
  id: string;
  link: string;
}
export class LabQuery {
  free(): void;
  constructor();
  ids: string[];
  get name(): string;
  set name(value: string | null | undefined);
  pagination: Pagination;
}
export class LabSummary {
  private constructor();
  free(): void;
  handle: LabHandle;
  name: string;
  delivery_dir: string;
  readonly id: string;
  readonly link: string;
}
export class MultiplexingTag {
  private constructor();
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
export class Person {
  private constructor();
  free(): void;
  core: PersonCore;
  roles: any[];
  readonly id: string;
  readonly link: string;
  readonly name: string;
  readonly email: string;
  readonly orcid: string;
  readonly institution: Institution;
}
export class PersonCore {
  private constructor();
  free(): void;
  summary: PersonSummary;
  institution: Institution;
}
export class PersonHandle {
  private constructor();
  free(): void;
  id: string;
  link: string;
}
export class PersonQuery {
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
  free(): void;
  handle: PersonHandle;
  name: string;
  get email(): string;
  set email(value: string | null | undefined);
  get orcid(): string;
  set orcid(value: string | null | undefined);
  readonly id: string;
  readonly link: string;
}
export class SequencingRunHandle {
  private constructor();
  free(): void;
  id: string;
  link: string;
}
export class SequencingRunSummary {
  private constructor();
  free(): void;
  handle: SequencingRunHandle;
  readable_id: string;
  begun_at: Date;
  finished_at: Date;
  get notes(): string;
  set notes(value: string | null | undefined);
}
export class Specimen {
  private constructor();
  free(): void;
  core: SpecimenCore;
  measurements: SpecimenMeasurement[];
}
export class SpecimenCore {
  private constructor();
  free(): void;
  summary: SpecimenSummary;
  lab: LabSummary;
  submitted_by: PersonSummary;
  returned_by: PersonSummary;
}
export class SpecimenHandle {
  private constructor();
  free(): void;
  id: string;
  link: string;
}
export class SpecimenMeasurement {
  private constructor();
  free(): void;
  measured_by: PersonHandle;
}
export class SpecimenQuery {
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
  free(): void;
  summary: SuspensionSummary;
  parent_specimen: SpecimenSummary;
  multiplexing_tag: MultiplexingTag;
}
export class SuspensionHandle {
  private constructor();
  free(): void;
  id: string;
  link: string;
}
export class SuspensionMeasurement {
  private constructor();
  free(): void;
  measured_by: PersonHandle;
}
export class SuspensionPoolHandle {
  private constructor();
  free(): void;
  id: string;
  link: string;
}
export class SuspensionPoolSummary {
  private constructor();
  free(): void;
  handle: SuspensionPoolHandle;
  readable_id: string;
  pooled_at: Date;
}
export class SuspensionSummary {
  private constructor();
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
