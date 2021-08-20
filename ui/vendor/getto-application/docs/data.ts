export type DocsDomain<U, A, D> = Readonly<{
    title: string
    purpose: string[]
    usecase: U[]
    toUsecase: DocsUsecaseMap<U, A, D>
}>
export type DocsDomainContent = Readonly<{
    title: string
    purpose: string[]
    usecase: DocsUsecaseContent[]
    data: DocsData[]
}>
export interface DocsUsecaseMap<U, A, D> {
    (usecase: U): DocsUsecase<A, D>
}

export type DocsUsecase<A, D> = Readonly<{
    title: A
    purpose: string[]
}> &
    DocsUsecaseDescription<A, D> &
    DocsUsecaseDescriptionMap<A, D>
export type DocsUsecaseContent = Readonly<{
    title: string
    purpose: string[]
    action: DocsAction[]
    data: DocsData[]
}>
export type DocsUsecaseDescription<A, D> = Readonly<{
    action: A[]
    data: D[]
}>
export type DocsUsecaseDescriptionMap<A, D> = Readonly<{
    toAction: DocsActionMap<A>
    toData: DocsDataMap<D>
}>
export interface DocsActionMap<A> {
    (action: A): DocsAction
}
export interface DocsDataMap<D> {
    (data: D): DocsData
}

export type DocsAction = Readonly<{ title: string; item: DocsActionItem[] }>
export type DocsActionItem = Readonly<{
    type: DocsActionItemType
    content: string[]
    help: string[]
}>
export type DocsActionItemType = "input" | "check" | "success" | "error"

export type DocsData = Readonly<{
    title: string
    data: DocsDataDescription[]
}>
export type DocsDataDescription = Readonly<{ description: string; help: string[] }>

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
