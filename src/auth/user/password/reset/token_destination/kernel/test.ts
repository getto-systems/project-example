import { test, expect } from "vitest"

import { restoreResetTokenDestination } from "./convert"

test("restore reset-token destination", () => {
    expect(restoreResetTokenDestination({ type: "email", email: "valid@example.com" })).toEqual({
        type: "email",
        email: "valid@example.com",
    })
})
