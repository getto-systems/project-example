export type LocationPathname = string & { LocationPathname: never }
export type ScriptPath = string & { ScriptPath: never }

export type ConvertScriptPathResult =
    | Readonly<{ valid: true; value: ScriptPath }>
    | Readonly<{ valid: false }>

export type LoadScriptError = Readonly<{ type: "infra-error"; err: string }>
