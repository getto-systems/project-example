import { VNode } from "preact"

export type VNodeContent = VNodeEntry | VNodeEntry[]
type VNodeEntry = string | number | VNode

export type VNodeKey = string | number
