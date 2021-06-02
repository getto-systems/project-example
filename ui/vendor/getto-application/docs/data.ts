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
    | Readonly<{ type: "action"; content: DocsAction[] }>
    | Readonly<{ type: "note"; content: string[] }>

export type DocsDescription = Readonly<{ title: string; body: string[]; help: string[] }>
export type DocsNegativeNote = Readonly<{ message: string; help: string }>

export type DocsAction =
    | Readonly<{ type: "request"; content: DocsAction_request }>
    | Readonly<{ type: "action"; content: DocsAction_action }>

export type DocsAction_request = Readonly<{
    from: DocsActionTargetType
    to: DocsActionTargetType
    body: DocsActionContent[]
    help: string[]
}>
export type DocsAction_action = Readonly<{
    on: DocsActionTargetType
    body: DocsActionContent[]
    help: string[]
}>

export enum DocsActionTarget {
    "content-server",
    "api-server",
    "http-client",
    "text-client",
}
export type DocsActionTargetType = keyof typeof DocsActionTarget
export type DocsActionContent = Readonly<{ type: "normal" | "validate"; message: string }>
