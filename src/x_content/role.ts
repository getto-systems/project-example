export const allAuthRoles = ["user"] as const

export function authRoleLabel(role: typeof allAuthRoles[number]): string {
    switch (role) {
        case "user":
            return "ユーザー管理"
    }
}
