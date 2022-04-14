// TODO x_content に移動
export const allGrantedAuthRoles = ["user"] as const

// TODO kernel に移動
export type GrantedAuthRole = typeof allGrantedAuthRoles[number]
