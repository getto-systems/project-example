export type DocsDescription = Readonly<{
    title: string
    descriptions: readonly Readonly<{
        title: string
        description: readonly string[]
    }>[]
    links?: readonly Readonly<{
        title: string
        path: string
    }>[]
}>

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
