import test from "ava";

import { one, many } from "../index.js";

test("xid from native", (t) => {
	t.is(typeof one(), "string");

	t.is(Array.isArray(many(10)), true);
	t.is(many(10).length === 10, true);
});
