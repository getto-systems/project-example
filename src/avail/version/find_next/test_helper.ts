import { ApplicationTargetPath } from "./data"

export function standardApplicationTargetPath(path: string): ApplicationTargetPath {
    return { path, specified: false } as ApplicationTargetPath
}
