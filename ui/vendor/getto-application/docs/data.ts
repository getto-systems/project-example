export type DocsDomain = Readonly<{
    path: string
    title: string
    usecase: readonly DocsUsecase[]
}>

export type DocsUsecase = Readonly<{
    path: string
    title: string
    purpose: readonly string[]
    action: readonly DocsAction[]
}>

export type DocsAction = Readonly<{
    title: string
    action: readonly DocsActionContent[]
    data: readonly DocsData[]
}>
export type DocsActionContent =
    | Readonly<{ type: "input"; content: readonly string[]; help?: readonly string[] }>
    | Readonly<{ type: "check"; check: readonly string[]; help?: readonly string[] }>
    | Readonly<{ type: "success"; action: readonly string[]; help?: readonly string[] }>
    | Readonly<{ type: "error"; err: readonly string[]; help?: readonly string[] }>

export type DocsData = Readonly<{
    title: string
    data: readonly DocsDataDescription[]
}>
export type DocsDataDescription = Readonly<{ data: string; help?: readonly string[] }>

// TODO 以下削除予定

export type DocsSection = Readonly<{
    type: "normal" | "pending" | "double"
    title: string
    body: DocsContent[]
}>
export type DocsContent =
    | Readonly<{ type: "purpose"; content: string[] }>
    | Readonly<{ type: "module"; content: string[] }>
    | Readonly<{ type: "item"; title: string; content: string[] }>
    | Readonly<{ type: "description"; content: DocsDescription[] }>
    | Readonly<{ type: "explanation"; target: DocsActionTargetType[] }>
    | Readonly<{ type: "negativeNote"; content: DocsNegativeNote[] }>
    | Readonly<{ type: "action"; content: DocsAction_legacy[] }>
    | Readonly<{ type: "note"; content: string[] }>

export type DocsDescription = Readonly<{ title: string; body: string[]; help: string[] }>
export type DocsNegativeNote = Readonly<{ message: string; help: string }>

export type DocsAction_legacy =
    | Readonly<{ type: "request"; content: DocsAction_request }>
    | Readonly<{ type: "action"; content: DocsAction_action }>

export type DocsAction_request = Readonly<{
    from: DocsActionTargetType
    to: DocsActionTargetType
    body: DocsActionContent_legacy[]
    help: string[]
}>
export type DocsAction_action = Readonly<{
    on: DocsActionTargetType
    body: DocsActionContent_legacy[]
    help: string[]
}>

export enum DocsActionTarget {
    "content-server",
    "api-server",
    "http-client",
    "text-client",
}
export type DocsActionTargetType = keyof typeof DocsActionTarget
export type DocsActionContent_legacy = Readonly<{ type: "normal" | "validate"; message: string }>
