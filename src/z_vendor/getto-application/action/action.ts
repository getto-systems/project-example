export interface ApplicationState<S> {
    readonly ignitionState: Promise<S>
    subscribe(handler: ApplicationStateHandler<S>): void
    unsubscribe(target: ApplicationStateHandler<S>): void
    currentState(): S
}
export interface ApplicationStateHandler<S> {
    (state: S): void
}

export type ApplicationStateProps<S> = Readonly<{
    initialState: S
    ignite?: () => Promise<S>
}>
export function initApplicationState<S>(props: ApplicationStateProps<S>): Readonly<{
    state: ApplicationState<S>
    post: Post<S>
}> {
    let state = props.initialState
    let handlers: ApplicationStateHandler<S>[] = []

    return {
        state: {
            ignitionState: new Promise((resolve) => {
                // コンストラクタが重くならないように初期 action は setTimeout で呼び出す
                // 状態は currentState() で最新のものを参照するので subscribe を待つ必要はない
                setTimeout(async () => {
                    resolve(props.ignite ? await props.ignite() : props.initialState)
                }, 0)
            }),

            subscribe(handler: ApplicationStateHandler<S>): void {
                handlers = [...handlers, handler]
            },
            unsubscribe(target: ApplicationStateHandler<S>): void {
                handlers = handlers.filter((handler) => handler !== target)
            },

            currentState(): S {
                return state
            },
        },
        post,
    }

    function post(newState: S): S {
        state = newState
        handlers.forEach((handler) => handler(newState))
        return newState
    }
}

interface Post<S> {
    (state: S): S
}
