export type ApplicationTargetPath = string & { ApplicationTargetPath: never }

export type Version = Version_data & { Version: never }
type Version_data = Readonly<{ major: number; minor: number; patch: number; suffix: string }>

export type ParseVersionResult =
    | Readonly<{ valid: true; value: Version }>
    | Readonly<{ valid: false }>

export type CheckDeployExistsError = CheckDeployExistsRemoteError
export type CheckDeployExistsRemoteError =
    | Readonly<{ type: "server-error" }>
    | Readonly<{ type: "infra-error"; err: string }>
