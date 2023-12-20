import { ResetTokenDestination, ResetTokenDestinationEmail } from "./data"

export function restoreResetTokenDestination(
    data: Readonly<{ type: string; email: string }>,
): ResetTokenDestination {
    switch (data.type) {
        case "email":
            return { type: "email", email: restoreResetTokenDestinationEmail(data.email) }

        default:
            return { type: "none" }
    }
}

export function restoreResetTokenDestinationEmail(data: string): ResetTokenDestinationEmail {
    return data as ResetTokenDestinationEmail
}
