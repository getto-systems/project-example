export type ObserveBoardFieldInfra = Readonly<{
    observer: BoardFieldObserver
}>

export interface BoardFieldObserver {
    pin(): void
    hasChanged(): boolean
}
