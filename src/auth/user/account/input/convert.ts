import { GrantedAuthRole, allGrantedAuthRoles } from "./data"

export function toGrantedRoles(roles: readonly string[]): readonly GrantedAuthRole[] {
    const converted: GrantedAuthRole[] = []

    allGrantedAuthRoles.forEach((role) => {
        if (roles.includes(role)) {
            converted.push(role)
        }
    })

    return converted
}
