import { seasonRepositoryConverter } from "./converter"

test("seasonRepositoryConverter", () => {
    const year = 2021

    const result = seasonRepositoryConverter.fromRepository({ year })
    if (!result.valid) {
        throw new Error("convert failed")
    }

    const value = seasonRepositoryConverter.toRepository(result.value)
    expect(value).toEqual({ year })
})
