import { calculatePercentageChange } from "@/helpers/calculate";
import { debounce } from "@/helpers/debounce";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useCallback } from "react";

export default function useYearStatusFailedWithdraw(currentYear: number, toast: any) {
    const {
        findYearStatusFailed,
        yearStatusFailed,
        loadingYearStatusFailed,
        setLoadingYearStatusFailed,
        setErrorYearStatusFailed,
    } = useWithdrawStore();

    const currentYearFailedData = yearStatusFailed?.find(
        (balance) => balance.year === currentYear.toString(),
    );

    const previousYearFailedData = yearStatusFailed?.find(
        (balance) => balance.year === (currentYear - 1).toString(),
    );

    const currentYearFailed = currentYearFailedData?.total_amount || 0;
    const previousYearFailed = previousYearFailedData?.total_amount || 0;

    const yearPercentageFailedChange = calculatePercentageChange(
        currentYearFailed,
        previousYearFailed,
    );

    const fetchYearlyFailed = useCallback(
        async (year: number) => {
            try {
                setLoadingYearStatusFailed(true);
                setErrorYearStatusFailed(null);

                await findYearStatusFailed(toast, year);
            } catch (error) {
                setErrorYearStatusFailed("Failed to fetch yearly balance");
            } finally {
                setLoadingYearStatusFailed(false);
            }
        },
        [
            findYearStatusFailed,
            setLoadingYearStatusFailed,
            setLoadingYearStatusFailed,
        ],
    );

    const debouncedFetchYearlyFailed = debounce(fetchYearlyFailed, 300);

    return {
        debouncedFetchYearlyFailed,
        loadingYearStatusFailed,
        currentYearFailedData,
        previousYearFailedData,
        currentYearFailed,
        previousYearFailed,
        yearPercentageFailedChange,
        findYearStatusFailed
    }
}