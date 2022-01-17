export type AuthTicket = Readonly<{
    authAt: Date
    roles: readonly string[]
}> & { AuthProfile: never }
