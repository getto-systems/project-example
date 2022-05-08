export const ALL_AUTH_ROLES = ["auth-user"] as const

export function authRoleLabel(role: typeof ALL_AUTH_ROLES[number]): string {
    switch (role) {
        case "auth-user":
            return "ユーザー管理"
    }
}
