export type StatefulApplicationAction<S> = Readonly<{
    state: ApplicationState<S>
}>

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
    const action = new State(props)
    return {
        state: action,
        post: (state) => action.post(state),
    }
}

class State<S> implements ApplicationState<S> {
    handlers: ApplicationStateHandler<S>[] = []

    readonly ignitionState: Promise<S>
    state: S

    constructor(props: ApplicationStateProps<S>) {
        this.ignitionState = ignite()
        this.state = props.initialState

        function ignite(): Promise<S> {
            return new Promise((resolve) => {
                // コンストラクタが重くならないように初期 action は setTimeout で呼び出す
                // 状態は currentState() で最新のものを参照するので subscribe を待つ必要はない
                setTimeout(async () => {
                    resolve(props.ignite ? await props.ignite() : props.initialState)
                }, 0)
            })
        }
    }

    subscribe(handler: ApplicationStateHandler<S>): void {
        this.handlers = [...this.handlers, handler]
    }
    unsubscribe(target: ApplicationStateHandler<S>): void {
        this.handlers = this.handlers.filter((handler) => handler !== target)
    }
    post(state: S): S {
        this.state = state
        this.handlers.forEach((post) => post(state))
        return state
    }

    currentState(): S {
        return this.state
    }
}

interface Post<S> {
    (state: S): S
}
