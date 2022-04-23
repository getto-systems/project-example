export const ALL_AUTH_ROLES = ["user"] as const

export function authRoleLabel(role: typeof ALL_AUTH_ROLES[number]): string {
    switch (role) {
        case "user":
            return "ユーザー管理"
    }
}
