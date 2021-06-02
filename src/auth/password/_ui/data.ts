export type Password = string & { Password: never }

export type PasswordCharacterState = Readonly<{ multiByte: boolean }>

export type ValidatePasswordError =
    | Readonly<{ type: "empty" }>
    | Readonly<{ type: "too-long"; maxBytes: number; multiByte: boolean }>
