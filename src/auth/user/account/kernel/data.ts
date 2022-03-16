export type AuthUserAccountBasket = Readonly<{
    loginId: string
    grantedRoles: readonly string[]
    resetTokenDestination: { type: "none" } | { type: "email", email: string }
}>
