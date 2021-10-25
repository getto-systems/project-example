// TODO UserAccountBasket
export type UserAccount = Readonly<{
    // TODO LoginID / GrantedRoles は機能を持っているので、data transfer object 的な何かにするべき
    loginID: string // TODO LoginIDBasket
    grantedRoles: string[] // TODO GrantedRolesBasket
}>
