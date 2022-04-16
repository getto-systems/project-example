import { allAuthRoles } from "../../../../x_content/role"
import { AuthRole } from "../../kernel/data"

export function toGrantedRoles(roles: readonly string[]): readonly AuthRole[] {
    const converted: AuthRole[] = []

    allAuthRoles.forEach((role) => {
        if (roles.includes(role)) {
            converted.push(role)
        }
    })

    return converted
}
