import { VersionString } from "../../data"

export interface GetCurrentVersionCoreAction {
    getCurrent(): VersionString
}
