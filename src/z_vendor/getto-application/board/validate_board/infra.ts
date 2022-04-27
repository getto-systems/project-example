import { ConvertBoardResult } from "../kernel/data"

export interface ValidateBoardStack {
    get(name: string): ValidateBoardStateFound
    set(name: string, result: boolean): void
    delete(name: string): void
}

// ValidateBoardFieldState のサブセット
export type ValidateBoardCheckState =
    | Readonly<{ type: "initial" }>
    | Readonly<{ type: "validated"; result: Readonly<{ valid: boolean }> }>
export interface ValidateBoardChecker<N extends string, T> {
    update(name: N, state: ValidateBoardCheckState): void
    get(): ConvertBoardResult<T>
}

export type ValidateBoardStateFound =
    | Readonly<{ found: true; state: boolean }>
    | Readonly<{ found: false }>
