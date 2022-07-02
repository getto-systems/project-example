import { StaticMenuPath } from "../../y_environment/ui/path"

export type DecodeOutlineMenuBadgePathResult =
    | Readonly<{ success: false }>
    | Readonly<{ success: true; path: StaticMenuPath }>

export function decodeOutlineMenuBadgePath(path: string): DecodeOutlineMenuBadgePathResult {
    switch (path) {
        case "index":
            return { success: true, path: "index.html" }

        default:
            return { success: false }
    }
}
