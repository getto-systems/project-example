export interface BoardFieldObserver {
    pin(): void
    hasChanged(): boolean
}
