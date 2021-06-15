import { authnRepositoryConverter } from "./converter"

test("authnRepositoryConverter", () => {
    const authAt = new Date("2020-01-01 10:00:00").toISOString()

    const result = authnRepositoryConverter.fromRepository({ authAt })
    if (!result.valid) {
        throw new Error("convert failed")
    }

    const value = authnRepositoryConverter.toRepository(result.value)
    expect(value).toEqual({ authAt })
})
