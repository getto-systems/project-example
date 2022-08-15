import { BoardFieldObserver } from "../infra"

export function initBoardFieldObserver<V>(
    props: Readonly<{
        current: { (): V }
        isSame?: { (a: V, b: V): boolean }
    }>,
): BoardFieldObserver {
    const current = props.current
    const isSame = props.isSame ? props.isSame : (a: V, b: V) => a === b

    let value = props.current()

    return {
        pin(): void {
            value = current()
        },
        hasChanged(): boolean {
            return !isSame(value, current())
        },
    }
}
