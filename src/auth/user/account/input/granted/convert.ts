import { ALL_AUTH_PERMISSIONS } from "../../../../../x_content/permission"

import { AuthPermission } from "../../../kernel/data"

export function toGranted(permissions: readonly string[]): readonly AuthPermission[] {
    const converted: AuthPermission[] = []

    ALL_AUTH_PERMISSIONS.forEach((permission) => {
        if (permissions.includes(permission)) {
            converted.push(permission)
        }
    })

    return converted
}
