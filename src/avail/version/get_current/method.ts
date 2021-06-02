import { VersionString } from "../data";

export interface GetCurrentVersionMethod {
    (): VersionString
}
