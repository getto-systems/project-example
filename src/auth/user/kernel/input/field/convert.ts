import { ALL_AUTH_PERMISSIONS } from "../../../../../x_content/permission"

import { AuthPermission } from "../../data"

export function restoreAuthPermission(value: string): [] | [AuthPermission] {
    if (ALL_AUTH_PERMISSIONS.some((permission) => permission === value)) {
        return [value as AuthPermission]
    } else {
        return []
    }
}

export function toGranted(permissions: readonly string[]): readonly AuthPermission[] {
    const converted: AuthPermission[] = []

    ALL_AUTH_PERMISSIONS.forEach((permission) => {
        if (permissions.includes(permission)) {
            converted.push(permission)
        }
    })

    return converted
}
