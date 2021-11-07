export type ObserveBoardStore = Readonly<{
    stack: ObserveBoardStack
}>

export interface ObserveBoardStack {
    get(name: string): ObserveBoardStateFound
    set(name: string, hasChanged: boolean): void
}

export interface ObserveBoardChecker<N extends string> {
    update(name: N, hasChanged: boolean): void
}

export type ObserveBoardStateFound =
    | Readonly<{ found: true; hasChanged: boolean }>
    | Readonly<{ found: false }>
