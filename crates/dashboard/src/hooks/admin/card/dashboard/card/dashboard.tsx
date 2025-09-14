import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback } from "react";

export default function AnalyticsDashboardCard(toast: any) {
    const {
        dashboardCard,
        dashboard,
        loadingDashboard,
        setLoadingDashboard,
        setErrorDashboard,
    } = useCardStore();

    const fetchDashboardCard = useCallback(async () => {
        try {
            setLoadingDashboard(true);
            setErrorDashboard(null);

            await dashboardCard(toast);
        } catch (error) {
            console.error("Failed to fetch dashboard card:", error);
            setErrorDashboard("Failed to fetch dashboard card");
        } finally {
            setLoadingDashboard(false);
        }
    }, [dashboard]);

    const debouncedFetchDashboardCard = debounce(fetchDashboardCard, 1000);

    return {
        dashboardCard,
        dashboard,
        loadingDashboard,
        setLoadingDashboard,
        setErrorDashboard,
        fetchDashboardCard,
        debouncedFetchDashboardCard
    }
}