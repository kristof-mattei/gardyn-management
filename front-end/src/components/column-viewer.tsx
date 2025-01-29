import React, { Fragment } from "react";

import type { Column } from "@/lib/column";
import type { Gardyn } from "@/lib/gardyn";
import type { Slot } from "@/lib/slot";

interface ColumnViewerProperties {
    gardyn: Gardyn;
    column: Column;
}

const colors = ["bg-red-100", "bg-green-100", "bg-orange-100"];

const SlotViewer: React.FC<{ slot: null | Slot; index: number }> = ({ slot, index }) => {
    return (
        <Fragment>
            {index % 2 === 0 && <div></div>}
            <div className="border-2 border-red-500">
                {slot === null ? (
                    <div className="place-items-center">
                        <div>-</div>
                    </div>
                ) : (
                    <div className={index % 2 === 0 ? "place-items-start" : "place-items-end"}>
                        <div>{slot.name}</div>
                    </div>
                )}
            </div>
            {index % 2 !== 0 && <div></div>}
        </Fragment>
    );
};

export const ColumnViewer: React.FC<ColumnViewerProperties> = ({ column }) => {
    return (
        <div className={`col-span-2 grid grid-cols-subgrid gap-4 ${colors[column.x]}`}>
            {column.slots.toReversed().map((slot, index) => {
                return <SlotViewer key={`column-${column.x}-row-${index}`} slot={slot} index={index} />;
            })}
        </div>
    );
};
