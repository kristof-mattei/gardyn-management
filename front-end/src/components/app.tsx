import { createAsyncStoragePersister } from "@tanstack/query-async-storage-persister";
import type { UseQueryResult } from "@tanstack/react-query";
import { QueryClient, useQuery } from "@tanstack/react-query";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import type { Persister } from "@tanstack/react-query-persist-client";
import { PersistQueryClientProvider } from "@tanstack/react-query-persist-client";
import axios from "axios";

import type React from "react";
import { useEffect, useState } from "react";

import { ErrorBoundary } from "@/components/error-boundary.tsx";
import { GardynViewer } from "@/components/gardyn-viewer.tsx";
import type { Gardyn } from "@/lib/gardyn.ts";

const appQueryClient = new QueryClient({
    defaultOptions: {
        queries: {
            gcTime: 1000 * 60 * 60 * 24, // 24 hours
        },
    },
});

const persister: Persister = createAsyncStoragePersister({
    storage: globalThis.localStorage,
});

function useGardyns(): UseQueryResult<Gardyn[]> {
    return useQuery({
        queryKey: ["gardyns"],
        queryFn: async (): Promise<Gardyn[]> => {
            const response = await axios.get<Gardyn[]>("/api/gardyns");

            return response.data;
        },
    });
}
export const App: React.FC = () => {
    const [selectedGardyn, setSelectedGardyn] = useState<Gardyn | null>(null);

    const { data } = useGardyns();

    useEffect(() => {
        setSelectedGardyn(data?.[0] ?? null);
    }, [data]); // Empty dependency array ensures the effect runs once on mount

    const selectedGardynChanged: React.ChangeEventHandler<HTMLSelectElement> = (event): void => {
        const newSelection = Number.parseInt(event.target.value);

        const newGardyn = data?.find((g) => {
            return g.id === newSelection;
        });

        if (newGardyn !== undefined) {
            setSelectedGardyn(newGardyn);
        }
    };

    return (
        <PersistQueryClientProvider client={appQueryClient} persistOptions={{ persister }}>
            <div>
                Gardyn:{" "}
                <select onChange={selectedGardynChanged} value={selectedGardyn?.id}>
                    {data?.map((gardyn) => {
                        return (
                            <option key={gardyn.id} value={gardyn.id} selected={selectedGardyn?.id === gardyn.id}>
                                {gardyn.name}
                            </option>
                        );
                    })}
                </select>
                <ErrorBoundary>{selectedGardyn !== null && <GardynViewer gardyn={selectedGardyn} />}</ErrorBoundary>
                <ReactQueryDevtools initialIsOpen />
            </div>
        </PersistQueryClientProvider>
    );
};
