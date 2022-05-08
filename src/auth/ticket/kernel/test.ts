import { test, expect } from "vitest"
import { authTicketRepositoryConverter } from "./convert"

test("authTicketRepositoryConverter", () => {
    const authAt = new Date("2020-01-01 10:00:00").toISOString()
    const grantedRoles = ["auth-user"]

    const result = authTicketRepositoryConverter.fromRepository({ authAt, grantedRoles })
    if (!result.valid) {
        throw new Error("convert failed")
    }

    const value = authTicketRepositoryConverter.toRepository(result.value)
    expect(value).toEqual({ authAt, grantedRoles })
})
