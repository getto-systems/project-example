export enum InputBoardValueTypeEnum {
    "text",
    "password",
    "search",
    "number",
    "tel",
    "email",
    "date",
    "time",
}
export type InputBoardValueType = keyof typeof InputBoardValueTypeEnum
