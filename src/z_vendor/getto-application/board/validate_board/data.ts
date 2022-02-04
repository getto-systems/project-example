export const validateBoardStates = ["initial", "valid", "invalid"] as const
export type ValidateBoardState = typeof validateBoardStates[number]
