export type ValidateBoardStore = Readonly<{
    stack: ValidateBoardStack
}>

export interface ValidateBoardStack {
    get(name: string): ValidateBoardStateFound
    set(name: string, result: boolean): void
}

export type ValidateBoardStateFound =
    | Readonly<{ found: true; state: boolean }>
    | Readonly<{ found: false }>
