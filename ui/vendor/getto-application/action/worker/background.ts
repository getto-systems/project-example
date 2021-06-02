export interface WorkerHandler<M> {
    (message: M): void
}
