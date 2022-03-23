import { GrantedRole } from "../input/data"

export type AuthUserAccountBasket = Readonly<{
    loginId: string
    grantedRoles: readonly GrantedRole[]
    resetTokenDestination: { type: "none" } | { type: "email", email: string }
}>
