import type React from "react";

import { useEffect, useState } from "react";

import { ColumnViewer } from "@/components/column-viewer";
import type { Column } from "@/lib/column";
import type { Gardyn } from "@/lib/gardyn";

interface GardynViewerProperties {
    gardyn: Gardyn;
}

export const GardynViewer: React.FC<GardynViewerProperties> = ({ gardyn }) => {
    const [columns, setColumns] = useState<Column[] | null>();

    useEffect(() => {
        setColumns([
            {
                x: 0,
                slots: [
                    { name: "Plant 1" },
                    { name: "Plant 2" },
                    { name: "Plant 3" },
                    { name: "Plant 4" },
                    null,
                    null,
                    { name: "Plant 7" },
                    { name: "Plant 8" },
                    null,
                    null,
                    { name: "Plant 11" },
                    { name: "Plant 12" },
                ],
            },
            {
                x: 1,
                slots: [
                    { name: "Plant 1" },
                    { name: "Plant 2" },
                    { name: "Plant 3" },
                    { name: "Plant 4" },
                    null,
                    null,
                    { name: "Plant 7" },
                    { name: "Plant 8" },
                    null,
                    null,
                    { name: "Plant 11" },
                    { name: "Plant 12" },
                ],
            },
            {
                x: 2,
                slots: [
                    { name: "Plant 1" },
                    { name: "Plant 2" },
                    { name: "Plant 3" },
                    { name: "Plant 4" },
                    null,
                    null,
                    { name: "Plant 7" },
                    { name: "Plant 8" },
                    null,
                    null,
                    { name: "Plant 11" },
                    { name: "Plant 12" },
                ],
            },
        ]);
    }, []);

    return (
        <div className="h-screen p-8">
            {columns?.length !== undefined && (
                <div className={`grid grid-cols-${columns.length * 2} border-2`}>
                    {columns.map((column) => (
                        <ColumnViewer gardyn={gardyn} column={column} key={`column-${column.x}`} />
                    ))}
                </div>
            )}
        </div>
    );
};
