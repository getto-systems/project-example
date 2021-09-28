import { VersionString } from "../kernel/data"
import { ApplicationTargetPath, CheckDeployExistsError } from "./data"
import { ConvertLocationResult } from "../../../z_lib/ui/location/data"

export type FindNextVersionEvent =
    | Readonly<{ type: "take-longtime-to-find" }>
    | Readonly<{ type: "failed-to-find"; err: CheckDeployExistsError }>
    | Readonly<{
          type: "succeed-to-find"
          upToDate: boolean
          version: VersionString
          target: ConvertLocationResult<ApplicationTargetPath>
      }>
