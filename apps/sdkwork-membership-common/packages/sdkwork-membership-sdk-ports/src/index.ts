export const APP_MEMBERSHIP_METHOD_TREE = {
  memberships: {
    benefits: { list: true },
    current: {
      retrieve: true,
      status: { retrieve: true },
    },
    plans: { list: true },
    packageGroups: {
      list: true,
      retrieve: true,
      packages: { list: true },
    },
    packages: {
      list: true,
      retrieve: true,
    },
    purchases: {
      create: true,
      renew: true,
      upgrade: true,
    },
    points: {
      balance: { retrieve: true },
      history: { list: true },
      dailyRewards: {
        create: true,
        status: { retrieve: true },
      },
    },
    privileges: {
      usage: { retrieve: true },
      speedUps: { create: true },
    },
  },
} as const;

export type MembershipRequestParams = Record<string, unknown>;
export type MembershipSdkResponse<T> = Promise<
  T | { code?: number | string; data?: T; message?: string; msg?: string }
>;
export type MembershipSdkMethod = (...args: any[]) => MembershipSdkResponse<any>;

type MethodTree = {
  readonly [key: string]: true | MethodTree;
};

export type ClientFromMethodTree<TTree extends MethodTree> = {
  readonly [TKey in keyof TTree]: TTree[TKey] extends true
    ? MembershipSdkMethod
    : TTree[TKey] extends MethodTree
      ? ClientFromMethodTree<TTree[TKey]>
      : never;
};

export type MembershipAppSdkClient = {
  commerce: ClientFromMethodTree<typeof APP_MEMBERSHIP_METHOD_TREE>;
};
