import { appApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { AppMembershipDailyRewardResponse, AppMembershipDailyRewardStatusResponse, AppMembershipInfoResponse, AppMembershipPackageGroupItem, AppMembershipPackageItem, AppMembershipPointsBalanceResponse, AppMembershipPrivilegeUsageResponse, AppMembershipPurchaseOutcome, AppMembershipStatusResponse, CommerceOperationCommand, MembershipCategory, MembershipFeatureAccessCheckCommand, MembershipFeatureAccessCheckResult, SdkWorkCommandData, SdkWorkPageDataBenefits, SdkWorkPageDataPackageGroups, SdkWorkPageDataPackages, SdkWorkPageDataPlans, SdkWorkPageDataPointsHistory } from '../types';


export class MembershipsAccessChecksApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Check whether the current member level grants access to a feature. */
  async create(body: MembershipFeatureAccessCheckCommand, requestOptions?: ApiRequestOptions): Promise<MembershipFeatureAccessCheckResult> {
    return this.client.request<MembershipFeatureAccessCheckResult>(appApiPath(`/memberships/access/checks`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class MembershipsPrivilegesSpeedUpsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships privileges speed Ups create. */
  async create(body: CommerceOperationCommand, requestOptions?: ApiRequestOptions): Promise<SdkWorkCommandData> {
    return this.client.request<SdkWorkCommandData>(appApiPath(`/memberships/privileges/speed_ups`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class MembershipsPrivilegesUsageApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships privileges usage retrieve. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<AppMembershipPrivilegeUsageResponse> {
    return this.client.request<AppMembershipPrivilegeUsageResponse>(appApiPath(`/memberships/privileges/usage`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class MembershipsPrivilegesApi {
  private client: HttpClient;
  public readonly usage: MembershipsPrivilegesUsageApi;
  public readonly speedUps: MembershipsPrivilegesSpeedUpsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.usage = new MembershipsPrivilegesUsageApi(client);
    this.speedUps = new MembershipsPrivilegesSpeedUpsApi(client);
  }

}

export class MembershipsPointsDailyRewardsStatusApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships points daily Rewards status retrieve. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<AppMembershipDailyRewardStatusResponse> {
    return this.client.request<AppMembershipDailyRewardStatusResponse>(appApiPath(`/memberships/points/daily_rewards/status`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class MembershipsPointsDailyRewardsApi {
  private client: HttpClient;
  public readonly status: MembershipsPointsDailyRewardsStatusApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.status = new MembershipsPointsDailyRewardsStatusApi(client);
  }


/** Memberships points daily Rewards create. */
  async create(body: CommerceOperationCommand, requestOptions?: ApiRequestOptions): Promise<AppMembershipDailyRewardResponse> {
    return this.client.request<AppMembershipDailyRewardResponse>(appApiPath(`/memberships/points/daily_rewards`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface MembershipsPointsHistoryListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
}

export class MembershipsPointsHistoryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships points history list. */
  async list(params?: MembershipsPointsHistoryListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageDataPointsHistory> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageDataPointsHistory>(appendQueryString(appApiPath(`/memberships/points/history`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class MembershipsPointsBalanceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships points balance retrieve. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<AppMembershipPointsBalanceResponse> {
    return this.client.request<AppMembershipPointsBalanceResponse>(appApiPath(`/memberships/points/balance`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class MembershipsPointsApi {
  private client: HttpClient;
  public readonly balance: MembershipsPointsBalanceApi;
  public readonly history: MembershipsPointsHistoryApi;
  public readonly dailyRewards: MembershipsPointsDailyRewardsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.balance = new MembershipsPointsBalanceApi(client);
    this.history = new MembershipsPointsHistoryApi(client);
    this.dailyRewards = new MembershipsPointsDailyRewardsApi(client);
  }

}

export class MembershipsPurchasesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships purchases create. */
  async create(body: CommerceOperationCommand, requestOptions?: ApiRequestOptions): Promise<AppMembershipPurchaseOutcome> {
    return this.client.request<AppMembershipPurchaseOutcome>(appApiPath(`/memberships/purchases`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Memberships purchases renew. */
  async renew(body: CommerceOperationCommand, requestOptions?: ApiRequestOptions): Promise<AppMembershipPurchaseOutcome> {
    return this.client.request<AppMembershipPurchaseOutcome>(appApiPath(`/memberships/purchases/renew`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Memberships purchases upgrade. */
  async upgrade(body: CommerceOperationCommand, requestOptions?: ApiRequestOptions): Promise<AppMembershipPurchaseOutcome> {
    return this.client.request<AppMembershipPurchaseOutcome>(appApiPath(`/memberships/purchases/upgrade`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface MembershipsPackagesListParams {
  category?: MembershipCategory;
  status?: string;
  page?: number;
  pageSize?: number;
  packageGroupId?: string;
  planId?: string;
  cursor?: string;
}

export class MembershipsPackagesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships packages list. */
  async list(params?: MembershipsPackagesListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageDataPackages> {
    const query = buildQueryString([
      { name: 'category', value: params?.category, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'package_group_id', value: params?.packageGroupId, style: 'form', explode: true, allowReserved: false },
      { name: 'plan_id', value: params?.planId, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageDataPackages>(appendQueryString(appApiPath(`/memberships/packages`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, skipAuth: true, sdkworkUnwrapKind: 'page' });
  }

/** Memberships packages retrieve. */
  async retrieve(packageId: string, requestOptions?: ApiRequestOptions): Promise<AppMembershipPackageItem> {
    return this.client.request<AppMembershipPackageItem>(appApiPath(`/memberships/packages/${serializePathParameter(packageId, { name: 'packageId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, skipAuth: true, sdkworkUnwrapKind: 'item' });
  }
}

export interface MembershipsPackageGroupsPackagesListParams {
  status?: string;
  page?: number;
  pageSize?: number;
  planId?: string;
  recommendedOnly?: boolean;
  cursor?: string;
}

export class MembershipsPackageGroupsPackagesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships package Groups packages list. */
  async list(packageGroupId: string, params?: MembershipsPackageGroupsPackagesListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageDataPackages> {
    const query = buildQueryString([
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'plan_id', value: params?.planId, style: 'form', explode: true, allowReserved: false },
      { name: 'recommended_only', value: params?.recommendedOnly, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageDataPackages>(appendQueryString(appApiPath(`/memberships/package_groups/${serializePathParameter(packageGroupId, { name: 'packageGroupId', style: 'simple', explode: false })}/packages`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, skipAuth: true, sdkworkUnwrapKind: 'page' });
  }
}

export interface MembershipsPackageGroupsListParams {
  category?: MembershipCategory;
  status?: string;
  page?: number;
  pageSize?: number;
  planId?: string;
  recommendedOnly?: boolean;
  cursor?: string;
}

export class MembershipsPackageGroupsApi {
  private client: HttpClient;
  public readonly packages: MembershipsPackageGroupsPackagesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.packages = new MembershipsPackageGroupsPackagesApi(client);
  }


/** Memberships package Groups list. */
  async list(params?: MembershipsPackageGroupsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageDataPackageGroups> {
    const query = buildQueryString([
      { name: 'category', value: params?.category, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'plan_id', value: params?.planId, style: 'form', explode: true, allowReserved: false },
      { name: 'recommended_only', value: params?.recommendedOnly, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageDataPackageGroups>(appendQueryString(appApiPath(`/memberships/package_groups`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, skipAuth: true, sdkworkUnwrapKind: 'page' });
  }

/** Memberships package Groups retrieve. */
  async retrieve(packageGroupId: string, requestOptions?: ApiRequestOptions): Promise<AppMembershipPackageGroupItem> {
    return this.client.request<AppMembershipPackageGroupItem>(appApiPath(`/memberships/package_groups/${serializePathParameter(packageGroupId, { name: 'packageGroupId', style: 'simple', explode: false })}`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, skipAuth: true, sdkworkUnwrapKind: 'item' });
  }
}

export interface MembershipsPlansListParams {
  category?: MembershipCategory;
  status?: string;
  page?: number;
  pageSize?: number;
  cursor?: string;
}

export class MembershipsPlansApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships plans list. */
  async list(params?: MembershipsPlansListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageDataPlans> {
    const query = buildQueryString([
      { name: 'category', value: params?.category, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageDataPlans>(appendQueryString(appApiPath(`/memberships/plans`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, skipAuth: true, sdkworkUnwrapKind: 'page' });
  }
}

export interface MembershipsBenefitsListParams {
  page?: number;
  pageSize?: number;
  planId?: string;
  cursor?: string;
}

export class MembershipsBenefitsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships benefits list. */
  async list(params?: MembershipsBenefitsListParams, requestOptions?: ApiRequestOptions): Promise<SdkWorkPageDataBenefits> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'plan_id', value: params?.planId, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<SdkWorkPageDataBenefits>(appendQueryString(appApiPath(`/memberships/benefits`), query), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, skipAuth: true, sdkworkUnwrapKind: 'page' });
  }
}

export class MembershipsCurrentStatusApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Memberships current status retrieve. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<AppMembershipStatusResponse> {
    return this.client.request<AppMembershipStatusResponse>(appApiPath(`/memberships/current/status`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class MembershipsCurrentApi {
  private client: HttpClient;
  public readonly status: MembershipsCurrentStatusApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.status = new MembershipsCurrentStatusApi(client);
  }


/** Memberships current retrieve. */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<AppMembershipInfoResponse> {
    return this.client.request<AppMembershipInfoResponse>(appApiPath(`/memberships/current`), { signal: requestOptions?.signal, timeout: requestOptions?.timeout, method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class MembershipsApi {
  private client: HttpClient;
  public readonly current: MembershipsCurrentApi;
  public readonly benefits: MembershipsBenefitsApi;
  public readonly plans: MembershipsPlansApi;
  public readonly packageGroups: MembershipsPackageGroupsApi;
  public readonly packages: MembershipsPackagesApi;
  public readonly purchases: MembershipsPurchasesApi;
  public readonly points: MembershipsPointsApi;
  public readonly privileges: MembershipsPrivilegesApi;
  public readonly accessChecks: MembershipsAccessChecksApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.current = new MembershipsCurrentApi(client);
    this.benefits = new MembershipsBenefitsApi(client);
    this.plans = new MembershipsPlansApi(client);
    this.packageGroups = new MembershipsPackageGroupsApi(client);
    this.packages = new MembershipsPackagesApi(client);
    this.purchases = new MembershipsPurchasesApi(client);
    this.points = new MembershipsPointsApi(client);
    this.privileges = new MembershipsPrivilegesApi(client);
    this.accessChecks = new MembershipsAccessChecksApi(client);
  }

}

export function createMembershipsApi(client: HttpClient): MembershipsApi {
  return new MembershipsApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
