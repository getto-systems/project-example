export type ValidateResetTokenDestinationError =
    | Readonly<{ type: "invalid-type" }>
    | Readonly<{ type: "empty-email" }>
    | Readonly<{ type: "invalid-email" }>
    | Readonly<{ type: "too-long-email"; maxLength: number }>
