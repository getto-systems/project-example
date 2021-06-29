import { VersionString } from "../data"
import { ApplicationTargetPath, CheckDeployExistsError } from "./data"
import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"

export type FindNextVersionEvent =
    | Readonly<{ type: "take-longtime-to-find" }>
    | Readonly<{ type: "failed-to-find"; err: CheckDeployExistsError }>
    | Readonly<{
          type: "succeed-to-find"
          upToDate: boolean
          version: VersionString
          target: ConvertLocationResult<ApplicationTargetPath>
      }>
