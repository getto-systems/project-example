import { authProfileRepositoryConverter } from "./convert"

test("authProfileRepositoryConverter", () => {
    const authAt = new Date("2020-01-01 10:00:00").toISOString()
    const roles = ["admin"]

    const result = authProfileRepositoryConverter.fromRepository({ authAt, roles })
    if (!result.valid) {
        throw new Error("convert failed")
    }

    const value = authProfileRepositoryConverter.toRepository(result.value)
    expect(value).toEqual({ authAt, roles })
})
