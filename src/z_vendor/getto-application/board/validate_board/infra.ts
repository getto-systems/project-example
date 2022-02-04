import { ConvertBoardResult } from "../kernel/data"

export interface ValidateBoardStack {
    get(name: string): ValidateBoardStateFound
    set(name: string, result: boolean): void
}

export interface ValidateBoardChecker<N extends string, T> {
    update(name: N, result: boolean): void
    get(): ConvertBoardResult<T>
}

export type ValidateBoardStateFound =
    | Readonly<{ found: true; state: boolean }>
    | Readonly<{ found: false }>
