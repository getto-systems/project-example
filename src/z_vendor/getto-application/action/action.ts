export type StatefulApplicationAction<S> = Readonly<{
    state: ApplicationStateAction<S>
}>

export interface ApplicationStateAction<S> {
    readonly ignitionState: Promise<S>
    subscribe(handler: ApplicationStateHandler<S>): void
    unsubscribe(target: ApplicationStateHandler<S>): void
    currentState(): S
}
export interface ApplicationStateHandler<S> {
    (state: S): void
}

export type ApplicationStateActionProps<S> = Readonly<{
    initialState: S
    ignite?: () => Promise<S>
}>
export function initApplicationStateAction<S>(props: ApplicationStateActionProps<S>): Readonly<{
    state: ApplicationStateAction<S>
    post: Post<S>
}> {
    const action = new StateAction(props)
    return {
        state: action,
        post: (state) => action.post(state),
    }
}

class StateAction<S> implements ApplicationStateAction<S> {
    handlers: ApplicationStateHandler<S>[] = []

    readonly ignitionState: Promise<S>
    state: S

    constructor(props: ApplicationStateActionProps<S>) {
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
