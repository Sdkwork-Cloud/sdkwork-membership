import assert from "node:assert/strict";
import test from "node:test";

import {
  createSdkworkMembershipListQuery,
  unwrapSdkworkMembershipPageItems,
  unwrapSdkworkMembershipResponse,
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

test("createSdkworkMembershipListQuery defaults to token category and page size 20", () => {
  assert.deepEqual(createSdkworkMembershipListQuery(), {
    page: 1,
    pageSize: 20,
    category: "token",
  });
  // Token Plan queries always transmit the type parameter; community is opt-in.
  assert.deepEqual(createSdkworkMembershipListQuery(1, 20, "community"), {
    page: 1,
    pageSize: 20,
    category: "community",
  });
});

test("unwrapSdkworkMembershipResponse rejects legacy string-code envelopes", () => {
  assert.throws(
    () =>
      unwrapSdkworkMembershipResponse({
        code: "0",
        data: { id: "membership-1" },
        traceId: "trace-legacy",
      }),
    /Invalid SDKWork membership response envelope/u,
  );
});

test("unwrapSdkworkMembershipResponse does not unwrap objects that only contain data", () => {
  const payload = {
    data: { id: "domain-object" },
  };

  assert.equal(unwrapSdkworkMembershipResponse(payload), payload);
});
