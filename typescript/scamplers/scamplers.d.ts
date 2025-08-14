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
export enum UserRole {
  AppAdmin = 0,
  BiologyStaff = 1,
  ComputationalStaff = 2,
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
  expected_library_types: any[];
  found_library_types: any[];
  expected_volumes: Float32Array;
  found_volumes: Float32Array;
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
export class CreatedUser {
  private constructor();
  free(): void;
  person: Person;
  api_key: string;
}
export class DatasetCmdlineError {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): DatasetCmdlineError;
  static from_json_string(json_str: string): DatasetCmdlineError;
  static from_base64_json(base64_json_bytes: string): DatasetCmdlineError;
  get chemistry(): string;
  set chemistry(value: string | null | undefined);
  expected_cmdlines: string[];
  found_cmdline: string;
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
  get name(): string;
  set name(value: string | null | undefined);
  pagination: Pagination;
  readonly get_order_by: OrderBy[];
  set order_by(value: OrderBy[]);
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
  private constructor();
  free(): void;
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
  core: PersonCore;
  roles: any[];
}
export class PersonCore {
  private constructor();
  free(): void;
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): PersonCore;
  static from_json_string(json_str: string): PersonCore;
  static from_base64_json(base64_json_bytes: string): PersonCore;
  summary: PersonSummary;
  institution: Institution;
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
  get name(): string;
  set name(value: string | null | undefined);
  get email(): string;
  set email(value: string | null | undefined);
  get orcid(): string;
  set orcid(value: string | null | undefined);
  get ms_user_id(): string;
  set ms_user_id(value: string | null | undefined);
  pagination: Pagination;
  readonly get_order_by: OrderBy[];
  set order_by(value: OrderBy[]);
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
  constructor(api_base_url: string, frontend_token?: string | null, api_key?: string | null, accept_invalid_certificates?: boolean | null);
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
  to_json_bytes(): Uint8Array;
  to_json_string(): string;
  to_base64_json(): string;
  static from_json_bytes(json_bytes: Uint8Array): SuspensionCore;
  static from_json_string(json_str: string): SuspensionCore;
  static from_base64_json(base64_json_bytes: string): SuspensionCore;
  summary: SuspensionSummary;
  parent_specimen: SpecimenSummary;
  multiplexing_tag: MultiplexingTag;
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
  measured_by: PersonSummary;
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
  get created_at(): Date | undefined;
  set created_at(value: Date | null | undefined);
  get lysis_duration_minutes(): number | undefined;
  set lysis_duration_minutes(value: number | null | undefined);
  target_cell_recovery: number;
  target_reads_per_cell: number;
  get notes(): string;
  set notes(value: string | null | undefined);
}
