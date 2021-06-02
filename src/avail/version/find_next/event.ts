import { VersionString } from "../data"
import { ApplicationTargetPath, CheckDeployExistsError } from "./data"
import { ConvertLocationResult } from "../../../../ui/vendor/getto-application/location/data"

export type FindNextVersionEvent =
    | Readonly<{ type: "take-longtime-to-find" }>
    | Readonly<{ type: "failed-to-find"; err: CheckDeployExistsError }>
    | Readonly<{
          type: "succeed-to-find"
          upToDate: boolean
          version: VersionString
          target: ConvertLocationResult<ApplicationTargetPath>
      }>
