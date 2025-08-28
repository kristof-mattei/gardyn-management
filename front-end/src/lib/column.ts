import type { Slot } from "@/lib/slot";

export interface Column {
    x: number;
    slots: (null | Slot)[];
}
