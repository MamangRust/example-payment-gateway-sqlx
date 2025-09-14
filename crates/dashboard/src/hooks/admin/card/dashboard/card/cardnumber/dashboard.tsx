import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback } from "react";

export default function AnalyticsDashboardCard(card_number: string,toast: any) {
    const {
        dashboardCardNumber,
        dashboardCardCardNumber,
        loadingDashboardCardNumber,
        setLoadingDashboardCardNumber,
        setErrorDashboardCardNumber,
    } = useCardStore();

    const fetchDashboardCard = useCallback(async () => {
        try {
            setLoadingDashboardCardNumber(true);
            setErrorDashboardCardNumber(null);

            await dashboardCardCardNumber(toast, card_number as string);
        } catch (error) {
            console.error("Failed to fetch dashboard card:", error);
            setErrorDashboardCardNumber("Failed to fetch dashboard card");
        } finally {
            setLoadingDashboardCardNumber(false);
        }
    }, [dashboardCardNumber]);

    const debouncedFetchDashboardCard = debounce(fetchDashboardCard, 1000);

    return {
        dashboardCardNumber,
        dashboardCardCardNumber,
        loadingDashboardCardNumber,
        setLoadingDashboardCardNumber,
        setErrorDashboardCardNumber,
        fetchDashboardCard,
        debouncedFetchDashboardCard
    }
}