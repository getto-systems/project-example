export const grantedRoles = ["user"] as const
export type GrantedRole = typeof grantedRoles[number]

export function isGrantedRole(role: string): role is GrantedRole {
    switch (role) {
        case "user":
            return true

        default:
            return false
    }
}

export type ResetTokenDestination =
    | Readonly<{ type: "none" }>
    | Readonly<{ type: "email"; email: ResetTokenDestinationEmail }>
export type ResetTokenDestinationEmail = string & { ResetTokenDestinationEmail: never }

export type ValidateResetTokenDestinationError =
    | Readonly<{ type: "empty-email" }>
    | Readonly<{ type: "invalid-email" }>
    | Readonly<{ type: "too-long-email"; maxLength: number }>
