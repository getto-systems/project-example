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
