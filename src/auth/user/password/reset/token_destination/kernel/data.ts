export type ResetTokenDestination =
    | Readonly<{ type: "none" }>
    | Readonly<{ type: "email"; email: ResetTokenDestinationEmail }>
export type ResetTokenDestinationEmail = string & { ResetTokenDestinationEmail: never }
