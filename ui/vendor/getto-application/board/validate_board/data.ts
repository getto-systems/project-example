export enum ValidateBoardStateEnum {
    "initial",
    "valid",
    "invalid",
}

export type ValidateBoardState = keyof typeof ValidateBoardStateEnum
