import { allAuthRoles } from "../../../../x_content/role"
import { GrantedAuthRole } from "../../kernel/data"

export function toGrantedRoles(roles: readonly string[]): readonly GrantedAuthRole[] {
    const converted: GrantedAuthRole[] = []

    allAuthRoles.forEach((role) => {
        if (roles.includes(role)) {
            converted.push(role)
        }
    })

    return converted
}
