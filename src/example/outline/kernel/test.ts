import { menuExpandRepositoryConverter } from "./convert"

test("menuExpandRepositoryConverter", () => {
    const expand = [["MENU"]]

    const result = menuExpandRepositoryConverter.fromRepository(expand)
    if (!result.valid) {
        throw new Error("convert failed")
    }

    const value = menuExpandRepositoryConverter.toRepository(result.value)
    expect(value).toEqual(expand)
})
