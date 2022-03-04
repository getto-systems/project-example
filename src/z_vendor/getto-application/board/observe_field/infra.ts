export interface BoardFieldObserver<V> {
    pin(): void
    peek(): V
    hasChanged(): boolean
}
