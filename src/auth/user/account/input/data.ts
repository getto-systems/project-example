export const allGrantedAuthRoles = ["user"] as const
export type GrantedAuthRole = typeof allGrantedAuthRoles[number]
