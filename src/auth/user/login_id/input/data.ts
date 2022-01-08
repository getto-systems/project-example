export type LoginID = string & { LoginID: never }

export type ValidateLoginIDError =
    | Readonly<{ type: "empty" }>
    | Readonly<{ type: "too-long"; maxLength: number }>
