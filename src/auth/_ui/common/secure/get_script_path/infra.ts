export type GetScriptPathInfra = Readonly<{
    config: Readonly<{
        secureServerURL: SecureServerURL
    }>
}>

export type SecureServerURL = string & { SecureServerURL: never }
