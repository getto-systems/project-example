export const ALL_AUTH_PERMISSIONS = ["auth-user"] as const

export function authPermissionLabel(permission: typeof ALL_AUTH_PERMISSIONS[number]): string {
    switch (permission) {
        case "auth-user":
            return "ユーザー管理"
    }
}
