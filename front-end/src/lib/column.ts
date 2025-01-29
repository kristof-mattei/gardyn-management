import type { Slot } from "@/lib/slot";

export interface Column {
    x: number;
    slots: Array<null | Slot>;
}
