// TODO deprecated
export interface WorkerHandler<M> {
    (message: M): void
}

export interface WorkerBackgroundHandler<M, R> {
    (message: M): Promise<R>
}
