export type LoginId = string & { LoginId: never }

export type ValidateLoginIdError =
    | Readonly<{ type: "empty" }>
    | Readonly<{ type: "too-long"; maxLength: number }>
