export type ApplicationView<R> = Readonly<{
    resource: R
    terminate: { (): void }
}>

export interface ApplicationAction {
    terminate(): void
}
export interface ApplicationStateAction<S> extends ApplicationAction {
    readonly subscriber: ApplicationStateSubscriber<S>
    currentState(): S
    ignite(): Promise<S>
}

export interface ApplicationStateSubscriber<S> {
    subscribe(handler: ApplicationStateHandler<S>): void
    unsubscribe(target: ApplicationStateHandler<S>): void
}
export interface ApplicationStateHandler<S> {
    (state: S): void
}
