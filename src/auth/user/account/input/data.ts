export const grantedRoles = ["user"] as const
export type GrantedRole = typeof grantedRoles[number]

export type ResetTokenDestination =
    | Readonly<{ type: "none" }>
    | Readonly<{ type: "email"; email: ResetTokenDestinationEmail }>
export type ResetTokenDestinationEmail = string & { ResetTokenDestinationEmail: never }

export type ValidateResetTokenDestinationError =
    | Readonly<{ type: "empty-email" }>
    | Readonly<{ type: "invalid-email" }>
    | Readonly<{ type: "too-long-email"; maxLength: number }>
