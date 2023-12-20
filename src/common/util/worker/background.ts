export interface WorkerBackgroundHandler<M, R> {
    (message: M): Promise<R>
}
