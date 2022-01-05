export type AuthTicket = Readonly<{
    authAt: Date
    roles: string[]
}> & { AuthProfile: never }
