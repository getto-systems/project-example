import { ALL_AUTH_ROLES } from "../../../../x_content/role"
import { AuthRole } from "../../kernel/data"

export function toGrantedRoles(roles: readonly string[]): readonly AuthRole[] {
    const converted: AuthRole[] = []

    ALL_AUTH_ROLES.forEach((role) => {
        if (roles.includes(role)) {
            converted.push(role)
        }
    })

    return converted
}
