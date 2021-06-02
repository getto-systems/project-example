export interface ApplicationActionStatePublisher<S> {
    post(state: S): S
    terminate(): void
}

export interface ApplicationActionIgniteRunner<S> {
    ignite(): Promise<S>
}
export interface ApplicationActionIgniteHook<S> {
    (): Promise<S>
}

export interface ApplicationActionTerminateRunner {
    register(hook: ApplicationActionTerminateHook): void
    terminate(): void
}
export interface ApplicationActionTerminateHook {
    (): void
}
