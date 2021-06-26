export const inputBoardValueTypes = [
    "text",
    "password",
    "search",
    "number",
    "tel",
    "email",
    "date",
    "time",
] as const
export type InputBoardValueType = typeof inputBoardValueTypes[number]
