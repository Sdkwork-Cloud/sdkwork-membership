import assert from "node:assert/strict";
import test from "node:test";

import {
  createSdkworkMembershipListQuery,
  unwrapSdkworkMembershipPageItems,
} from "../../apps/sdkwork-membership-common/packages/sdkwork-membership-service/src/list-envelope.ts";

test("unwrapSdkworkMembershipPageItems extracts items from SdkWorkPageData", () => {
  const payload = {
    code: 0,
    traceId: "trace-1",
    data: {
      items: [{ id: 1, name: "Basic" }],
      pageInfo: {
        mode: "offset",
        page: 1,
        pageSize: 20,
        hasMore: false,
      },
    },
  };

  const items = unwrapSdkworkMembershipPageItems(payload);
  assert.equal(items.length, 1);
  assert.equal(items[0]?.name, "Basic");
});

test("createSdkworkMembershipListQuery defaults page size to 20", () => {
  assert.deepEqual(createSdkworkMembershipListQuery(), {
    page: 1,
    page_size: 20,
  });
});
